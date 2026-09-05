#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Verify deep token/SVG changes invalidate the same Gallery build.

Run with exclusive access to this checkout and target directory. Each experiment
restores exact original bytes in finally, then rebuilds the restored source.
No GUI/display server is needed. This measures build/resource invalidation,
not pixel equivalence or native interaction.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify(root=ROOT, target_dir=None):
    command = ['cargo', 'build', '--locked', '-p', 'quadrant-kit-gallery']
    if target_dir:
        command += ['--target-dir', str(target_dir)]
    metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--no-deps', '--format-version', '1'], cwd=root, text=True, encoding='utf-8'))
    target = Path(target_dir or metadata['target_directory']).resolve()
    binary = target / 'debug' / ('quadrant-kit-gallery.exe' if os.name == 'nt' else 'quadrant-kit-gallery')
    evidence = target / 'incremental-verification'
    evidence.mkdir(parents=True, exist_ok=True)

    def build(label):
        with (evidence / f'{label}.log').open('w', encoding='utf-8') as log:
            subprocess.run(command + ['-vv'], cwd=root, stdout=log, stderr=subprocess.STDOUT, check=True)
        outputs = {str(p.relative_to(target)): p.stat().st_mtime_ns for p in (target / 'debug/build').glob('quadrant-kit-gallery-*/output')}
        if not outputs:
            raise ValueError('Gallery build script output missing')
        return {'binary_sha256': digest(binary), 'build_outputs': outputs}

    scenarios = [('token', root / 'ui/foundation/theme.slint', b'#005fb8', b'#005fb9'),
                 ('svg', root / 'assets/icons/navigation-20-regular.svg', b'<svg ', b'<svg opacity="0.99" ')]
    result = []
    original_files = {path: path.read_bytes() for _, path, _, _ in scenarios}
    for name, path, old, new in scenarios:
        original = original_files[path]
        if old not in original:
            raise ValueError(f'Incremental fixture no longer matches {path}')
        before = build(f'{name}-before')
        try:
            path.write_bytes(original.replace(old, new, 1))
            after = build(f'{name}-changed')
            if before['build_outputs'] == after['build_outputs']:
                raise ValueError(f'{name} did not rerun the Gallery build script')
            if before['binary_sha256'] == after['binary_sha256']:
                raise ValueError(f'{name} did not change the built Gallery binary')
            result.append({'scenario': name, 'source': str(path.relative_to(root)), 'before': before, 'changed': after})
        finally:
            path.write_bytes(original)
            build(f'{name}-restored')
        if path.read_bytes() != original:
            raise ValueError(f'Failed to restore {path}')
    report = {'status': 'PASS', 'target': str(target), 'scenarios': result, 'source_bytes_restored': True}
    (evidence / 'result.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
    return report


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target-dir', type=Path)
    args = parser.parse_args()
    try:
        print(json.dumps(verify(target_dir=args.target_dir), indent=2))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f'Incremental verification failed: {error}', file=sys.stderr)
        sys.exit(1)
