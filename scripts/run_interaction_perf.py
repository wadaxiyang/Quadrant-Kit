#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Public key/focus/wheel to software-buffer timing; OS IME/presentation is separate."""
import argparse
from datetime import datetime, timezone
import json
import math
import os
import shutil
import subprocess
from pathlib import Path

from run_perf import ROOT, execute, generate, percentile, resolved_fingerprint, source_identity

SCENES = ('text-empty', 'text-long', 'text-group', 'lists-100', 'lists-1000', 'lists-10000')


def source(variant):
    field, listing = ('FluentTextField', 'FluentListView') if variant == 'kit' else ('LineEdit', 'ListView')
    imports = 'import { Palette } from "std-widgets.slint";\n'
    imports += f'import {{ {field}, {listing} }} from "' + ('@quadrant-kit' if variant == 'kit' else 'std-widgets.slint') + '";\n'
    return imports + '''export component InteractionBench inherits Window {
        width: 820px; height: 440px; background: #f3f3f3;
        default-font-family: "Segoe UI Variable Text"; default-font-size: 14px;
        init => { Palette.color-scheme = ColorScheme.light; }
        in property <bool> list_scene;
        in property <bool> group;
        in property <int> row_count: 100;
        in-out property <string> input_text;
        out property <int> edits;
        out property <int> other_edits;
        out property <length> viewport_y: list.viewport-y;
        callback delegate_created();
        callback focus_input();
        callback focus_other();
        focus_input => { first.focus(); }
        focus_other => { other.focus(); }
        first := FIELD { visible: !root.list_scene; x:16px;y:16px;width:320px;height:32px; text <=> root.input_text; edited => { root.edits += 1; } }
        other := FIELD { visible: !root.list_scene; x:406px;y:16px;width:320px;height:32px;text:"Other"; edited => { root.other_edits += 1; } }
        if root.group: Rectangle {
            for index in 18: FIELD { x:16px + mod(index,2)*390px;y:56px + floor(index/2)*40px;width:320px;height:32px;text:"Group"; }
        }
        list := LISTING { visible:root.list_scene;x:20px;y:20px;width:780px;height:400px;
            for index in (root.list_scene ? root.row_count : 0): Text {
                height:24px;text:"Row " + index;color:#202020;
                init => { root.delegate_created(); }
            }
        }
    }
'''.replace('FIELD', field).replace('LISTING', listing)


def parse_samples(text):
    samples = [json.loads(line[7:]) for line in text.splitlines() if line.startswith('SAMPLE ')]
    if 'RESULT=PASS' not in text or [s.get('index') for s in samples] != list(range(200)):
        raise ValueError('Incomplete/duplicate interaction samples or failed runtime assertions')
    for sample in samples:
        for key in ('dispatch_ms', 'frame_ms', 'private_bytes', 'working_set_bytes', 'delegates_created'):
            value = sample.get(key)
            if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or value < 0:
                raise ValueError(f'Invalid {key}')
    return samples


def interaction_gates(runs):
    gates = {}
    for scene in SCENES:
        for metric in ('frame_ms', 'dispatch_plus_frame_ms'):
            values = {v: [(s['frame_ms'] if metric == 'frame_ms' else s['dispatch_ms']+s['frame_ms'])
                          for r in runs if r['scene']==scene and r['variant']==v for s in r['samples']]
                      for v in ('native','kit')}
            if not values['native'] or len(values['native']) != len(values['kit']):
                raise ValueError('Missing paired interaction metrics')
            a,b = (percentile(values[v],.95) for v in ('native','kit'))
            gates[scene+'/'+metric] = dict(native_p95=a,kit_p95=b,delta=b-a,allowance=max(1,a*.1),
                                          status='PASS' if b-a<=max(1,a*.1) else 'FAIL')
    return gates


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--pairs', type=int, default=3, help='Independent paired processes, each with 200 retained events')
    args = parser.parse_args()
    if args.pairs < 1:
        parser.error('positive pairs required')
    output = ROOT / 'target/interaction-perf' / datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    output.mkdir(parents=True)
    report = dict(source=source_identity(ROOT), status='IN_PROGRESS', runs=[], pairs=args.pairs,
                  scope='public WindowEvent to software snapshot; not OS IME, layout counter or actual present')
    env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT/'target'), SLINT_BACKEND='winit-software', SLINT_SCALE_FACTOR='1')
    for key in ('SLINT_STYLE', 'SLINT_DEFAULT_FONT', 'SLINT_FULLSCREEN', 'SLINT_DEBUG_PERFORMANCE'):
        env.pop(key, None)
    print('Report:', output/'result.json', flush=True)
    try:
        graph = None
        for variant in ('native', 'kit'):
            project = output/variant
            generate(project, source(variant), 'interaction-'+variant, True)
            shutil.copyfile(ROOT/'scripts/interaction_bench_host.rs', project/'src/main.rs')
            shutil.copyfile(ROOT/'scripts/windows_observation.rs', project/'src/windows_observation.rs')
            metadata = subprocess.check_output(['cargo','metadata','--offline','--format-version','1'], cwd=project,env=env,text=True,encoding='utf-8')
            (project/'resolved.json').write_text(metadata,encoding='utf-8')
            fingerprint = resolved_fingerprint(json.loads(metadata))
            if graph is not None and graph != fingerprint:
                raise ValueError('Native/Kit resolved graph differs')
            graph = fingerprint
            result = execute(['cargo','build','--locked','--offline','--release'], project, project/'build.log', env)
            if result['exit_code']:
                raise RuntimeError('Interaction build failed')
            shutil.copyfile(ROOT/f'target/release/interaction-{variant}.exe', project/'bench.exe')
        report['external_graph_sha256'] = graph
        for scene in SCENES:
            for pair in range(args.pairs):
                for variant in (('native','kit') if pair % 2 == 0 else ('kit','native')):
                    project = output/variant
                    log = project/f'{scene}-{pair}.log'
                    result = execute([str(project/'bench.exe'),scene], project, log, env)
                    if result['exit_code']:
                        raise RuntimeError('Interaction runtime failed')
                    samples = parse_samples(log.read_text(encoding='utf-8'))
                    report['runs'].append(dict(scene=scene, variant=variant, pair=pair, samples=samples, log=str(log)))
                    print(scene,variant,pair,'PASS',flush=True)
        report['gates'] = interaction_gates(report['runs'])
        report['budget_status'] = 'FAIL' if any(g['status']=='FAIL' for g in report['gates'].values()) else 'PASS'
        if source_identity(ROOT) != report['source']:
            raise RuntimeError('Source changed during measurement')
        report['status'] = report['budget_status']
    except (ValueError,RuntimeError,OSError,subprocess.SubprocessError) as error:
        report.update(status='FAIL',error=str(error))
    (output/'result.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(report['status'],flush=True)
    return int(report['status'] != 'PASS')


if __name__ == '__main__':
    raise SystemExit(main())
