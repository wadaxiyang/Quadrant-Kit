# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import json
from pathlib import Path
import re
import sys
import unittest
from urllib.parse import unquote, urlsplit

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from gallery_catalog import load_catalog
from slint_contract import public_api

ROOT = Path(__file__).resolve().parents[2]


class CurrentPolicyTests(unittest.TestCase):
    def test_status_manifest_and_catalog_follow_current_visual_set(self):
        visuals = {n for n, v in public_api(ROOT).items() if v['signature']['kind'] == 'component'}
        manifest = json.loads((ROOT / 'scripts/native_reuse_manifest.json').read_text(encoding='utf-8'))
        self.assertEqual({r['component'] for r in manifest['records'] if r['public']}, visuals)
        document = (ROOT / 'docs/COMPONENT_STATUS.md').read_text(encoding='utf-8').split('## Planned names')[0]
        documented = re.findall(r'^\| (\w+) \| IMPLEMENTED', document, re.M)
        self.assertEqual(set(documented), visuals)
        self.assertEqual(len(documented), len(set(documented)))
        routes = {name: row['id'] for row in load_catalog() for name in row['exports'].split(',') if name}
        for name in visuals:
            row = next(line for line in document.splitlines() if line.startswith('| ' + name + ' |'))
            self.assertIn('| ' + routes[name] + ' |', row)

    def test_readme_keeps_architecture_mermaid_and_static_entry(self):
        readme = (ROOT / 'README.md').read_text(encoding='utf-8')
        spec = (ROOT / 'docs/specs/QUADRANT_KIT_FLUENT_EVOLUTION_SPEC.md').read_text(encoding='utf-8')
        diagrams = re.findall(r'```mermaid\n(.*?)\n```', readme, re.S)
        self.assertEqual(len(diagrams), 1)
        self.assertTrue(diagrams[0].startswith('flowchart TB\n'))
        edges = re.findall(r'^\s*(\w+)\s+(-->|-\.->)(?:\|[^\n]*?\|)?\s+(\w+)\s*$', diagrams[0], re.M)
        expected = {
            ('Facade', '-->', layer) for layer in ('Patterns', 'Overlays', 'Primitives', 'Foundation')
        } | {
            (layer, '-->', dependency) for layer in ('Patterns', 'Overlays')
            for dependency in ('Primitives', 'Foundation')
        } | {
            ('Primitives', '-->', 'Foundation'), ('App', '-->', 'Facade'),
            ('Gallery', '-->', 'Facade'), ('Build', '-.->', 'Facade'),
        } | {(layer, '-->', 'Slint') for layer in ('Patterns', 'Overlays', 'Primitives', 'Foundation')}
        self.assertEqual(set(edges), expected)
        self.assertEqual(len(edges), len(expected))
        self.assertNotIn('```mermaid', spec)
        self.assertIn('../../README.md', spec)
        self.assertIn('静态导出', readme)
        self.assertIn('patterns 与 overlays 不互相导入', readme)

    def test_current_document_links_and_anchors_resolve(self):
        # Current guides must not link to removed phase reports. Historical paths
        # in Git retrieval commands are code, not links to the current checkout.
        documents = sorted((ROOT / 'docs').rglob('*.md')) + sorted(ROOT.glob('*.md'))
        documents += [ROOT / 'ui/AGENTS.md', ROOT / 'gallery/AGENTS.md']

        def prose(path):
            source = path.read_text(encoding='utf-8')
            return re.sub(r'^```[^\n]*\n.*?^```[^\n]*$', '', source, flags=re.M | re.S)

        def anchors(path):
            result = set()
            for heading in re.findall(r'^#{1,6}\s+(.+?)\s*#*$', prose(path), re.M):
                slug = re.sub(r'[^\w\- ]', '', heading.lower()).replace(' ', '-')
                unique = slug
                suffix = 0
                while unique in result:
                    suffix += 1
                    unique = f'{slug}-{suffix}'
                result.add(unique)
            return result

        for document in documents:
            for destination in re.findall(r'\[[^\]\n]*\]\(([^)\n]+)\)', prose(document)):
                link = urlsplit(destination.strip('<>'))
                if link.scheme or link.netloc:
                    continue
                target = (document.parent / unquote(link.path)).resolve() if link.path else document
                with self.subTest(document=str(document.relative_to(ROOT)), link=destination):
                    self.assertTrue(target.exists(), f'Missing link target: {target}')
                    if target.is_file() and target.suffix == '.md' and link.fragment:
                        self.assertIn(unquote(link.fragment), anchors(target))


if __name__ == '__main__':
    unittest.main()
