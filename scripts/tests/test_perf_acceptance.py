# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import copy
import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from evaluate_perf import evaluate
import run_perf
from run_perf import scene_source
from run_interaction_perf import parse_samples, interaction_gates, SCENES
from run_motion_bench import parse_measurement, parse_memory


class AcceptanceTests(unittest.TestCase):
    def report(self, scene='buttons-1'):
        return {'status': 'PASS', 'profile': 'release', 'source': {'sha': 'fixture'},
                'samples': [dict(scene=scene, variant=v, index=i, exit_code=0,
                                 first_software_frame_ms=10, private_bytes=10_000_000,
                                 binary_bytes=10_000_000) for i in range(30) for v in ('native', 'kit')]}

    def test_complete_pair_passes_but_does_not_claim_present(self):
        result = evaluate(self.report())
        self.assertEqual(result['status'], 'PASS')
        self.assertIn('actual presentation', result['not_run'])

    def test_missing_duplicate_nonfinite_and_failed_samples_rejected(self):
        for mutation in ('missing', 'duplicate', 'nan', 'exit'):
            report = copy.deepcopy(self.report())
            if mutation == 'missing':
                report['samples'].pop()
            elif mutation == 'duplicate':
                report['samples'][0]['index'] = 1
            elif mutation == 'nan':
                report['samples'][0]['private_bytes'] = float('nan')
            else:
                report['samples'][0]['exit_code'] = 1
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                evaluate(report)

    def test_regression_and_unmatched_reference_cannot_pass(self):
        report = self.report()
        for sample in report['samples']:
            if sample['variant'] == 'kit':
                sample['binary_bytes'] += 600_000
        self.assertEqual(evaluate(report)['status'], 'FAIL')
        self.assertEqual(evaluate(self.report('hidden-toast'))['status'], 'BLOCKED')

    def test_sizes_and_text_are_real_matched_content(self):
        for category, counts in (('buttons', (1, 100, 1000)), ('selection', (1, 100, 1000)), ('lists', (100, 1000, 10000))):
            for count in counts:
                for variant in ('native', 'kit'):
                    self.assertIn(f'for index in {count}:', scene_source(f'{category}-{count}', variant))
        for variant in ('native', 'kit'):
            self.assertIn('text: "";', scene_source('text-empty', variant))
            self.assertEqual(scene_source('text-long', variant).count('Long text 输入 '), 200)
            self.assertIn('for index in 20:', scene_source('text-group', variant))

    def test_interaction_requires_every_event_and_finite_metrics(self):
        samples = [dict(index=i, dispatch_ms=.1, frame_ms=1, private_bytes=100,
                        working_set_bytes=200, delegates_created=10) for i in range(200)]
        def raw(rows):
            return '\n'.join('SAMPLE '+json.dumps(s) for s in rows)+'\nRESULT=PASS\n'
        self.assertEqual(len(parse_samples(raw(samples))), 200)
        with self.assertRaises(ValueError):
            parse_samples(raw(samples[:-1]))
        samples[4]['frame_ms'] = float('nan')
        with self.assertRaises(ValueError):
            parse_samples(raw(samples))

    def test_hidden_composition_accounts_for_actual_resources(self):
        native = scene_source('hidden-toast-composed', 'native')
        kit = scene_source('hidden-toast-composed', 'kit')
        self.assertNotIn('@quadrant-kit', native)
        self.assertIn('info-24-regular.svg', native)
        self.assertIn('dismiss-16-regular.svg', native)
        self.assertIn('Tooltip {', native)
        self.assertIn('ToastHost {', kit)
        for source in (native,kit):
            self.assertIn('in-out property <bool> transient_shown: false;',source)

    def test_dispatch_regression_cannot_hide_behind_fast_snapshot(self):
        runs = [dict(scene=scene,variant=v,samples=[dict(frame_ms=1,dispatch_ms=(5 if v=='kit' else 1))])
                for scene in SCENES for v in ('native','kit')]
        gates = interaction_gates(runs)
        self.assertEqual(gates['text-long/frame_ms']['status'], 'PASS')
        self.assertEqual(gates['text-long/dispatch_plus_frame_ms']['status'], 'FAIL')
        with self.assertRaises(ValueError):
            interaction_gates([])


class PerfHarnessTests(unittest.TestCase):
    def test_percentiles_keep_outliers(self):
        self.assertEqual(run_perf.percentile([1, 2, 3, 100], .95), 100)
        self.assertEqual(run_perf.percentile([1, 2, 3, 100], .5), 2)

    def test_missing_hooks_are_not_reported_as_zero(self):
        result = run_perf.summarize([{'scene': 'empty', 'variant': 'native', 'construct_ms': 5}])
        self.assertEqual(result['empty']['native']['first_render_callback_ms'], {'n': 0, 'status': 'NOT_RUN'})
        result = run_perf.summarize([{'scene': 'buttons-1', 'variant': 'kit', 'first_software_frame_ms': 42.5}])
        self.assertEqual(result['buttons-1']['kit']['first_render_callback_ms'], {'n': 0, 'status': 'NOT_RUN'})
        self.assertEqual(result['buttons-1']['kit']['first_software_frame_ms']['p50'], 42.5)

    def test_generated_pairs_keep_matching_content_and_geometry(self):
        for variant in ('native', 'kit'):
            buttons = run_perf.scene_source('buttons-100', variant)
            self.assertIn('for index in 100:', buttons)
            self.assertIn('width: 820px; height: 440px;', buttons)
            self.assertIn('width: 76px; height: 32px;', buttons)
            icons = run_perf.scene_source('icons-100', variant)
            self.assertIn('for index in 100:', icons)
            self.assertIn('width: 44px; height: 32px;', icons)
            self.assertIn('add-16-regular.svg', icons)
            progress = run_perf.scene_source('progress-100', variant)
            self.assertEqual(progress.count('for index in 50:'), 2)
            self.assertEqual(progress.count('progress: 0.6; indeterminate: false;'), 2)
            self.assertIn('width: 76px; height: 3px;', progress)
            self.assertIn('width: 32px; height: 32px;', progress)
            virtual_list = run_perf.scene_source('lists-10000', variant)
            self.assertIn('for index in 10000: Text { height: 24px;', virtual_list)
            self.assertIn('width: 780px; height: 400px;', virtual_list)
            self.assertNotIn('@children', virtual_list)
            table = run_perf.scene_source('table-100', variant)
            self.assertEqual(table.count('{text: "Row '), 100)
            self.assertEqual(table.count('width: 300px'), 2)
            self.assertIn('width: 780px; height: 400px;', table)
        self.assertNotIn('FluentButton', run_perf.scene_source('buttons-100', 'native'))
        native = run_perf.scene_source('segments-100', 'native')
        self.assertIn('checkable: false;', native)
        self.assertIn('checked: mod(index, 2) == 0;', native)
        self.assertIn('selected: mod(index, 2) == 0;', run_perf.scene_source('segments-100', 'kit'))

    def test_import_only_and_invalid_scenes(self):
        source = run_perf.scene_source('import-only', 'kit')
        self.assertIn('@quadrant-kit', source)
        self.assertNotIn('Theme.mode', source)
        self.assertNotIn('FluentButton {', source)
        self.assertNotIn('ToastHost {', source)
        with self.assertRaises(ValueError):
            run_perf.scene_source('unknown', 'kit')

    def test_resolved_features_and_version_drift_fail(self):
        graph = {'packages': [{'id': 'slint', 'name': 'slint', 'version': '1.17.1', 'source': 'registry'}],
                 'resolve': {'nodes': [{'id': 'slint', 'features': ['default']}]}}
        original = run_perf.resolved_fingerprint(graph)
        graph['resolve']['nodes'][0]['features'].append('extra')
        self.assertNotEqual(run_perf.resolved_fingerprint(graph), original)
        graph['packages'][0]['version'] = '1.18.0'
        with self.assertRaises(ValueError):
            run_perf.resolved_fingerprint(graph)


class MotionMeasurementTests(unittest.TestCase):
    def sample(self):
        frames = '\n'.join(f'FRAME count=1 sample={i} ms={1+i%2}.0 pixel=243' for i in range(200))
        return frames + '\nIDLE count=1 seconds=60.000000 cpu_seconds=0.0312500 one_core_percent=0.052083 render_callbacks=0 hook_supported=false\nRESULT=PASS\n'

    def test_complete_samples_and_render_hook_states(self):
        result = parse_measurement(self.sample(), 1)
        self.assertEqual(result['p95_ms'], 2)
        self.assertEqual(len(result['frames']), 200)
        self.assertIsNone(result['idle_render_callbacks'])
        supported = parse_measurement(
            self.sample().replace('render_callbacks=0 hook_supported=false',
                                  'render_callbacks=3 hook_supported=true'), 1)
        self.assertEqual(supported['idle_render_callbacks'], 3)

    def test_partial_duplicate_and_mismatched_data_fail_closed(self):
        text = self.sample()
        bad_samples = (text.replace('sample=199', 'sample=198'),
                       text.replace('sample=199', 'sample=200'),
                       text.replace('seconds=60.000000', 'seconds=59.999999'),
                       text.replace('one_core_percent=0.052083', 'one_core_percent=0.000000'))
        for bad in bad_samples:
            with self.subTest(bad=bad[-160:]), self.assertRaises(ValueError):
                parse_measurement(bad, 1)
        with self.assertRaises(ValueError):
            parse_measurement(text, 20)
        with self.assertRaises(ValueError):
            parse_measurement('\n'.join(text.splitlines()[1:]), 1)

    def test_lifecycle_memory_requires_all_checkpoints(self):
        lines = [f'MEMORY sample={i} private_bytes=100 working_set_bytes=200' for i in range(19, 200, 20)]
        lines.append('MEMORY_FINAL private_bytes=100 working_set_bytes=200')
        self.assertEqual(len(parse_memory('\n'.join(lines))['cycles']), 10)
        with self.assertRaises(ValueError):
            parse_memory('\n'.join(lines[1:]))


if __name__ == '__main__':
    unittest.main()
