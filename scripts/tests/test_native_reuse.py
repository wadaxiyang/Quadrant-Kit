# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from check_native_reuse import implementation_digest, verify
from slint_contract import ContractError, implementation_facts, parse, public_api


class NativeReuseTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name).resolve()
        (self.root / 'ui/primitives').mkdir(parents=True)
        (self.root / 'gallery/ui').mkdir(parents=True)
        self.source = 'import { Button as NativeButton } from "std-widgets.slint"; export component Command { NativeButton { text: "Go"; } }'
        self.write(self.source)

    def write(self, source):
        (self.root / 'ui/primitives/command.slint').write_text(source, encoding='utf-8')
        (self.root / 'ui/kit.slint').write_text('export { Command } from "primitives/command.slint";', encoding='utf-8')

    def record(self, status='native-wrapper'):
        classification = 'standard-control-wrapper' if status == 'native-wrapper' else 'presentation' if status == 'presenter' else 'standard-control-pending' if status == 'custom/pending-migration' else 'composed-missing-standard-control'
        return {'component': 'Command', 'implementation': 'ui/primitives/command.slint',
                'public': True, 'classification': classification, 'status': status,
                'native_owner': 'Button', 'native_dependencies': ['Button'],
                'missing_capability': '', 'permitted_custom_behavior': [],
                'forbidden_custom_behavior': ['duplicate input'], 'validation': ['compile', 'input'],
                'review_on_slint_upgrade': True}

    def manifest(self, record=None):
        return {'schema_version': 1, 'slint_version': '1.17.1', 'records': [record or self.record()]}

    def test_actual_aliased_native_owner_and_comments(self):
        self.write(self.source.replace('text: "Go";', 'text: "TouchArea { opacity: 0; }"; /* FocusScope {} */'))
        result = verify(self.root, self.manifest())
        self.assertFalse(result['runtime_verified'])
        self.assertEqual(result['pending_migration'], [])

    def test_import_only_and_fake_local_button_do_not_count(self):
        for source in [self.source.replace('NativeButton { text: "Go"; }', 'Text { text: "Go"; }'),
                       'export component Command inherits Button {}']:
            self.write(source)
            with self.assertRaises(ContractError):
                verify(self.root, self.manifest())
        self.write('component Button {} export component Command { Button {} }')
        with self.assertRaisesRegex(ContractError, 'records differ'):
            verify(self.root, self.manifest())

    def test_duplicate_input_keyboard_and_accessibility_fail(self):
        for extra in ('TouchArea { clicked => {} }', 'FocusScope {}',
                      'accessible-action-default => {}', 'key-pressed(event) => { return reject; }',
                      'NativeButton { text: "Second"; }'):
            self.write(self.source.replace('NativeButton {', extra + ' NativeButton {', 1))
            with self.subTest(extra=extra), self.assertRaisesRegex(ContractError, 'duplicate input'):
                verify(self.root, self.manifest())

    def test_hidden_native_proxy_fails(self):
        for binding in ('opacity: 0;', 'opacity: (0.0);', 'visible: false;'):
            self.write(self.source.replace('text: "Go";', binding + ' text: "Go";'))
            with self.assertRaisesRegex(ContractError, 'hidden native proxy'):
                verify(self.root, self.manifest())

    def test_missing_stale_duplicate_or_private_records_fail(self):
        for mutate in (lambda m: m['records'].clear(),
                       lambda m: m['records'].append(copy.deepcopy(m['records'][0])),
                       lambda m: m['records'][0].update(public=False),
                       lambda m: m['records'][0].update(component='Gone')):
            manifest = self.manifest()
            mutate(manifest)
            with self.assertRaises(ContractError):
                verify(self.root, manifest)

    def test_private_import_and_composition_hiding_input_fail(self):
        helper = self.root / 'ui/primitives/helper.slint'
        helper.write_text('export component Helper { TouchArea {} }', encoding='utf-8')
        self.write('import { Helper } from "helper.slint"; ' + self.source.replace('NativeButton { text: "Go"; }', 'NativeButton { text: "Go"; } Helper {}'))
        private = self.record('reviewed-exception')
        parsed = parse(helper.read_text())
        private.update(component='Helper', implementation='ui/primitives/helper.slint', public=False,
                       native_dependencies=[], missing_capability='Composition-only input',
                       permitted_custom_behavior=['specific helper action'],
                       reviewed_implementation_sha256=implementation_digest(parsed['definitions']['Helper'], parsed['bodies']['Helper']))
        manifest = self.manifest()
        manifest['records'].append(private)
        with self.assertRaisesRegex(ContractError, 'duplicate input'):
            verify(self.root, manifest)

    def test_new_pending_commands_rejected_even_with_digest(self):
        self.write('export component Command { TouchArea {} }')
        parsed = parse((self.root / 'ui/primitives/command.slint').read_text())
        record = self.record('custom/pending-migration')
        record.update(native_dependencies=[], missing_capability='Pending ordinary action', permitted_custom_behavior=['click'],
                      reviewed_implementation_sha256=implementation_digest(parsed['definitions']['Command'], parsed['bodies']['Command']))
        with self.assertRaisesRegex(ContractError, 'New pending command debt'):
            verify(self.root, self.manifest(record))

    def test_reviewed_exception_changes_require_review(self):
        self.write('export component Command { TouchArea {} }')
        parsed = parse((self.root / 'ui/primitives/command.slint').read_text())
        record = self.record('reviewed-exception')
        record.update(native_dependencies=[], missing_capability='Specific compound gesture', permitted_custom_behavior=['compound gesture'],
                      reviewed_implementation_sha256=implementation_digest(parsed['definitions']['Command'], parsed['bodies']['Command']))
        verify(self.root, self.manifest(record))
        self.write('export component Command { TouchArea {} FocusScope {} }')
        with self.assertRaisesRegex(ContractError, 'without scoped review'):
            verify(self.root, self.manifest(record))

    def test_native_types_reexport_and_builtin_references(self):
        (self.root / 'ui/kit.slint').write_text('export { Date as SelectedDate, Time } from "std-widgets.slint";', encoding='utf-8')
        api = public_api(self.root)
        self.assertEqual(api['SelectedDate']['signature']['fields'], [{'name': n, 'type': 'int'} for n in ('day', 'month', 'year')])
        (self.root / 'ui/primitives/types.slint').write_text('import { Date as D } from "std-widgets.slint"; export { D as ChoiceDate }', encoding='utf-8')
        (self.root / 'ui/kit.slint').write_text('export { ChoiceDate as SelectedDate } from "primitives/types.slint";', encoding='utf-8')
        self.assertEqual(public_api(self.root)['SelectedDate'], api['SelectedDate'])
        parsed = parse('export component C { in property <[[StandardListViewItem]]> rows; in-out property <[TableColumn]> columns; }')
        self.assertEqual(parsed['definitions']['C']['signature']['members']['rows']['type'], '[ [ StandardListViewItem ] ]')
        for name in ('RadioButton', 'TableColumn', 'Unknown', 'Button'):
            (self.root / 'ui/kit.slint').write_text(f'export {{ {name} }} from "std-widgets.slint";', encoding='utf-8')
            with self.assertRaisesRegex(ContractError, 'Unverified'):
                public_api(self.root)

    def test_compiler_special_native_export_has_real_owner(self):
        self.write('export { RadioGroup as Command } from "std-widgets.slint";')
        record = self.record()
        record.update(native_owner='RadioGroup', native_dependencies=['RadioGroup'])
        self.assertEqual(verify(self.root, self.manifest(record))['public_components'], 1)
        self.assertEqual(public_api(self.root)['Command']['signature']['inherits'], 'RadioGroup')
        record['native_owner'] = 'Button'
        with self.assertRaisesRegex(ContractError, 'owner mismatch'):
            verify(self.root, self.manifest(record))
        with self.assertRaisesRegex(ContractError, 'records differ'):
            verify(self.root, {'schema_version': 1, 'slint_version': '1.17.1', 'records': []})

    def test_current_repository_and_pending_debt_are_explicit(self):
        root = Path(__file__).resolve().parents[2]
        result = verify(root)
        records = json.loads((root / 'scripts/native_reuse_manifest.json').read_text())['records']
        self.assertEqual(result['public_components'], sum(r['public'] for r in records))
        self.assertEqual(set(result['pending_migration']), {r['component'] for r in records if r['status'] == 'custom/pending-migration'})

    def test_presenter_and_foundation_cannot_hide_input(self):
        self.write('export component Command { TouchArea {} }')
        record = self.record('presenter')
        record['native_dependencies'] = []
        with self.assertRaisesRegex(ContractError, 'Unreviewed custom input'):
            verify(self.root, self.manifest(record))
        (self.root / 'ui/foundation/private').mkdir(parents=True)
        (self.root / 'ui/foundation/private/hidden.slint').write_text('global Helpers { Timer {} }', encoding='utf-8')
        with self.assertRaisesRegex(ContractError, 'Foundation instantiates UI'):
            verify(self.root, self.manifest(record))


if __name__ == '__main__':
    unittest.main()
