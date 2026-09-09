#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Build/run current Button/foundation input checks in an isolated native Slint host."""
import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

from run_perf import ROOT, execute, generate, resolved_fingerprint, source_identity


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--build-only', action='store_true', help='Build an interactive host without claiming input PASS')
    parser.add_argument('--suite', choices=['button', 'foundation', 'selection', 'numeric', 'containers', 'pickers', 'toast', 'modal'], default='button')
    parser.add_argument('--timeout-seconds', type=int, default=30, help='Bounded runtime allowance for lifecycle/idle suites (1..300)')
    args = parser.parse_args(argv)
    if not 1 <= args.timeout_seconds <= 300:
        parser.error('timeout-seconds must be 1..300')
    stem = 'foundation_check' if args.suite == 'foundation' else 'button_check'
    executable_name = 'kit-p3-foundation-check' if args.suite == 'foundation' else 'kit-p2-button-check'
    if args.suite == 'selection':
        stem, executable_name = 'selection_check', 'kit-p4a-selection-check'
    if args.suite == 'numeric':
        stem, executable_name = 'numeric_check', 'kit-p4b-numeric-check'
    if args.suite == 'containers':
        stem, executable_name = 'containers_check', 'kit-p4c-containers-check'
    if args.suite == 'pickers':
        stem, executable_name = 'pickers_check', 'kit-p4d-pickers-check'
    if args.suite == 'toast':
        stem, executable_name = 'toast_check', 'kit-p5a-toast-check'
    if args.suite == 'modal':
        stem, executable_name = 'modal_check', 'kit-p5b-modal-check'
    output = ROOT / 'target/button-checks' / datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    source = source_identity(ROOT)
    report = {'suite': args.suite, 'source': source, 'status': 'IN_PROGRESS', 'commands': [], 'backend': 'winit-software', 'input': 'public WindowEvent dispatch; OS input/accessibility is separate'}
    project = output / 'consumer'
    generate(project, (ROOT / ('scripts/' + stem + '.slint')).read_text(encoding='utf-8'), executable_name, True)
    manifest = project / 'Cargo.toml'
    manifest.write_text(manifest.read_text(encoding='utf-8').replace('[dependencies]', '[dependencies]\npng = "=0.18.1"'), encoding='utf-8')
    shutil.copyfile(ROOT / ('scripts/' + stem + '_host.rs'), project / 'src/main.rs')
    env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / 'target'), SLINT_BACKEND='winit-software', SLINT_SCALE_FACTOR='1')
    for name in ('SLINT_STYLE', 'SLINT_DEFAULT_FONT', 'SLINT_FULLSCREEN', 'SLINT_DEBUG_PERFORMANCE'):
        env.pop(name, None)
    print(f'Report: {output / "result.json"}', flush=True)
    try:
        with (output / 'resolved.json').open('w', encoding='utf-8') as stdout, (output / 'resolve.log').open('w', encoding='utf-8') as stderr:
            subprocess.run(['cargo', 'metadata', '--offline', '--format-version', '1'], cwd=project, env=env, stdout=stdout, stderr=stderr, check=True, timeout=120)
        report['external_graph_sha256'] = resolved_fingerprint(json.loads((output / 'resolved.json').read_text(encoding='utf-8')))
        build = execute(['cargo', 'build', '--locked', '--offline'], project, output / 'build.log', env)
        report['commands'].append(build)
        if build['exit_code']:
            raise RuntimeError('Button verification build failed')
        binary = ROOT / 'target/debug' / (executable_name + '.exe' if os.name == 'nt' else executable_name)
        saved = output / binary.name
        shutil.copyfile(binary, saved)
        report['binary'] = str(saved)
        if not args.build_only:
            command = [str(saved), str(output / 'visual')]
            with (output / 'runtime.log').open('w', encoding='utf-8') as stream:
                result = subprocess.run(command, cwd=output, env=env, stdout=stream, stderr=subprocess.STDOUT, timeout=args.timeout_seconds)
            report['commands'].append({'command': command, 'exit_code': result.returncode, 'log': str(output / 'runtime.log')})
            if result.returncode or 'RESULT=PASS' not in (output / 'runtime.log').read_text(encoding='utf-8'):
                raise RuntimeError('Button runtime checks failed')
        if source != source_identity(ROOT):
            raise RuntimeError('Sources changed during verification')
        report['status'] = 'BUILT_NOT_RUN' if args.build_only else 'PASS'
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        report.update(status='FAIL', error=str(error))
    (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    print(report['status'], flush=True)
    return int(report['status'] == 'FAIL')


if __name__ == '__main__':
    sys.exit(main())
