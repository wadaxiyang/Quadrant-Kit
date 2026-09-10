#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Matched 60.1-second Windows idle and long-interval memory observations."""
from datetime import datetime, timezone
import json
import os
import shutil
import subprocess

from observe_idle import observe
from run_perf import ROOT, execute, generate, scene_source, source_identity, resolved_fingerprint

SCENES = ('empty', 'buttons-1000', 'selection-1000', 'text-group', 'lists-10000')
HOST = '''// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::ComponentHandle;
use std::time::Duration;
slint::include_modules!();
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = PerfWindow::new()?;
    ui.show()?;
    ui.window().take_snapshot()?;
    slint::Timer::single_shot(Duration::from_secs(2), || {
        println!("IDLE_BEGIN");
        slint::Timer::single_shot(Duration::from_millis(60_100), || {
            println!("FINAL idle_completed=true");
            println!("RESULT=PASS");
            slint::Timer::single_shot(Duration::from_millis(200), || slint::quit_event_loop().unwrap());
        });
    });
    slint::run_event_loop()?;
    Ok(())
}
'''


def main():
    output=ROOT/'target/idle-perf'/datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    output.mkdir(parents=True)
    report=dict(source=source_identity(ROOT),status='IN_PROGRESS',runs=[],
                scope='one independent pair per representative scene; 60.1s idle, one-core CPU, fixed interval memory; not 30-pair long-idle confidence')
    env=dict(os.environ,CARGO_TARGET_DIR=str(ROOT/'target'),SLINT_BACKEND='winit-software',SLINT_SCALE_FACTOR='1')
    for key in ('SLINT_STYLE','SLINT_DEFAULT_FONT','SLINT_FULLSCREEN','SLINT_DEBUG_PERFORMANCE'):
        env.pop(key,None)
    print('Report:',output/'result.json',flush=True)
    try:
        graph=None
        for scene in SCENES:
            for variant in ('native','kit'):
                project=output/f'{scene}-{variant}'
                name=f'idle-{scene}-{variant}'
                generate(project,scene_source(scene,variant),name,True)
                (project/'src/main.rs').write_text(HOST,encoding='utf-8')
                metadata=subprocess.check_output(['cargo','metadata','--offline','--format-version','1'],cwd=project,env=env,text=True,encoding='utf-8')
                (project/'resolved.json').write_text(metadata,encoding='utf-8')
                fingerprint=resolved_fingerprint(json.loads(metadata))
                if graph is not None and graph!=fingerprint:raise ValueError('Resolved graph differs')
                graph=fingerprint
                result=execute(['cargo','build','--locked','--offline','--release'],project,project/'build.log',env)
                if result['exit_code']:raise RuntimeError('Idle build failed')
                shutil.copyfile(ROOT/f'target/release/{name}.exe',project/'bench.exe')
        report['external_graph_sha256']=graph
        for index,scene in enumerate(SCENES):
            for variant in (('native','kit') if index%2==0 else ('kit','native')):
                project=output/f'{scene}-{variant}'
                idle=observe([str(project/'bench.exe')],project,project/'runtime.log',env)
                if 'RESULT=PASS' not in (project/'runtime.log').read_text(encoding='utf-8'):
                    raise RuntimeError('Missing completion')
                report['runs'].append(dict(scene=scene,variant=variant,idle=idle))
                print(scene,variant,idle,flush=True)
        report['gates']={}
        for scene in SCENES:
            a,b=(next(r['idle'] for r in report['runs'] if r['scene']==scene and r['variant']==v) for v in ('native','kit'))
            delta=b['one_core_percent']-a['one_core_percent']
            report['gates'][scene]=dict(idle_cpu_delta_pp=delta,allowance_pp=.2,status='PASS' if delta<=.2 else 'FAIL')
        if source_identity(ROOT)!=report['source']:raise RuntimeError('Source changed')
        report['status']='FAIL' if any(g['status']=='FAIL' for g in report['gates'].values()) else 'PASS'
    except (ValueError,RuntimeError,OSError,subprocess.SubprocessError) as error:
        report.update(status='FAIL',error=str(error))
    (output/'result.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(report['status'],flush=True)
    return int(report['status']!='PASS')


if __name__=='__main__':
    raise SystemExit(main())
