# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import sys
from pathlib import Path
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from run_motion_bench import parse_measurement

class MotionMeasurementTests(unittest.TestCase):
    def sample(self):
        return '\n'.join(f'FRAME count=1 sample={i} ms={1+i%2}.0 pixel=243' for i in range(200)) + '\nIDLE count=1 seconds=60.000000 cpu_seconds=0.0312500 one_core_percent=0.052083 render_callbacks=0 hook_supported=false\nRESULT=PASS\n'

    def test_complete_samples_and_unsupported_hook(self):
        result = parse_measurement(self.sample(), 1)
        self.assertEqual(result['p95_ms'], 2)
        self.assertEqual(len(result['frames']), 200)
        self.assertIsNone(result['idle_render_callbacks'])

    def test_partial_duplicate_and_mismatched_data_fail_closed(self):
        text = self.sample()
        for bad in (text.replace('sample=199', 'sample=198'), text.replace('sample=199', 'sample=200'), text.replace('seconds=60.000000', 'seconds=59.999999'), text.replace('one_core_percent=0.052083', 'one_core_percent=0.000000')):
            with self.subTest(bad=bad[-160:]), self.assertRaises(ValueError):
                parse_measurement(bad, 1)
        with self.assertRaises(ValueError):
            parse_measurement(text, 20)
        with self.assertRaises(ValueError):
            parse_measurement('\n'.join(text.splitlines()[1:]), 1)

    def test_supported_render_count_is_retained(self):
        result = parse_measurement(self.sample().replace('render_callbacks=0 hook_supported=false', 'render_callbacks=3 hook_supported=true'), 1)
        self.assertEqual(result['idle_render_callbacks'], 3)

if __name__ == '__main__':
    unittest.main()
