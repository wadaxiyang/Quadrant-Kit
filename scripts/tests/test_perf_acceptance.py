# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import copy
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from evaluate_perf import evaluate
from run_perf import scene_source
from run_interaction_perf import parse_samples, interaction_gates, SCENES
import json


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
