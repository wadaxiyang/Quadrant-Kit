#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Neutral current native/Kit A/B harness; raw collection is separate from budgets."""
import argparse
import ctypes
from ctypes import wintypes
from datetime import datetime, timezone
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import threading
import time

from capture_gallery_baseline import source_identity

ROOT = Path(__file__).resolve().parents[1]
SCENES = ('empty', 'import-only', 'buttons-1', 'buttons-100', 'buttons-1000',
          'text-input', 'text-empty', 'text-long', 'text-group', 'hidden-toast',
          'icons-100', 'segments-100', 'selection-1', 'selection-100',
          'selection-1000', 'progress-100', 'lists-100', 'lists-1000',
          'lists-10000', 'table-100', 'hidden-toast-composed')
NEGATIVE = {
    'palette-write': ('import { Palette } from "std-widgets.slint"; export component Probe inherits Window { init => { Palette.accent-background = #ff0000; } }', 'Assignment on a output property'),
    'radio-index': ('import { RadioGroup } from "std-widgets.slint"; export component Probe inherits Window { RadioGroup { current-index: 0; RadioButton { text: "One"; } } }', 'Unknown property current-index'),
    'list-slot': ('import { ListView } from "std-widgets.slint"; component Wrapper inherits Rectangle { ListView { @children } } export component Probe inherits Window { Wrapper { for value in ["One"]: Text { text: value; } } }', "A ListView can just have a single 'for'"),
}


def scene_source(scene, variant):
    """Only the implementation under comparison varies; geometry stays paired."""
    if scene not in SCENES or variant not in ('native', 'kit'):
        raise ValueError('Unknown scenario or variant')
    imports = 'import { Button, LineEdit, Palette } from "std-widgets.slint";\n'
    kit = variant == 'kit' and scene != 'empty'
    if kit:
        imports += 'import { FluentButton, IconButton, SegmentButton, FluentTextField, ToastHost, Theme, ThemeMode } from "@quadrant-kit";\n'
    body = ''
    if scene.startswith('buttons-'):
        count = int(scene.split('-')[1])
        control = 'FluentButton' if kit else 'Button'
        body = f'for index in {count}: {control} {{ x: mod(index, 10) * 80px; y: floor(index / 10) * 40px; width: 76px; height: 32px; text: "Test"; enabled: true; }}'
    elif scene == 'icons-100':
        control = 'IconButton' if kit else 'Button'
        asset = (ROOT / 'assets/icons/add-16-regular.svg').as_posix()
        extra = 'tooltip: "Add";' if kit else 'icon-size: 20px; colorize-icon: true; accessible-label: "Add"; Tooltip { Text { text: "Add"; } }'
        body = f'for index in 100: {control} {{ x: mod(index, 10) * 80px; y: floor(index / 10) * 40px; width: 44px; height: 32px; icon: @image-url("{asset}"); enabled: true; {extra} }}'
    elif scene == 'segments-100':
        control = 'SegmentButton' if kit else 'Button'
        state = 'selected: mod(index, 2) == 0;' if kit else 'checked: mod(index, 2) == 0; checkable: false; accessible-checkable: true;'
        body = f'for index in 100: {control} {{ x: mod(index, 10) * 80px; y: floor(index / 10) * 40px; width: 76px; height: 32px; text: "Test"; enabled: true; {state} }}'
    elif scene.startswith('selection-'):
        count = int(scene.split('-')[1])
        imports += 'import { CheckBox } from "std-widgets.slint";\n'
        if kit:
            imports += 'import { FluentCheckBox } from "@quadrant-kit";\n'
        control = 'FluentCheckBox' if kit else 'CheckBox'
        body = f'for index in {count}: {control} {{ x: mod(index, 10) * 80px; y: floor(index / 10) * 40px; width: 76px; height: 32px; text: "Test"; checked: mod(index, 2) == 0; enabled: true; }}'
    elif scene == 'progress-100':
        imports += 'import { ProgressIndicator, Spinner } from "std-widgets.slint";\n'
        if kit:
            imports += 'import { FluentProgressBar, FluentProgressRing } from "@quadrant-kit";\n'
        bar, ring = ('FluentProgressBar', 'FluentProgressRing') if kit else ('ProgressIndicator', 'Spinner')
        body = f'for index in 50: {bar} {{ x: mod(index, 10) * 80px; y: floor(index / 10) * 80px + 10px; width: 76px; height: 3px; progress: 0.6; indeterminate: false; }}\n'
        body += f'for index in 50: {ring} {{ x: mod(index, 10) * 80px + 20px; y: floor(index / 10) * 80px + 20px; width: 32px; height: 32px; progress: 0.6; indeterminate: false; }}'
    elif scene.startswith('lists-'):
        count = int(scene.split('-')[1])
        imports += 'import { ListView } from "std-widgets.slint";\n'
        if kit:
            imports += 'import { FluentListView } from "@quadrant-kit";\n'
        control = 'FluentListView' if kit else 'ListView'
        body = f'{control} {{ x: 20px; y: 20px; width: 780px; height: 400px; for index in {count}: Text {{ height: 24px; text: "Row " + index; color: #202020; }} }}'
    elif scene == 'table-100':
        imports += 'import { StandardTableView } from "std-widgets.slint";\n'
        if kit:
            imports += 'import { FluentStandardTableView } from "@quadrant-kit";\n'
        control = 'FluentStandardTableView' if kit else 'StandardTableView'
        rows = ','.join('[' + '{text: "Row ' + str(i) + '"}, {text: "Value"}' + ']' for i in range(100))
        body = f'{control} {{ x: 20px; y: 20px; width: 780px; height: 400px; columns: [{{title: "Name", width: 300px}}, {{title: "Value", width: 300px}}]; rows: [{rows}]; }}'
    elif scene in ('text-input', 'text-empty', 'text-long', 'text-group'):
        control = 'FluentTextField' if kit else 'LineEdit'
        value = '' if scene == 'text-empty' else ('Long text 输入 ' * 200 if scene == 'text-long' else 'Text 输入')
        count = 20 if scene == 'text-group' else 1
        body = f'for index in {count}: {control} {{ x: 16px + mod(index, 2) * 390px; y: 16px + floor(index / 2) * 40px; width: 320px; height: 32px; text: {json.dumps(value, ensure_ascii=False)}; }}'
    elif scene == 'hidden-toast' and kit:
        body = 'ToastHost { shown: false; message: "Saved"; }'
    elif scene == 'hidden-toast-composed':
        # Hidden-state cost reference, not a new std Toast or a visible-motion baseline.
        body = 'in-out property <bool> transient_shown: false;\n'
        if kit:
            body += 'ToastHost { width:360px; shown:root.transient_shown; auto_dismiss:false; message:"Saved"; }'
        else:
            info = (ROOT/'assets/icons/info-24-regular.svg').as_posix()
            close = (ROOT/'assets/icons/dismiss-16-regular.svg').as_posix()
            body += f'''Rectangle {{
                width:360px;height:root.transient_shown ? 56px : 0px;
                opacity:root.transient_shown ? 1 : 0;background:#ffffff;
                border-width:root.transient_shown ? 1px : 0px;border-color:#005fb8;border-radius:6px;clip:true;
                if root.transient_shown: Rectangle {{
                    Rectangle {{width:4px;height:100%;background:#005fb8;}}
                    TouchArea {{}}
                    HorizontalLayout {{
                        padding-left:16px;padding-right:8px;padding-top:8px;padding-bottom:8px;spacing:8px;
                        Image {{source:@image-url("{info}");width:24px;height:24px;colorize:#005fb8;}}
                        Text {{text:"Saved";horizontal-stretch:1;color:#202020;font-size:14px;vertical-alignment:center;wrap:word-wrap;max-height:40px;overflow:elide;}}
                        Button {{icon:@image-url("{close}");icon-size:20px;colorize-icon:true;accessible-label:"Dismiss";
                            clicked=>{{root.transient_shown=false;}}
                            Tooltip {{Rectangle {{background:#ffffff;Text {{text:"Dismiss";}}}}}}
                        }}
                    }}
                }}
            }}'''
    # import-only intentionally does not initialize or instantiate any Kit symbol.
    init = 'Palette.color-scheme = ColorScheme.light;'
    if kit and scene != 'import-only':
        init += ' Theme.mode = ThemeMode.light; Theme.system-dark = false; Theme.ui-font-family = "Segoe UI Variable Text";'
    # Large groups have 1,000 real objects, with 100 in the fixed viewport.
    # This measures allocation growth, not 1,000 simultaneously visible controls.
    return imports + f'''export component PerfWindow inherits Window {{
    width: 820px; height: 440px; background: #f3f3f3;
    default-font-family: "Segoe UI Variable Text"; default-font-size: 14px;
    init => {{ {init} }}
    {body}
}}
'''


def percentile(values, fraction):
    ordered = sorted(values)
    return ordered[max(0, math.ceil(len(ordered) * fraction) - 1)]


def resolved_fingerprint(metadata):
    nodes = {node['id']: node for node in metadata['resolve']['nodes']}
    packages = [p for p in metadata['packages'] if p['source'] is not None]
    versions = {p['version'] for p in packages if p['name'] in ('slint', 'slint-build')}
    if versions != {'1.17.1'}:
        raise ValueError(f'Unexpected resolved Slint versions: {versions}')
    rows = sorted((p['name'], p['version'], p['source'], sorted(nodes[p['id']]['features'])) for p in packages)
    return hashlib.sha256(json.dumps(rows, sort_keys=True).encode()).hexdigest()


def summarize(samples):
    output = {}
    for scene in sorted({s['scene'] for s in samples}):
        output[scene] = {}
        for variant in ('native', 'kit'):
            group = [s for s in samples if s['scene'] == scene and s['variant'] == variant]
            metrics = {}
            for key in ('construct_ms', 'first_render_callback_ms', 'first_software_frame_ms', 'show_return_ms', 'private_bytes', 'private_bytes_1s', 'memory_sample_delta_bytes', 'working_set_bytes', 'binary_bytes'):
                values = [s[key] for s in group if isinstance(s.get(key), (int, float))]
                metrics[key] = {'n': len(values), 'p50': percentile(values, .5), 'p95': percentile(values, .95), 'min': min(values), 'max': max(values)} if values else {'n': 0, 'status': 'NOT_RUN'}
            output[scene][variant] = metrics
    return output


def memory_sample(process):
    if os.name != 'nt':
        return {'memory_status': 'NOT_RUN: Windows sampler required'}
    class Counters(ctypes.Structure):
        _fields_ = [('cb', wintypes.DWORD), ('PageFaultCount', wintypes.DWORD)] + [(name, ctypes.c_size_t) for name in ('PeakWorkingSetSize', 'WorkingSetSize', 'QuotaPeakPagedPoolUsage', 'QuotaPagedPoolUsage', 'QuotaPeakNonPagedPoolUsage', 'QuotaNonPagedPoolUsage', 'PagefileUsage', 'PeakPagefileUsage', 'PrivateUsage')]
    counters = Counters()
    counters.cb = ctypes.sizeof(counters)
    api = ctypes.WinDLL('psapi', use_last_error=True).GetProcessMemoryInfo
    api.argtypes = [wintypes.HANDLE, ctypes.POINTER(Counters), wintypes.DWORD]
    api.restype = wintypes.BOOL
    if not api(wintypes.HANDLE(int(process._handle)), ctypes.byref(counters), counters.cb):
        raise ctypes.WinError(ctypes.get_last_error())
    return {'private_bytes': counters.PrivateUsage, 'working_set_bytes': counters.WorkingSetSize}


def cpu_seconds(process):
    """Return kernel and user CPU time for a Windows child process."""
    api = ctypes.WinDLL('kernel32', use_last_error=True).GetProcessTimes
    api.argtypes = [wintypes.HANDLE] + [ctypes.POINTER(wintypes.FILETIME)] * 4
    api.restype = wintypes.BOOL
    values = [wintypes.FILETIME() for _ in range(4)]
    if not api(wintypes.HANDLE(int(process._handle)), *(ctypes.byref(v) for v in values)):
        raise ctypes.WinError(ctypes.get_last_error())
    return sum((v.dwHighDateTime << 32) | v.dwLowDateTime for v in values[2:]) / 10_000_000


def observe_idle(command, cwd, log, env):
    """Observe CPU and memory between a host's explicit idle markers."""
    start = None
    report = None
    process = subprocess.Popen(command, cwd=cwd, env=env, stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT, text=True, encoding='utf-8')
    deadline = threading.Timer(180, process.kill)
    deadline.daemon = True
    deadline.start()
    try:
        with log.open('w', encoding='utf-8') as stream:
            for line in process.stdout:
                stream.write(line)
                stream.flush()
                if line.strip() == 'IDLE_BEGIN':
                    if start is not None:
                        raise ValueError('Duplicate idle start')
                    start = (time.monotonic(), cpu_seconds(process), memory_sample(process))
                elif line.startswith('FINAL '):
                    if start is None:
                        raise ValueError('Missing idle start')
                    elapsed = time.monotonic() - start[0]
                    cpu = cpu_seconds(process) - start[1]
                    report = dict(seconds=elapsed, cpu_seconds=cpu, one_core_percent=100 * cpu / elapsed,
                                  before=start[2], after=memory_sample(process))
        code = process.wait(timeout=15)
        # Pipe delivery has millisecond jitter; the host timer is explicitly 60s.
        if code or report is None or report['seconds'] < 59.99:
            raise ValueError('Incomplete idle observation')
        return report
    finally:
        deadline.cancel()
        if process.poll() is None:
            process.kill()
            process.wait()


def execute(command, cwd, log, env=None):
    with log.open('w', encoding='utf-8') as stream:
        result = subprocess.run(command, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT, timeout=1800)
    return {'command': command, 'cwd': str(cwd), 'exit_code': result.returncode, 'log': str(log)}


def generate(project, source, name, runtime):
    project.mkdir(parents=True)
    (project / 'src').mkdir()
    (project / 'probe.slint').write_text(source, encoding='utf-8')
    dependencies = '[dependencies]\nslint = { version = "=1.17.1", features = ["unstable-winit-030"] }\n' if runtime else ''
    (project / 'Cargo.toml').write_text(f'''[package]
name = "{name}"
version = "0.0.0"
edition = "2024"
rust-version = "1.92"
publish = false
[workspace]
{dependencies}
[build-dependencies]
slint-build = "=1.17.1"
quadrant-kit = {{ path = {json.dumps(ROOT.as_posix())} }}
''', encoding='utf-8')
    # Same reviewed workspace resolution seed for every generated project.
    shutil.copyfile(ROOT / 'Cargo.lock', project / 'Cargo.lock')
    (project / 'build.rs').write_text('''fn main() {
    let libraries = std::collections::HashMap::from([(
        quadrant_kit::SLINT_LIBRARY_NAME.to_owned(), quadrant_kit::slint_library_path())]);
    let config = slint_build::CompilerConfiguration::new().with_style("fluent".into())
        .with_library_paths(libraries).embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("probe.slint", config).expect("public capability probe");
}
''', encoding='utf-8')
    (project / 'src/main.rs').write_text((ROOT / 'scripts/perf_host.rs').read_text(encoding='utf-8') if runtime else 'fn main() {}\n', encoding='utf-8')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--scenes', nargs='+', choices=SCENES, default=list(SCENES))
    parser.add_argument('--samples', type=int, default=3, help='Paired new-process samples; >=30 recommended for later release gates')
    parser.add_argument('--profile', choices=['debug', 'release'], default='release')
    parser.add_argument('--native-probe', action='store_true', help='Compile positive probe and verify three expected public-API rejections only')
    parser.add_argument('--generate-only', action='store_true')
    parser.add_argument('--target-dir', type=Path, help='Exclusive shared build cache; generated consumers stay under target/')
    args = parser.parse_args(argv)
    if args.native_probe:
        args.profile = 'debug'
    if args.samples < 1:
        parser.error('--samples must be positive')
    stamp = datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    output = ROOT / 'target/perf-harness' / stamp
    output.mkdir(parents=True)
    report = {'schema_version': 2, 'source': source_identity(ROOT), 'os': platform.platform(), 'machine': platform.machine(), 'profile': args.profile, 'backend': 'winit-software', 'scale': 1, 'font': 'Segoe UI Variable Text', 'size': [820, 440], 'theme': 'light', 'commands': [], 'samples': [], 'status': 'IN_PROGRESS', 'outlier_policy': 'keep all; nearest-rank percentiles', 'timing_origin': 'Rust main entry (not process spawn/present); software-frame is synchronous snapshot after show, not present; memory sampled 1s/2s after Popen; harness markers at 1.5s/2.5s', 'budgets': 'NOT_EVALUATED: explicit report review required'}
    report['rustc'] = subprocess.check_output(['rustc', '-Vv'], cwd=ROOT, text=True)
    print(f'Report: {output / "result.json"}', flush=True)
    build_target = (args.target_dir or ROOT / 'target').resolve()
    env = dict(os.environ, CARGO_TARGET_DIR=str(build_target), SLINT_BACKEND='winit-software', SLINT_SCALE_FACTOR='1')
    for key in ('SLINT_STYLE', 'SLINT_DEFAULT_FONT', 'SLINT_FULLSCREEN', 'SLINT_DEBUG_PERFORMANCE'):
        env.pop(key, None)
    projects = []
    reference_graph = None
    try:
        cases = [('positive', (ROOT / 'gallery/ui/native_control_probe.slint').read_text(encoding='utf-8'), None)] + [(name, source, diagnostic) for name, (source, diagnostic) in NEGATIVE.items()] if args.native_probe else [(f'{scene}-{variant}', scene_source(scene, variant), None) for scene in args.scenes for variant in ('native', 'kit')]
        for case, source, diagnostic in cases:
            project = output / case
            name = 'kit-p0-' + case
            generate(project, source, name, not args.native_probe)
            projects.append((case, project, name))
            if args.generate_only:
                continue
            # Explicit offline reconciliation of the copied workspace lock, then locked builds.
            command = ['cargo', 'metadata', '--offline', '--format-version', '1']
            with (project / 'resolved.json').open('w', encoding='utf-8') as stdout, (project / 'resolve.log').open('w', encoding='utf-8') as stderr:
                resolved = subprocess.run(command, cwd=project, env=env, stdout=stdout, stderr=stderr, timeout=120)
            report['commands'].append({'command': command, 'exit_code': resolved.returncode, 'cwd': str(project), 'log': str(project / 'resolve.log'), 'graph': str(project / 'resolved.json')})
            if resolved.returncode:
                raise RuntimeError(f'{case}: metadata failed')
            fingerprint = resolved_fingerprint(json.loads((project / 'resolved.json').read_text(encoding='utf-8')))
            if reference_graph is not None and reference_graph != fingerprint:
                raise RuntimeError(f'{case}: external package versions/features differ from the reference')
            reference_graph = fingerprint
            report['resolved_external_graph_sha256'] = fingerprint
            command = ['cargo', 'build', '--locked', '--offline']
            if args.profile == 'release' and not args.native_probe:
                command.append('--release')
            built = execute(command, project, project / 'build.log', env)
            report['commands'].append(built)
            log = (project / 'build.log').read_text(encoding='utf-8')
            if diagnostic:
                if built['exit_code'] == 0 or diagnostic not in log:
                    raise RuntimeError(f'{case}: expected rejection missing ({diagnostic})')
            elif built['exit_code']:
                raise RuntimeError(f'{case}: build failed')
            if not args.native_probe:
                binary = build_target / args.profile / (name + ('.exe' if os.name == 'nt' else ''))
                shutil.copyfile(binary, project / binary.name)
        if not args.native_probe and not args.generate_only:
            for scene in args.scenes:
                for index in range(args.samples):
                    for variant in (('native', 'kit') if index % 2 == 0 else ('kit', 'native')):
                        project = output / f'{scene}-{variant}'
                        binary = project / ('kit-p0-' + scene + '-' + variant + ('.exe' if os.name == 'nt' else ''))
                        raw = project / f'sample-{index}.log'
                        sample = {'scene': scene, 'variant': variant, 'index': index, 'binary_bytes': binary.stat().st_size, 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest()}
                        with raw.open('w', encoding='utf-8') as stream:
                            process = subprocess.Popen([str(binary)], cwd=project, env=env, stdout=stream, stderr=subprocess.STDOUT)
                            try:
                                time.sleep(1)
                                if process.poll() is not None:
                                    raise RuntimeError(f'{scene}/{variant}: process exited before 1s sample')
                                early_memory = memory_sample(process)
                                if 'private_bytes' in early_memory:
                                    sample['private_bytes_1s'] = early_memory['private_bytes']
                                time.sleep(1)
                                if process.poll() is not None:
                                    raise RuntimeError(f'{scene}/{variant}: process exited before sample')
                                sample.update(memory_sample(process))
                                if 'private_bytes_1s' in sample:
                                    sample['memory_sample_delta_bytes'] = sample['private_bytes'] - sample['private_bytes_1s']
                                sample['exit_code'] = process.wait(timeout=15)
                            finally:
                                if process.poll() is None:
                                    process.kill()
                                    process.wait()
                        if sample['exit_code']:
                            raise RuntimeError(f'{scene}/{variant}: nonzero process exit')
                        for line in raw.read_text(encoding='utf-8').splitlines():
                            key, sep, value = line.partition('=')
                            if sep and key.endswith('_ms'):
                                sample[key] = float(value)
                            elif key == 'render_hook_supported':
                                sample[key] = value == 'true'
                        if 'settled_marker_ms' not in sample:
                            raise RuntimeError('Missing timing marker')
                        report['samples'].append(sample)
                        print(f'{scene}/{variant} sample {index + 1}/{args.samples}', flush=True)
        if source_identity(ROOT) != report['source']:
            raise RuntimeError('Source changed during run; results are not a stable-source baseline')
        report['status'] = 'GENERATED_ONLY' if args.generate_only else 'PASS'
        return 0
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        report['status'] = 'FAIL'
        report['error'] = str(error)
        print(str(error), file=sys.stderr)
        return 1
    finally:
        report['summary'] = summarize(report['samples'])
        (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')


if __name__ == '__main__':
    sys.exit(main())
