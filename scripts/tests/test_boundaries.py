# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import json
import re
from collections import Counter
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from slint_contract import ContractError, images, lex, parse, public_api
from check_ui_boundaries import baseline_findings, boundaries, check, check_product, provenance
from check_cargo_boundaries import KIT_URL, check_manifest, check_metadata, reachable
from verify_distribution import verify, verify_archive

HEADER = '// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors\n// SPDX-License-Identifier: GPL-3.0-only\n'


class ScannerTests(unittest.TestCase):
    def shape(self, source):
        return parse(source)['definitions']

    def test_multiline_alias_comments_and_literal_delimiters(self):
        parsed = parse('''/* { /* nested */ } */ import {
            Theme as T,
        } from "../foundation/theme.slint";
        export { T as PublicTheme };
        export component A inherits FocusScope {
          in property <string> value: "{ // /* } ; \\\" "; // }
          callback done(string, int);
        }''')
        self.assertEqual(parsed['imports'][0][1], [('Theme', 'T')])
        self.assertEqual(parsed['exports']['PublicTheme'], (None, 'T'))
        self.assertEqual(parsed['definitions']['A']['signature']['inherits'], 'FocusScope')
        self.assertIn('//', parsed['definitions']['A']['defaults']['value']['expression'])

    def test_formatting_and_identifier_spelling_equivalence(self):
        a = 'export component A { in-out property <string> some_value <=> child.text; callback done(string,int); }'
        b = 'export component A {\n in-out /*comment*/ property<string> some-value <=> child . text ;\n callback done ( string , int ) ;\n}'
        self.assertEqual(self.shape(a), self.shape(b))

    def test_string_whitespace_is_semantic_and_empty_string_is_not_eof(self):
        a = self.shape('export global A { out property <string> x: "a  b"; out property <string> y: ""; }')
        b = self.shape('export global A { out property <string> x: "a b"; out property <string> y: ""; }')
        self.assertNotEqual(a['A']['defaults'], b['A']['defaults'])
        self.assertIn('y', a['A']['signature']['members'])

    def test_unicode_escape_equivalence_and_invalid_scalar(self):
        self.assertEqual(self.shape(r'export global A { out property <string> x: "\u{41}"; }'),
                         self.shape('export global A { out property <string> x: "A"; }'))
        for escape in (r'\u{d800}', r'\u{110000}', r'\u{oops}', r'\r'):
            with self.subTest(escape=escape), self.assertRaises(ContractError):
                self.shape('export global A { out property <string> x: "' + escape + '"; }')

    def test_function_callback_binding_and_struct(self):
        definitions = self.shape('''export struct Record { name: string, values: [int], }
        export enum Kind { first, second, }
        export component A inherits Rectangle {
          pure callback compute(int) -> int;
          callback notify <=> child.notify;
          public pure function convert(value: Record, factor: float) -> string { return "}"; }
          private property <bool> internal: false;
        }''')
        api = definitions['A']['signature']['members']
        self.assertTrue(api['convert']['pure'])
        self.assertEqual(api['compute']['arguments'], ['int'])
        self.assertEqual(api['convert']['return'], 'string')
        self.assertEqual(len(definitions['Record']['signature']['fields']), 2)
        self.assertNotIn('internal', api)

    def test_api_mutations_and_defaults_reported_separately(self):
        original = '''export struct Record { name: string, count: int }
        export component A { in property <string> value: "old"; callback done(int, string); public pure function f(x: int) -> bool { return true; } }'''
        baseline = {'exports': self.shape(original)}
        for before, after in [('count: int', 'count: string'), (', count: int', ''),
                              ('in property', 'out property'), ('int, string', 'int'),
                              ('public pure', 'public'), ('-> bool', '-> int'),
                              ('x: int', 'x: string')]:
            with self.subTest(after=after):
                findings = baseline_findings(self.shape(original.replace(before, after)), baseline)
                self.assertTrue(any('API signature changed' in f for f in findings))
        changed = baseline_findings(self.shape(original.replace('"old"', '"new"')), baseline)
        self.assertEqual(len(changed), 1)
        self.assertIn('API defaults changed', changed[0])

    def test_unknown_public_and_malformed_syntax_fail_closed(self):
        for source in ['export trait A {}', 'export component A { public mystery x; }',
                       'export component A { in property <string> x strange; }',
                       'export component A { experimental public function f() {} }',
                       'export component A { callback f(string) extra; }',
                       'export struct A { x }', 'export enum A { x = 3 }',
                       'export component A { in property <string> x;',
                       '/* unfinished', 'export component A { in property <string> x: "oops; }']:
            with self.subTest(source=source), self.assertRaises(ContractError):
                parse(source)

    def test_duplicate_definitions_members_and_exports(self):
        for source in ['export component A {} export component A {}',
                       'export { A, A } from "a.slint";',
                       'export component A { callback f; callback f; }',
                       'export struct A { x: int, x: string }']:
            with self.subTest(source=source), self.assertRaises(ContractError):
                parse(source)

    def test_images_ignore_comments_and_string_lookalikes(self):
        self.assertEqual(images('''// @image-url("fake.svg")
        export global A { out property <string> text: "@image-url(\\\"fake.svg\\\")";
        out property <image> icon: @image-url( /* } */ "real.svg" ); }'''), ['real.svg'])
        with self.assertRaises(ContractError):
            images('@image-url(root.path)')


class RepositoryFixtures(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.write('ui/kit.slint', 'export { A } from "primitives/a.slint";')
        self.write('ui/primitives/a.slint', 'import { Theme } from "../foundation/theme.slint"; export component A inherits FocusScope { in property <string> task: "general task"; }')
        self.write('ui/foundation/theme.slint', 'export global Theme { out property <color> focus_ring: #fff; }')
        self.write('gallery/ui/gallery.slint', 'import { A } from "@quadrant-kit"; export component Gallery inherits Window {}')

    def write(self, path, content):
        path = self.root / path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(HEADER + content, encoding='utf-8')

    def test_valid_layers_named_library_and_generic_focus_task(self):
        boundaries(self.root)
        check_product(public_api(self.root))

    def test_reexport_alias_chain_and_path_movement(self):
        self.write('ui/kit.slint', 'export { Middle as Public } from "bridge.slint";')
        self.write('ui/bridge.slint', 'import { A as Local } from "primitives/a.slint"; export { Local as Middle };')
        api = public_api(self.root)
        self.assertEqual(set(api), {'Public'})
        source = (self.root / 'ui/primitives/a.slint').read_text(encoding='utf-8')
        self.write('ui/primitives/moved.slint', source)
        self.write('ui/bridge.slint', 'export { A as Middle } from "primitives/moved.slint";')
        self.assertEqual(api, public_api(self.root))

    def test_missing_and_cyclic_reexports(self):
        for content in ['export { Missing } from "primitives/a.slint";', 'export { A } from "kit.slint";']:
            self.write('ui/kit.slint', content)
            with self.assertRaises(ContractError):
                public_api(self.root)

    def test_extra_export_changes_baseline(self):
        baseline = {'exports': public_api(self.root)}
        self.write('ui/kit.slint', 'export { A, A as Extra } from "primitives/a.slint";')
        self.assertTrue(baseline_findings(public_api(self.root), baseline))

    def test_duplicate_exported_definition_and_hidden_product_token(self):
        self.write('ui/primitives/b.slint', 'export component A {}')
        with self.assertRaisesRegex(ContractError, 'Duplicate exported'):
            boundaries(self.root)
        self.write('ui/primitives/b.slint', 'export global Theme { out property <color> q1_accent: #fff; }')
        with self.assertRaisesRegex(ContractError, 'Product member'):
            boundaries(self.root)

    def test_same_layer_acyclic_helper_is_legal(self):
        self.write('ui/primitives/b.slint', 'import { A } from "a.slint"; export component B inherits A {}')
        boundaries(self.root)

    def test_missing_source_header(self):
        path = self.root / 'ui/primitives/a.slint'
        path.write_text('export component A {}', encoding='utf-8')
        with self.assertRaisesRegex(ContractError, 'provenance header'):
            provenance(self.root)

    def test_gallery_bypass(self):
        self.write('gallery/ui/gallery.slint', 'import { A } from "../../ui/primitives/a.slint";')
        with self.assertRaisesRegex(ContractError, 'bypasses'):
            boundaries(self.root)

    def test_upward_and_cross_layer_dependencies(self):
        self.write('ui/patterns/p.slint', 'export component P {}')
        self.write('ui/overlays/o.slint', 'export component O {}')
        for owner, target in [('ui/foundation/theme.slint', '../primitives/a.slint'),
                              ('ui/primitives/a.slint', '../patterns/p.slint'),
                              ('ui/patterns/p.slint', '../overlays/o.slint')]:
            with self.subTest(owner=owner):
                old = (self.root / owner).read_text(encoding='utf-8')
                self.write(owner, f'import {{ P }} from "{target}";')
                with self.assertRaisesRegex(ContractError, 'Upward'):
                    boundaries(self.root)
                (self.root / owner).write_text(old, encoding='utf-8')

    def test_same_layer_cycle_and_implementation_facade(self):
        self.write('ui/primitives/b.slint', 'import { A } from "a.slint"; export component B {}')
        for target in ('b.slint', '../kit.slint', '@quadrant-kit'):
            self.write('ui/primitives/a.slint', f'import {{ B }} from "{target}"; export component A {{}}')
            with self.subTest(target=target), self.assertRaises(ContractError):
                boundaries(self.root)

    def test_static_escape_and_external_paths(self):
        for target in ('../../../outside.svg', '/tmp/a.svg', 'C:/a.svg', 'https://example.org/a.svg'):
            self.write('ui/primitives/a.slint', f'export component A {{ in property <image> icon: @image-url("{target}"); }}')
            with self.subTest(target=target), self.assertRaises(ContractError):
                boundaries(self.root)

    def test_product_names_and_tokens(self):
        for source in ['export component InboxPane {}', 'export global Theme { out property <color> q1_accent: #fff; }']:
            self.write('ui/kit.slint', source)
            with self.subTest(source=source), self.assertRaises(ContractError):
                check_product(public_api(self.root))

    def test_asset_manifest_mit_hash_and_unrecorded_file(self):
        self.write('ui/kit.slint', 'export global Icons { out property <image> generic: @image-url("../assets/icons/a.svg"); }')
        self.write('assets/icons/a.svg', '<svg/>')
        import hashlib
        asset = self.root / 'assets/icons/a.svg'
        manifest = {'assets': [{'new_path': 'assets/icons/a.svg', 'sha256': hashlib.sha256(asset.read_bytes()).hexdigest(), 'spdx_license': 'MIT'}]}
        # Third-party bytes do not get a GPL source header.
        asset.write_bytes(b'<svg/>')
        manifest['assets'][0]['sha256'] = hashlib.sha256(asset.read_bytes()).hexdigest()
        for name in ('LICENSE', 'assets/icons/LICENSE-MIT', 'THIRD-PARTY-NOTICES.md', 'docs/PROVENANCE.md'):
            self.write(name, 'Fixture license/provenance')
        self.write('LICENSE', 'GNU GENERAL PUBLIC LICENSE\nVersion 3, 29 June 2007')
        self.write('assets/icons/LICENSE-MIT', 'MIT License\nMicrosoft Corporation\nPermission is hereby granted')
        self.write('scripts/asset_manifest.json', '')
        (self.root / 'scripts/asset_manifest.json').write_text(json.dumps(manifest), encoding='utf-8')
        self.assertEqual(verify(self.root)['svg_assets'], 1)
        import tarfile
        archive = self.root / 'fixture.crate'
        with tarfile.open(archive, 'w:gz') as package:
            for path in self.root.rglob('*'):
                if path.is_file() and path != archive:
                    package.add(path, arcname='quadrant-kit-0.1.0/' + path.relative_to(self.root).as_posix())
        verify_archive(archive, self.root)
        asset.write_bytes(b'<svg changed="yes"/>')
        with self.assertRaisesRegex(ValueError, 'archived'):
            verify_archive(archive, self.root)
        with self.assertRaisesRegex(ValueError, 'hash'):
            verify(self.root)
        asset.write_bytes(b'<svg/>')
        manifest['assets'][0]['spdx_license'] = 'GPL-3.0-only'
        (self.root / 'scripts/asset_manifest.json').write_text(json.dumps(manifest), encoding='utf-8')
        with self.assertRaisesRegex(ValueError, 'license'):
            verify(self.root)
        manifest['assets'][0]['spdx_license'] = 'MIT'
        (self.root / 'scripts/asset_manifest.json').write_text(json.dumps(manifest), encoding='utf-8')
        self.write('assets/icons/extra.svg', '<svg/>')
        with self.assertRaisesRegex(ValueError, 'Unrecorded'):
            verify(self.root)


class CargoFixtures(unittest.TestCase):
    def test_legal_gallery_path_and_tasks_internal_path(self):
        check_manifest({'package': {'name': 'quadrant-kit-gallery'}, 'build-dependencies': {'kit': {'package': 'quadrant-kit', 'path': '..'}}}, {})
        check_manifest({'dependencies': {'domain': {'package': 'quadrant-domain', 'path': '../domain'}}}, {}, 'tasks')

    def test_tasks_alias_workspace_target_and_dev_cannot_hide_path(self):
        kit = {'package': 'quadrant-kit', 'path': '../../Quadrant-Kit'}
        for manifest, workspace in [({'dependencies': {'alias': kit}}, {}),
                                    ({'dev-dependencies': {'alias': {'workspace': True}}}, {'dependencies': {'alias': kit}}),
                                    ({'target': {'cfg(windows)': {'build-dependencies': {'alias': kit}}}}, {})]:
            with self.subTest(manifest=manifest), self.assertRaises(ContractError):
                check_manifest(manifest, workspace, 'tasks')

    def test_git_pin_and_overrides(self):
        valid = {'git': KIT_URL, 'rev': 'a' * 40}
        check_manifest({'build-dependencies': {'quadrant-kit': valid}}, {}, 'tasks')
        for changed in ({**valid, 'rev': 'main'}, {**valid, 'git': 'file:///kit'}, {**valid, 'path': '../kit'}, {**valid, 'branch': 'main'}):
            with self.assertRaises(ContractError):
                check_manifest({'dependencies': {'quadrant-kit': changed}}, {}, 'tasks')
        for key in ('patch', 'replace'):
            with self.assertRaises(ContractError):
                check_manifest({key: {'some-source': {}}}, {})

    def metadata(self):
        return {'packages': [{'id': 'kit', 'name': 'quadrant-kit', 'source': None}, {'id': 'slint', 'name': 'slint', 'version': '1.17.1'}, {'id': 'gallery', 'name': 'quadrant-kit-gallery'}],
                'workspace_members': ['kit', 'gallery'], 'resolve': {'nodes': [
                    {'id': 'kit', 'deps': []}, {'id': 'slint', 'deps': []},
                    {'id': 'gallery', 'deps': [{'pkg': 'slint', 'dep_kinds': [{'kind': None}]}, {'pkg': 'kit', 'dep_kinds': [{'kind': 'build'}]}]}]}}

    def test_actual_local_source_and_mismatched_resolved_rev_rejected(self):
        data = self.metadata()
        with self.assertRaises(ContractError):
            check_metadata(data, 'tasks')
        data['packages'][0]['source'] = 'git+' + KIT_URL + '?rev=' + 'a' * 40 + '#' + 'a' * 40
        check_metadata(data, 'tasks')
        data['packages'][0]['source'] = data['packages'][0]['source'][:-1] + 'b'
        with self.assertRaises(ContractError):
            check_metadata(data, 'tasks')

    def test_graph_kinds_and_reachability_not_workspace_presence(self):
        data = self.metadata()
        check_metadata(data)
        self.assertEqual(reachable(data, 'kit'), set())
        self.assertEqual(reachable(data, 'gallery'), {'slint'})
        self.assertEqual(reachable(data, 'gallery', ('build',)), {'kit'})
        data['resolve']['nodes'][0]['deps'] = [{'pkg': 'slint', 'dep_kinds': [{'kind': None}]}]
        with self.assertRaisesRegex(ContractError, 'runtime'):
            check_metadata(data)

    def test_product_dependency_alias_rejected(self):
        with self.assertRaises(ContractError):
            check_manifest({'dependencies': {'innocent': {'package': 'quadrant-storage', 'version': '1'}}}, {})


class CurrentRepositoryTests(unittest.TestCase):
    def test_documented_contract_and_probe_cover_current_api(self):
        root = Path(__file__).resolve().parents[2]
        api = public_api(root)
        documented = {}
        text = (root / 'docs/PUBLIC_API.md').read_text(encoding='utf-8')
        for source in re.findall(r'```slint\s*\n(.*?)```', text, re.S):
            definitions = parse(source)['definitions']
            self.assertFalse(documented.keys() & definitions.keys(), 'Duplicate documented declaration')
            documented.update(definitions)
        self.assertEqual(documented, api, 'Documented signatures/defaults must match the facade')
        probe = (root / 'gallery/ui/api_probe.slint').read_text(encoding='utf-8')
        imports = [pair for target, pairs in parse(probe)['imports'] if target == '@quadrant-kit' for pair in pairs]
        self.assertEqual({name for name, _ in imports}, set(api))
        tokens = Counter(token.value for token in lex(probe))
        for _, alias in imports:
            self.assertGreater(tokens[alias], 1, f'{alias} is imported but unused by the probe')
        members = [member for item in api.values() for member in item['signature'].get('members', {}).values()]
        self.assertEqual(Counter(member['kind'] for member in members), {'property': 240, 'callback': 20})
        self.assertEqual(Counter(item['signature']['kind'] for item in api.values()),
                         {'component': 21, 'global': 6, 'enum': 7, 'struct': 1})
        self.assertEqual(len(api['NavigationEntry']['signature']['fields']), 10)

    def test_live_legacy_navigation_is_absent(self):
        root = Path(__file__).resolve().parents[2]
        forbidden = {'SidebarItem', 'sidebar-bg', 'sidebar-collapsed-width',
                     'sidebar-expanded-width', 'catalog-filter', 'CatalogFilter'}
        for folder in ('ui', 'gallery/ui'):
            for path in (root / folder).rglob('*.slint'):
                tokens = lex(path.read_text(encoding='utf-8'))
                self.assertFalse(forbidden & {token.value for token in tokens}, str(path))
                self.assertFalse(any(token.kind == 'string' and 'Catalog filter' in token.value
                                     for token in tokens), str(path))
        self.assertFalse((root / 'ui/patterns/navigation/sidebar_item.slint').exists())

    def test_reviewed_current_api_and_assets(self):
        result = check(Path(__file__).resolve().parents[2], metadata=False)
        self.assertEqual(result['exports'], 35)
        self.assertEqual(result['distribution']['svg_assets'], 32)


if __name__ == '__main__':
    unittest.main()
