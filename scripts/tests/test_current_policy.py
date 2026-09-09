# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import json
from pathlib import Path
import re
import sys
import unittest

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

    def test_readme_keeps_spec_mermaid_source_and_static_entry(self):
        readme = (ROOT / 'README.md').read_text(encoding='utf-8')
        spec = (ROOT / 'docs/specs/QUADRANT_KIT_FLUENT_EVOLUTION_SPEC.md').read_text(encoding='utf-8')
        target = re.search(r'### 3\.5.*?```mermaid\n(.*?)\n```', spec, re.S)[1]
        self.assertIn('```mermaid\n' + target + '\n```', readme)
        self.assertIn('静态导出', readme)
        self.assertIn('patterns 与 overlays 不互相导入', readme)


if __name__ == '__main__':
    unittest.main()
