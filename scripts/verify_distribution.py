#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Check this source package's static import/image closure and asset hashes.

This is a scoped distribution check, not the Phase 2 Slint/API boundary parser.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
IMPORT = re.compile(r'(?:import|export)\s*\{[^}]*\}\s*from\s*"([^"\n]+)"', re.S)
IMAGE = re.compile(r'@image-url\(\s*"([^"\n]+)"\s*\)')


def static_closure(root=ROOT):
    root = root.resolve()
    required = set()
    pending = [root / 'ui/kit.slint']
    while pending:
        path = pending.pop().resolve()
        if not path.is_relative_to(root) or not path.is_file():
            raise ValueError(f'Missing or out-of-package dependency: {path}')
        if path in required:
            continue
        required.add(path)
        if path.suffix != '.slint':
            continue
        text = path.read_text(encoding='utf-8')
        for source in IMPORT.findall(text):
            if source == 'std-widgets.slint':
                continue
            if source.startswith('@') or ':' in source or source.startswith(('/', '\\')):
                raise ValueError(f'Invalid Kit implementation import in {path}: {source}')
            pending.append(path.parent / source)
        for source in IMAGE.findall(text):
            if ':' in source or source.startswith(('/', '\\')):
                raise ValueError(f'Invalid static resource in {path}: {source}')
            pending.append(path.parent / source)
    return {p.relative_to(root).as_posix() for p in required}


def verify(root=ROOT, package=False):
    root = root.resolve()
    required = static_closure(root)
    manifest = json.loads((root / 'scripts/asset_manifest.json').read_text(encoding='utf-8'))
    assets = {entry['new_path']: entry for entry in manifest['assets']}
    if not assets:
        raise ValueError('Asset manifest is empty')
    for path, entry in assets.items():
        actual = (root / path).resolve()
        if not actual.is_relative_to(root) or not actual.is_file():
            raise ValueError(f'Invalid asset path: {path}')
        if hashlib.sha256(actual.read_bytes()).hexdigest() != entry['sha256']:
            raise ValueError(f'Asset hash mismatch: {path}')
        if entry['spdx_license'] != 'MIT':
            raise ValueError(f'Unexpected asset license: {path}')
    if {p for p in required if p.endswith('.svg')} != set(assets):
        raise ValueError('Static SVG closure and asset manifest differ')
    required.update(['assets/icons/LICENSE-MIT', 'LICENSE', 'THIRD-PARTY-NOTICES.md',
                     'scripts/asset_manifest.json', 'docs/PROVENANCE.md'])
    if package:
        output = subprocess.check_output(['cargo', 'package', '--locked', '-p', 'quadrant-kit', '--list'], cwd=root, text=True, encoding='utf-8')
        listed = {line.replace('\\', '/') for line in output.splitlines()}
        missing = required - listed
        if missing:
            raise ValueError(f'Package omits dependencies: {sorted(missing)}')
        if any(p.startswith(('gallery/', 'target/', '.vscode/')) for p in listed):
            raise ValueError('Package contains excluded development content')
    return {'static_files': len(required), 'svg_assets': len(assets), 'package_checked': package}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--package', action='store_true', help='also compare cargo package --list')
    args = parser.parse_args()
    try:
        print(json.dumps(verify(package=args.package), indent=2))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f'Distribution check failed: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
