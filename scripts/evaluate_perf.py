#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Evaluate retained allowances without equating collection with acceptance."""
import argparse
import json
import math
from pathlib import Path

from run_perf import percentile


def evaluate(report):
    if report.get('status') != 'PASS' or report.get('profile') != 'release':
        raise ValueError('Require a completed release collection')
    samples = report.get('samples', [])
    if not samples:
        raise ValueError('Missing samples')
    if any(s.get('variant') not in ('native', 'kit') for s in samples):
        raise ValueError('Unknown sample variant')
    result = {'status': 'PASS', 'source': report['source'], 'scenes': {},
              'scope': 'software-buffer milestone and fixed 2s observations; not present or proven steady state',
              'not_run': ['actual presentation', 'cold start', 'GPU memory', 'steady-state acceptance']}
    for scene in sorted({s['scene'] for s in samples}):
        groups = {v: [s for s in samples if s['scene'] == scene and s['variant'] == v]
                  for v in ('native', 'kit')}
        indices = [{s['index'] for s in groups[v]} for v in groups]
        count = len(groups['native'])
        if count < 30 or len(groups['kit']) != count or indices[0] != indices[1] or indices[0] != set(range(count)):
            raise ValueError(f'{scene}: require >=30 complete unique ordered-index pairs')
        checks = {}
        for metric, fraction, absolute, relative in (
            ('first_software_frame_ms', .5, 10, .10),
            ('first_software_frame_ms', .95, 20, .15),
            ('private_bytes', .5, 2 * 1024**2, .05),
            ('binary_bytes', .5, 512 * 1024, .05),
        ):
            values = {}
            for variant, group in groups.items():
                raw = [s.get(metric) for s in group]
                if any(isinstance(x, bool) or not isinstance(x, (int, float)) or not math.isfinite(x) or x < 0 for x in raw):
                    raise ValueError(f'{scene}/{variant}: incomplete or invalid {metric}')
                if any(s.get('exit_code') != 0 for s in group):
                    raise ValueError('Nonzero measured process exit')
                values[variant] = percentile(raw, fraction)
            delta = values['kit'] - values['native']
            allowance = max(absolute, relative * values['native'])
            checks[f'{metric}_p{round(fraction * 100)}'] = {
                **values, 'delta': delta, 'allowance': allowance,
                'status': 'PASS' if delta <= allowance else 'FAIL'}
        status = 'FAIL' if any(c['status'] == 'FAIL' for c in checks.values()) else 'PASS'
        # An empty window is a useful cost baseline but is not an equivalent Toast.
        if scene == 'hidden-toast':
            status = 'BLOCKED'
        result['scenes'][scene] = {'n_pairs': count, 'checks': checks, 'status': status,
                                  'comparison': 'unmatched empty reference' if scene == 'hidden-toast' else 'matched current scene'}
    statuses = {s['status'] for s in result['scenes'].values()}
    result['status'] = 'FAIL' if 'FAIL' in statuses else 'BLOCKED' if 'BLOCKED' in statuses else 'PASS'
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('report', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = evaluate(json.loads(args.report.read_text(encoding='utf-8')))
    args.output.write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
    print(result['status'])
    return int(result['status'] != 'PASS')


if __name__ == '__main__':
    raise SystemExit(main())
