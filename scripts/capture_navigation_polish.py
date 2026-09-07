#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Phase 7: full-viewport NavigationView plus real Gallery simulated-DPI captures."""
import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform
import subprocess
from capture_gallery_baseline import ROOT, digest, png_info, scene_environment, source_identity


def cells():
    result = []
    for width, height in [(1040, 800), (760, 520)]:
        for theme in ['light', 'dark']:
            for scale in [100, 125, 150, 200, 225]:
                result.append(dict(host='gallery', width=width, height=height, theme=theme, scale=scale, compact=False, variant=0, case=0))
            for compact in [False, True]:
                for variant in range(8):
                    for scale in ([100, 125, 150, 200, 225] if variant == 0 else [100, 225]):
                        result.append(dict(host='navigation-validation', width=width, height=height, theme=theme, scale=scale, compact=compact, variant=variant, case=0))
    for theme in ['light', 'dark']:
        for compact in [False, True]:
            for case in [1, 2, 16]:
                result.append(dict(host='navigation-validation', width=1040, height=800, theme=theme, scale=100, compact=compact, variant=0, case=case))
    return result


def key(cell):
    return '{host}_{width}x{height}_{theme}_dpi-{scale}_compact-{compact}_variant-{variant}_case-{case}'.format(**cell)


def environment(cell, output):
    env = scene_environment(cell['width'], cell['height'], cell['theme'], 'home', 1, output,
                            cell['case'], cell['compact'], 'winit-software', cell['scale'])
    if cell['host'] == 'navigation-validation':
        env.pop('QUADRANT_GALLERY_DESTINATION')
        env['QUADRANT_GALLERY_NAV_VALIDATION'] = str(cell['variant'])
    return env


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output-directory', type=Path, default=ROOT/'target/navigation-phase7/render')
    parser.add_argument('--smoke', action='store_true', help='Only default full-viewport NavigationView; not the full gate')
    args = parser.parse_args()
    before = source_identity(ROOT)
    subprocess.run(['cargo', 'build', '--locked', '-p', 'quadrant-kit-gallery'], cwd=ROOT, check=True)
    if source_identity(ROOT) != before:
        raise ValueError('Sources changed while building')
    binary = ROOT/'target/debug'/('quadrant-kit-gallery.exe' if os.name == 'nt' else 'quadrant-kit-gallery')
    binary_hash = digest(binary.read_bytes())
    output = args.output_directory.resolve()/('dirty-'+before['content_sha256'] if before['dirty'] else before['source_full_sha'])
    output.mkdir(parents=True, exist_ok=True)
    scenarios = [next(c for c in cells() if c['host'] == 'navigation-validation')] if args.smoke else cells()
    records = []
    for cell in scenarios:
        png = output/(key(cell)+'.png')
        fresh = output/(key(cell)+'.capturing.png')
        fresh.unlink(missing_ok=True)
        subprocess.run([str(binary)], env=environment(cell, fresh), cwd=ROOT, check=True, timeout=30)
        if source_identity(ROOT) != before or digest(binary.read_bytes()) != binary_hash:
            raise ValueError('Source or executable changed during capture')
        info = png_info(fresh)
        expected = (round(cell['width']*cell['scale']/100), round(cell['height']*cell['scale']/100))
        if (info['pixel_width'], info['pixel_height']) != expected:
            raise ValueError('Unexpected dimensions for '+key(cell))
        fresh.replace(png)
        record = dict(schema_version=1, suite='navigation-polish-v1', source=before, binary_sha256=binary_hash,
                      os=platform.platform(), slint_version='1.17.1', backend='winit-software', simulated_dpi=True,
                      scene=cell, image=info, generated_at_utc=datetime.now(timezone.utc).isoformat())
        png.with_suffix('.json').write_text(json.dumps(record, indent=2)+'\n', encoding='utf-8')
        records.append(key(cell))
        print('Captured '+key(cell), flush=True)
    (output/'results.json').write_text(json.dumps(dict(result='PASS', suite='navigation-polish-v1', smoke=args.smoke,
        count=len(records), scenes=records, source=before, binary_sha256=binary_hash), indent=2)+'\n', encoding='utf-8')
    print(f'{len(records)} scenes PASS; simulated DPI, no real monitor-transition claim', flush=True)


if __name__ == '__main__':
    main()
