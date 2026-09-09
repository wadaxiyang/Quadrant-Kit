# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Measurement integrity checks; compilation/runtime smoke is a separate gate."""
import sys
from pathlib import Path
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import run_perf


class PerfTests(unittest.TestCase):
    def test_percentiles_keep_outliers(self):
        self.assertEqual(run_perf.percentile([1, 2, 3, 100], .95), 100)
        self.assertEqual(run_perf.percentile([1, 2, 3, 100], .5), 2)

    def test_missing_hook_is_not_zero(self):
        result = run_perf.summarize([{'scene': 'empty', 'variant': 'native', 'construct_ms': 5}])
        self.assertEqual(result['empty']['native']['first_render_callback_ms'], {'n': 0, 'status': 'NOT_RUN'})

    def test_software_frame_does_not_fake_present_or_render_hook(self):
        result = run_perf.summarize([{'scene': 'buttons-1', 'variant': 'kit', 'first_software_frame_ms': 42.5}])
        self.assertEqual(result['buttons-1']['kit']['first_render_callback_ms'], {'n': 0, 'status': 'NOT_RUN'})
        self.assertEqual(result['buttons-1']['kit']['first_software_frame_ms']['p50'], 42.5)

    def test_pair_geometry_and_real_instances(self):
        for variant in ('native', 'kit'):
            source = run_perf.scene_source('buttons-100', variant)
            self.assertIn('for index in 100:', source)
            self.assertIn('width: 820px; height: 440px;', source)
            self.assertIn('width: 76px; height: 32px;', source)
        self.assertNotIn('FluentButton', run_perf.scene_source('buttons-100', 'native'))

    def test_import_only_does_not_initialize_kit(self):
        source = run_perf.scene_source('import-only', 'kit')
        self.assertIn('@quadrant-kit', source)
        self.assertNotIn('Theme.mode', source)
        self.assertNotIn('FluentButton {', source)
        self.assertNotIn('ToastHost {', source)

    def test_unknown_scene_fails(self):
        with self.assertRaises(ValueError):
            run_perf.scene_source('unknown', 'kit')

    def test_resolved_features_and_version_drift_fail(self):
        graph = {'packages': [{'id': 'slint', 'name': 'slint', 'version': '1.17.1', 'source': 'registry'}], 'resolve': {'nodes': [{'id': 'slint', 'features': ['default']}]}}
        original = run_perf.resolved_fingerprint(graph)
        graph['resolve']['nodes'][0]['features'].append('extra')
        self.assertNotEqual(run_perf.resolved_fingerprint(graph), original)
        graph['packages'][0]['version'] = '1.18.0'
        with self.assertRaises(ValueError):
            run_perf.resolved_fingerprint(graph)


if __name__ == '__main__':
    unittest.main()
