#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Build Gallery once, then capture isolated scenes with content-checked reuse."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import struct
import subprocess
import sys

from gallery_catalog import load_catalog, resolve_destination

SCHEMA_VERSION = 2

ROOT = Path(__file__).resolve().parents[1]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def source_identity(root):
    head = subprocess.run(['git', 'rev-parse', '--verify', 'HEAD'], cwd=root, capture_output=True, text=True)
    sha = head.stdout.strip() if head.returncode == 0 else None
    status = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all'], cwd=root)
    names = subprocess.check_output(['git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard'], cwd=root).split(b'\0')
    hasher = hashlib.sha256()
    for raw in sorted(set(filter(None, names))):
        path = root / os.fsdecode(raw)
        hasher.update(raw + b'\0')
        hasher.update(path.read_bytes() if path.is_file() else b'<deleted>')
        hasher.update(b'\0')
    return {'source_full_sha': sha, 'dirty': bool(status) or sha is None, 'content_sha256': hasher.hexdigest()}


def png_info(path):
    data = path.read_bytes()
    if len(data) < 24 or data[:8] != b'\x89PNG\r\n\x1a\n':
        raise ValueError(f'Invalid PNG: {path}')
    width, height = struct.unpack('>II', data[16:24])
    if not width or not height:
        raise ValueError(f'Empty PNG dimensions: {path}')
    return {'pixel_width': width, 'pixel_height': height, 'image_sha256': digest(data)}


def reusable(record, scene, path):
    if not record or record.get('schema_version') != SCHEMA_VERSION or record.get('scene') != scene or not path.is_file():
        return False
    try:
        return record.get('image') == png_info(path)
    except (OSError, ValueError):
        return False


def navigation_cells():
    """Phase 2 render coverage; not the later DPI or keyboard acceptance matrix."""
    return [(1040, 800, theme, 100, case, compact)
            for theme in ['light', 'dark'] for compact in [False, True] for case in range(17)]


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode', choices=['Smoke', 'Matrix', 'All', 'Navigation', 'Catalog'], default='Smoke')
    routes = parser.add_mutually_exclusive_group()
    routes.add_argument('--page', type=int, choices=range(8))
    routes.add_argument('--destination')
    parser.add_argument('--preview', type=int, choices=range(3), default=1)
    parser.add_argument('--output-directory', type=Path, default=ROOT/'target/visual-baselines')
    parser.add_argument('--reuse-existing', action='store_true')
    # Snapshot-only selection; no change to ordinary Gallery runtime defaults.
    parser.add_argument('--backend', choices=['winit-software', 'winit-skia', 'winit-femtovg'], default='winit-software')
    args = parser.parse_args(argv)
    try:
        args.resolved_destination = resolve_destination(args.page, args.destination)
    except ValueError as error:
        parser.error(str(error))
    if args.mode == 'Navigation' and args.resolved_destination != 'navigation-view':
        parser.error('Navigation capture requires --destination navigation-view or --page 7')
    if args.mode == 'Catalog' and (args.page is not None or args.destination is not None):
        parser.error('Catalog captures every destination; omit route options')
    return args


def scene_environment(width, height, theme, destination, preview, fresh, case, compact, backend, scale):
    # CLI route selection is authoritative; inherited Gallery options cannot add ambiguity.
    env = {key:value for key,value in os.environ.items() if not key.startswith('QUADRANT_GALLERY_')}
    env.update(QUADRANT_GALLERY_WIDTH=str(width),QUADRANT_GALLERY_HEIGHT=str(height),
               QUADRANT_GALLERY_THEME=theme,QUADRANT_GALLERY_DESTINATION=destination,
               QUADRANT_GALLERY_PREVIEW=str(preview),QUADRANT_GALLERY_SNAPSHOT=str(fresh),
               QUADRANT_GALLERY_NAV_CASE=str(case),QUADRANT_GALLERY_NAV_COMPACT=str(int(compact)),
               SLINT_BACKEND=backend,SLINT_SCALE_FACTOR=str(scale/100))
    return env


def main():
    args = parse_args()
    before = source_identity(ROOT)
    subprocess.run(['cargo', 'build', '--locked', '-p', 'quadrant-kit-gallery'], cwd=ROOT, check=True)
    if before != source_identity(ROOT):
        raise ValueError('Sources changed during build; rerun capture from a stable checkout')
    metadata = json.loads(subprocess.check_output(['cargo', 'metadata', '--locked', '--format-version', '1'], cwd=ROOT, text=True, encoding='utf-8'))
    slint_versions = {p['version'] for p in metadata['packages'] if p['name'] in ['slint', 'slint-build']}
    if slint_versions != {'1.17.1'}:
        raise ValueError(f'Unexpected Slint versions: {slint_versions}')
    binary = Path(metadata['target_directory'])/'debug'/('quadrant-kit-gallery.exe' if os.name=='nt' else 'quadrant-kit-gallery')
    source_key = before['source_full_sha'] if not before['dirty'] else 'dirty-'+before['content_sha256']
    output = args.output_directory.resolve()/source_key/(platform.system().lower()+'-'+args.backend)
    output.mkdir(parents=True, exist_ok=True)
    common = {**before, 'os': platform.platform(), 'architecture': platform.machine(),
              'backend': 'winit', 'renderer': args.backend.removeprefix('winit-'),
              'font_policy': 'Segoe UI Variable Text' if os.name=='nt' else 'Slint system default',
              'slint_version': '1.17.1', 'binary_sha256': digest(binary.read_bytes()),
              'scenario_schema': SCHEMA_VERSION, 'preview': args.preview}
    cells = [(1040,800,'light',100)] if args.mode in ['Smoke','All'] else []
    if args.mode in ['Matrix','All']:
        cells += [(w,h,t,s) for w,h in [(760,520),(900,600),(1100,720),(1440,900)] for t in ['light','dark'] for s in [100,125,150,200,225]]
    cells = navigation_cells() if args.mode == 'Navigation' else [(*cell, 0, False) for cell in cells]
    destinations = [row['id'] for row in load_catalog() if row['component']] if args.mode == 'Catalog' else [args.resolved_destination]
    if args.mode == 'Catalog':
        cells = [(w,h,theme,100,0,False) for w,h in [(1040,800),(760,520)] for theme in ['light','dark']]
    scenarios = [(destination,*cell) for destination in destinations for cell in cells]
    for destination,width,height,theme,scale,case,compact in scenarios:
        scene = {**common, 'destination':destination, 'logical_width':width, 'logical_height':height, 'theme':theme, 'scale_percent':scale,
                 'navigation_case':case, 'navigation_compact':compact}
        key = f'destination-{destination}_preview-{args.preview}_{theme}_{width}x{height}_scale-{scale}'
        if args.mode == 'Navigation':
            key += f'_nav-{case:02}_compact-{int(compact)}'
        png = output/(key+'.png')
        manifest = output/(key+'.json')
        old = None
        if manifest.is_file():
            try: old = json.loads(manifest.read_text(encoding='utf-8'))
            except (ValueError, OSError): pass
        if args.reuse_existing and reusable(old,scene,png):
            print('Reusing '+key, flush=True)
            continue
        # Always capture to a new file; a successful exit cannot validate a stale PNG.
        fresh = output/(key+'.capturing.png')
        fresh.unlink(missing_ok=True)
        env = scene_environment(width,height,theme,destination,args.preview,fresh,case,compact,args.backend,scale)
        if args.page is not None:
            # Exercise the legacy host alias when explicitly requested.
            env.pop('QUADRANT_GALLERY_DESTINATION')
            env['QUADRANT_GALLERY_PAGE'] = str(args.page)
        # Timeout kills only this subprocess; no global process-name cleanup.
        subprocess.run([str(binary)], cwd=output, env=env, check=True, timeout=30)
        if before != source_identity(ROOT):
            raise ValueError('Sources changed during capture; discard this scene and rerun')
        info = png_info(fresh)
        fresh.replace(png)
        record = {'schema_version':SCHEMA_VERSION,'scene':scene,'image':info,'generated_at_utc':datetime.now(timezone.utc).isoformat()}
        manifest.write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
        print('Captured '+str(png), flush=True)
    return 0


if __name__ == '__main__':
    try: sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f'Capture failed: {error}', file=sys.stderr)
        sys.exit(1)
