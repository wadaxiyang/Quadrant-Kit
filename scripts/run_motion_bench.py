#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Paired before/after software-buffer frames and 60s idle, not actual presentation."""
import argparse,json,os,subprocess,sys,re,statistics,math,shutil
from datetime import datetime,timezone
from pathlib import Path
from run_perf import ROOT,generate,execute,resolved_fingerprint,source_identity

def parse_measurement(text, count):
    raw = [line for line in text.splitlines() if line.startswith('FRAME ')]
    pattern = r'FRAME count=(\d+) sample=(\d+) ms=([0-9.]+) pixel=(\d+)'
    if len(raw) != 200:
        raise ValueError('Expected all 200 frame samples')
    frames = []
    for index, line in enumerate(raw):
        match = re.fullmatch(pattern, line)
        if not match or int(match[1]) != count or int(match[2]) != index:
            raise ValueError('Mismatched count, missing or duplicate frame index')
        value = float(match[3])
        if not math.isfinite(value) or value < 0:
            raise ValueError('Invalid frame duration')
        frames.append(value)
    idle_lines = [line for line in text.splitlines() if line.startswith('IDLE ')]
    if len(idle_lines) != 1:
        raise ValueError('Expected one complete idle interval')
    idle = re.fullmatch(r'IDLE count=(\d+) seconds=([0-9.]+) cpu_seconds=([0-9.]+) one_core_percent=([0-9.]+) render_callbacks=(\d+) hook_supported=(true|false)', idle_lines[0])
    if not idle or int(idle[1]) != count or float(idle[2]) < 60:
        raise ValueError('Incomplete or mismatched idle measurement')
    seconds, cpu, percent = (float(idle[i]) for i in (2, 3, 4))
    if not all(math.isfinite(v) for v in (seconds, cpu, percent)) or abs(percent - 100 * cpu / seconds) > .00001:
        raise ValueError('Inconsistent CPU accounting')
    return {'count': count, 'frames': frames, 'p50_ms': statistics.median(frames),
            'p95_ms': sorted(frames)[math.ceil(.95 * len(frames)) - 1], 'max_ms': max(frames),
            'idle_seconds': seconds, 'idle_cpu_seconds': cpu, 'idle_one_core_percent': percent,
            'render_hook_supported': idle[6] == 'true',
            'idle_render_callbacks': int(idle[5]) if idle[6] == 'true' else None}

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--label',required=True);args=p.parse_args()
    output=ROOT/'target/motion-bench'/datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ');project=output/'consumer'
    source=source_identity(ROOT);report={'label':args.label,'source':source,'status':'IN_PROGRESS','runs':[]}
    generate(project,(ROOT/'scripts/motion_bench.slint').read_text(encoding='utf-8'),'kit-motion-bench',True)
    shutil.copyfile(ROOT/'scripts/motion_bench_host.rs',project/'src/main.rs')
    env=dict(os.environ,CARGO_TARGET_DIR=str(ROOT/'target'),SLINT_BACKEND='winit-software',SLINT_SCALE_FACTOR='1')
    for name in ('SLINT_STYLE','SLINT_DEFAULT_FONT','SLINT_FULLSCREEN','SLINT_DEBUG_PERFORMANCE'):env.pop(name,None)
    print('Report:',output/'result.json',flush=True)
    try:
        data=subprocess.check_output(['cargo','metadata','--offline','--format-version','1'],cwd=project,env=env,text=True,encoding='utf-8')
        (output/'resolved.json').write_text(data,encoding='utf-8');report['external_graph_sha256']=resolved_fingerprint(json.loads(data))
        report['build']=execute(['cargo','build','--locked','--offline','--release'],project,output/'build.log',env)
        if report['build']['exit_code']:raise RuntimeError('build failed')
        binary=output/'kit-motion-bench.exe';shutil.copyfile(ROOT/'target/release/kit-motion-bench.exe',binary);report['binary_bytes']=binary.stat().st_size
        for count in (1,20):
            log=output/f'count-{count}.log'
            with log.open('w',encoding='utf-8') as stream:result=subprocess.run([str(binary),str(count)],env=env,stdout=stream,stderr=subprocess.STDOUT,timeout=90)
            text=log.read_text(encoding='utf-8')
            if result.returncode or 'RESULT=PASS' not in text:raise RuntimeError('incomplete measurement')
            run=parse_measurement(text,count)
            run.update(exit_code=result.returncode,log=str(log))
            report['runs'].append(run);print({k:v for k,v in run.items() if k!='frames'},flush=True)
        if source!=source_identity(ROOT):raise RuntimeError('sources changed')
        report['status']='PASS'
    except (RuntimeError,subprocess.SubprocessError,OSError,ValueError) as error:report.update(status='FAIL',error=str(error))
    (output/'result.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8');print(report['status'],flush=True);return int(report['status']!='PASS')
if __name__=='__main__':sys.exit(main())
