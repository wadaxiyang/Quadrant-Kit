#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Inspect saved Windows benchmark binaries and actual client geometry."""
import argparse
import ctypes
from ctypes import wintypes
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import time

from run_perf import ROOT


def sections(data):
    if data[:2] != b'MZ':
        raise ValueError('Expected Windows PE')
    pe = struct.unpack_from('<I', data, 60)[0]
    if data[pe:pe+4] != b'PE\0\0':
        raise ValueError('Invalid PE signature')
    count = struct.unpack_from('<H', data, pe+6)[0]
    optional = struct.unpack_from('<H', data, pe+20)[0]
    start = pe+24+optional
    return {data[start+i*40:start+i*40+8].rstrip(b'\0').decode('ascii'):
            struct.unpack_from('<I', data, start+i*40+16)[0] for i in range(count)}


def client_sizes(pid):
    user = ctypes.WinDLL('user32', use_last_error=True)
    callback_type = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)
    user.EnumWindows.argtypes = [callback_type, wintypes.LPARAM]
    user.GetWindowThreadProcessId.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.DWORD)]
    user.IsWindowVisible.argtypes = [wintypes.HWND]
    user.GetClientRect.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.RECT)]
    sizes=[]
    @callback_type
    def visit(hwnd, _):
        owner=wintypes.DWORD()
        user.GetWindowThreadProcessId(hwnd,ctypes.byref(owner))
        if owner.value==pid and user.IsWindowVisible(hwnd):
            rect=wintypes.RECT()
            if not user.GetClientRect(hwnd,ctypes.byref(rect)):
                return True
            sizes.append([rect.right-rect.left,rect.bottom-rect.top])
        return True
    user.EnumWindows(visit,0)
    return sizes


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('report',type=Path)
    args=parser.parse_args()
    measured=json.loads(args.report.read_text(encoding='utf-8'))
    if measured['status']!='PASS':raise ValueError('Completed collection required')
    output={'source':measured['source'],'status':'PASS','artifacts':[],
            'scope':'one extra launch per saved binary; PE raw section sizes and byte-identical embedded local SVGs'}
    env=dict(os.environ,SLINT_BACKEND='winit-software',SLINT_SCALE_FACTOR='1')
    for key in ('SLINT_STYLE','SLINT_DEFAULT_FONT','SLINT_FULLSCREEN','SLINT_DEBUG_PERFORMANCE'):
        env.pop(key,None)
    for scene,variant in sorted({(s['scene'],s['variant']) for s in measured['samples']}):
        project=args.report.parent/f'{scene}-{variant}'
        binary=project/f'kit-p0-{scene}-{variant}.exe'
        data=binary.read_bytes()
        digest=hashlib.sha256(data).hexdigest()
        if any(s['binary_sha256']!=digest for s in measured['samples'] if s['scene']==scene and s['variant']==variant):
            raise ValueError('Saved binary differs from measured binary')
        embedded=[dict(name=p.name,bytes=p.stat().st_size) for p in (ROOT/'assets/icons').glob('*.svg') if p.read_bytes() in data]
        with (project/'geometry.log').open('w',encoding='utf-8') as log:
            process=subprocess.Popen([str(binary.resolve())],cwd=project,env=env,stdout=log,stderr=subprocess.STDOUT)
            try:
                sizes=[]
                for _ in range(15):
                    time.sleep(.1)
                    sizes=client_sizes(process.pid)
                    if sizes:break
                # Winit also owns a zero-area helper HWND in both variants.
                # Retain it in raw data; compare the actual drawable surfaces.
                drawable = [size for size in sizes if size[0] > 0 and size[1] > 0]
                if drawable!=[[820,440]]:output['status']='FAIL'
                output['artifacts'].append(dict(scene=scene,variant=variant,sha256=digest,
                    binary_bytes=len(data),sections=sections(data),embedded_kit_svg=embedded,client_sizes=sizes,drawable_client_sizes=drawable))
            finally:
                if process.poll() is None:
                    process.kill()
                process.wait()
    (args.report.parent/'artifact_review.json').write_text(json.dumps(output,indent=2)+'\n',encoding='utf-8')
    print(output['status'])
    return int(output['status']!='PASS')


if __name__=='__main__':
    raise SystemExit(main())
