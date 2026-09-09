# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Current migration policy and developer command input handling."""
import json
from pathlib import Path
import subprocess
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from check_native_reuse import implementation_digest, verify
from slint_contract import ContractError, parse

ROOT = Path(__file__).resolve().parents[2]


class ButtonCheckTests(unittest.TestCase):
    def test_migrated_button_cannot_downgrade_to_pending(self):
        manifest = json.loads((ROOT / 'scripts/native_reuse_manifest.json').read_text(encoding='utf-8'))
        record = next(r for r in manifest['records'] if r['component'] == 'FluentButton')
        parsed = parse((ROOT / record['implementation']).read_text(encoding='utf-8'))
        record.update(status='custom/pending-migration', classification='standard-control-pending',
                      reviewed_implementation_sha256=implementation_digest(parsed['definitions']['FluentButton'], parsed['bodies']['FluentButton']))
        with self.assertRaisesRegex(ContractError, 'New pending command debt'):
            verify(ROOT, manifest)

    def test_runner_help(self):
        result = subprocess.run([sys.executable, str(ROOT / 'scripts/run_button_checks.py'), '--help'], capture_output=True, text=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('--build-only', result.stdout)

    def test_runner_rejects_unknown_options(self):
        result = subprocess.run([sys.executable, str(ROOT / 'scripts/run_button_checks.py'), '--invented-input'], capture_output=True, text=True)
        self.assertEqual(result.returncode, 2)


if __name__ == '__main__':
    unittest.main()
