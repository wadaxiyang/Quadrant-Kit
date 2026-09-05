#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Check this source package's static import/image closure and asset hashes.

Uses the same lexical import/resource scanner as the boundary guard.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tarfile
from slint_contract import images, local_path, parse

ROOT = Path(__file__).resolve().parents[1]


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
        for source, _ in parse(text)['imports']:
            if source == 'std-widgets.slint':
                continue
            if source.startswith('@') or ':' in source or source.startswith(('/', '\\')):
                raise ValueError(f'Invalid Kit implementation import in {path}: {source}')
            pending.append(local_path(path, source, root))
        for source in images(text):
            if ':' in source or source.startswith(('/', '\\')):
                raise ValueError(f'Invalid static resource in {path}: {source}')
            pending.append(local_path(path, source, root))
    return {p.relative_to(root).as_posix() for p in required}


def verify(root=ROOT, package=False):
    root = root.resolve()
    required = static_closure(root)
    manifest = json.loads((root / 'scripts/asset_manifest.json').read_text(encoding='utf-8'))
    assets = {entry['new_path']: entry for entry in manifest['assets']}
    if len(assets) != len(manifest['assets']):
        raise ValueError('Duplicate asset manifest path')
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
        if b'SPDX-License-Identifier: GPL' in actual.read_bytes():
            raise ValueError(f'MIT asset incorrectly relabeled GPL: {path}')
    if {p for p in required if p.endswith('.svg')} != set(assets):
        raise ValueError('Static SVG closure and asset manifest differ')
    actual_assets = {p.relative_to(root).as_posix() for p in (root / 'assets/icons').rglob('*') if p.is_file() and p.name != 'LICENSE-MIT'}
    if actual_assets != set(assets):
        raise ValueError('Unrecorded or missing asset file')
    required.update(['assets/icons/LICENSE-MIT', 'LICENSE', 'THIRD-PARTY-NOTICES.md',
                     'scripts/asset_manifest.json', 'docs/PROVENANCE.md'])
    for name in required:
        if not (root / name).is_file():
            raise ValueError(f'Missing required distribution file: {name}')
    for name, markers in {
        'assets/icons/LICENSE-MIT': ('MIT License', 'Microsoft Corporation', 'Permission is hereby granted'),
        'LICENSE': ('GNU GENERAL PUBLIC LICENSE', 'Version 3, 29 June 2007'),
    }.items():
        text = (root / name).read_text(encoding='utf-8')
        if not all(marker in text for marker in markers):
            raise ValueError(f'Missing expected license text: {name}')
    if package:
        output = subprocess.check_output(['cargo', 'package', '--locked', '-p', 'quadrant-kit', '--list'], cwd=root, text=True, encoding='utf-8')
        listed = {line.replace('\\', '/') for line in output.splitlines()}
        missing = required - listed
        if missing:
            raise ValueError(f'Package omits dependencies: {sorted(missing)}')
        if any(p.startswith(('gallery/', 'target/', '.vscode/')) for p in listed):
            raise ValueError('Package contains excluded development content')
        tracked = set(subprocess.check_output(['git', 'ls-files'], cwd=root, text=True, encoding='utf-8').splitlines())
        if required - tracked:
            raise ValueError(f'Static closure not tracked by Git: {sorted(required - tracked)}')
    return {'static_files': len(required), 'svg_assets': len(assets), 'package_checked': package}


def verify_archive(archive, root=ROOT):
    """Compare shipped bytes without extracting an untrusted archive."""
    required = static_closure(root) | {'assets/icons/LICENSE-MIT', 'LICENSE',
               'THIRD-PARTY-NOTICES.md', 'scripts/asset_manifest.json', 'docs/PROVENANCE.md'}
    with tarfile.open(archive, 'r:gz') as package:
        files = {}
        for member in package.getmembers():
            parts = Path(member.name).parts
            if member.issym() or member.islnk() or '..' in parts or member.name.startswith('/'):
                raise ValueError('Unsafe package archive entry')
            if member.isfile():
                relative = '/'.join(parts[1:])
                if relative in files:
                    raise ValueError('Duplicate archive entry')
                files[relative] = member
        for path in required:
            if path not in files or package.extractfile(files[path]).read() != (root / path).read_bytes():
                raise ValueError(f'Missing/changed archived dependency: {path}')
    return {'archive_checked': str(archive), 'verified_files': len(required)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--package', action='store_true', help='also compare cargo package --list')
    parser.add_argument('--archive', type=Path, help='verify an already built .crate archive byte for byte')
    args = parser.parse_args()
    try:
        print(json.dumps(verify(package=args.package), indent=2))
        if args.archive:
            print(json.dumps(verify_archive(args.archive), indent=2))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f'Distribution check failed: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
