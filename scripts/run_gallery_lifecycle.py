#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Build current Gallery as an isolated consumer; separately instrument page lifetime."""
from datetime import datetime, timezone
import json
import os
import re
import shutil
import subprocess

from run_perf import ROOT, execute, generate, source_identity
from observe_idle import observe


def main():
    output = ROOT/'target/gallery-lifecycle'/datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    output.mkdir(parents=True)
    report = dict(source=source_identity(ROOT), status='IN_PROGRESS', runs=[],
                  scope='current Gallery pages and routing; generated host omits OS chrome adapter; instrumented lifetime is separate from timing')
    env = dict(os.environ,CARGO_TARGET_DIR=str(ROOT/'target'),SLINT_BACKEND='winit-software',SLINT_SCALE_FACTOR='1')
    for key in ('SLINT_STYLE','SLINT_DEFAULT_FONT','SLINT_FULLSCREEN','SLINT_DEBUG_PERFORMANCE'):
        env.pop(key,None)
    print('Report:',output/'result.json',flush=True)
    try:
        for instrumented in (False, True):
            name = 'gallery-lifecycle-' + ('probe' if instrumented else 'current')
            project = output/name
            generate(project,'',name,True)
            shutil.copytree(ROOT/'gallery/ui',project/'ui')
            shutil.copyfile(ROOT/'gallery/catalog.tsv',project/'catalog.tsv')
            for module in ('catalog','navigation','navigation_samples'):
                shutil.copyfile(ROOT/f'gallery/src/{module}.rs',project/f'src/{module}.rs')
            shutil.copyfile(ROOT/'scripts/gallery_lifecycle_host.rs',project/'src/main.rs')
            shutil.copyfile(ROOT/'scripts/windows_observation.rs',project/'src/windows_observation.rs')
            build = project/'build.rs'
            build.write_text(build.read_text(encoding='utf-8').replace('probe.slint','ui/gallery.slint'),encoding='utf-8')
            manifest = project/'Cargo.toml'
            manifest.write_text(manifest.read_text(encoding='utf-8')+'\n[features]\ninstance-probe = []\n',encoding='utf-8')
            if instrumented:
                probe = project/'ui/shared/page_lifetime_probe.slint'
                probe.write_text('export global PageLifetimeProbe { in-out property <bool> enabled:true; callback tick(string); callback mounted(string); }\n',encoding='utf-8')
                names = []
                for page in (project/'ui/pages').glob('*.slint'):
                    text = page.read_text(encoding='utf-8')
                    pattern = r'(export component (\w+Page) inherits \w+ \{)'
                    match = re.search(pattern,text)
                    if not match:
                        continue
                    name = match[2]
                    names.append(name)
                    insertion = f'\n init => {{ PageLifetimeProbe.mounted("{name}"); }}\n Timer {{ running:PageLifetimeProbe.enabled; interval:20ms; triggered => {{ PageLifetimeProbe.tick("{name}"); }} }}\n'
                    text = 'import { PageLifetimeProbe } from "../shared/page_lifetime_probe.slint";\n' + text[:match.end()] + insertion + text[match.end():]
                    page.write_text(text,encoding='utf-8')
                entry = project/'ui/gallery.slint'
                entry.write_text('export { PageLifetimeProbe } from "shared/page_lifetime_probe.slint";\n'+entry.read_text(encoding='utf-8'),encoding='utf-8')
                report['instrumented_pages'] = names
            subprocess.run(['cargo','metadata','--offline','--format-version','1'],cwd=project,env=env,stdout=subprocess.DEVNULL,check=True)
            command = ['cargo','build','--locked','--offline','--release'] + (['--features','instance-probe'] if instrumented else [])
            result = execute(command,project,project/'build.log',env)
            if result['exit_code']:
                raise RuntimeError('Gallery lifecycle build failed')
            binary_name = 'gallery-lifecycle-' + ('probe' if instrumented else 'current') + '.exe'
            shutil.copyfile(ROOT/'target/release'/binary_name,project/'bench.exe')
            for repeat in range(1 if instrumented else 3):
                log = project/f'run-{repeat}.log'
                idle = observe([str(project/'bench.exe')],project,log,env)
                raw = log.read_text(encoding='utf-8')
                samples = [json.loads(line[5:]) for line in raw.splitlines() if line.startswith('PAGE ')]
                if 'RESULT=PASS' not in raw or [s['index'] for s in samples] != list(range(200)):
                    raise RuntimeError('Incomplete Gallery lifecycle measurement')
                active = [line for line in raw.splitlines() if line.startswith('ACTIVE ')]
                if instrumented and len(active) != 200:
                    raise RuntimeError('Missing page instance lifetime evidence')
                report['runs'].append(dict(instrumented=instrumented,repeat=repeat,samples=samples,active_checks=len(active),idle=idle,log=str(log)))
                print('Gallery lifecycle',instrumented,repeat,'PASS',flush=True)
        if report['source'] != source_identity(ROOT):
            raise RuntimeError('Source changed')
        report['status']='PASS'
    except (ValueError,RuntimeError,OSError,subprocess.SubprocessError) as error:
        report.update(status='FAIL',error=str(error))
    (output/'result.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(report['status'],flush=True)
    return int(report['status']!='PASS')


if __name__=='__main__':
    raise SystemExit(main())
