#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Verify a current local source archive in a generated consumer under target/."""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile

from run_perf import ROOT, execute, generate, source_identity
from verify_distribution import verify_archive
from verify_remote import BUILD, RUST, SLINT


def extract_files(archive, destination):
    destination = destination.resolve()
    with tarfile.open(archive, 'r:gz') as package:
        for member in package.getmembers():
            target = (destination/member.name).resolve()
            if not target.is_relative_to(destination) or not member.isfile():
                raise ValueError('Archive must contain only contained regular files')
            target.parent.mkdir(parents=True, exist_ok=True)
            with package.extractfile(member) as stream:
                target.write_bytes(stream.read())


def main():
    source = source_identity(ROOT)
    if source['dirty']:
        raise ValueError('Package consumer requires a clean committed source')
    output = ROOT/'target/package-consumer'/datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    output.mkdir(parents=True)
    report = dict(source=source,status='IN_PROGRESS',scope='local packaged source; not remote retention/CI or fresh dependency cache')
    env = dict(os.environ,CARGO_TARGET_DIR=str(ROOT/'target'),SLINT_BACKEND='winit-software',SLINT_SCALE_FACTOR='1')
    for key in ('SLINT_STYLE','SLINT_DEFAULT_FONT','SLINT_FULLSCREEN','SLINT_DEBUG_PERFORMANCE'):
        env.pop(key,None)
    print('Report:',output/'result.json',flush=True)
    try:
        result = execute(['cargo','package','--locked','-p','quadrant-kit'],ROOT,output/'package.log',env)
        if result['exit_code']:
            raise RuntimeError('Cargo package verification failed')
        archive = ROOT/'target/package/quadrant-kit-0.1.1.crate'
        report['archive'] = verify_archive(archive)
        report['archive_sha256'] = hashlib.sha256(archive.read_bytes()).hexdigest()
        extract_files(archive,output/'source')
        package = output/'source/quadrant-kit-0.1.1'
        consumer = output/'consumer'
        generate(consumer,'','packaged-kit-consumer',True)
        manifest = consumer/'Cargo.toml'
        text = manifest.read_text(encoding='utf-8').replace(json.dumps(ROOT.as_posix()),json.dumps(package.as_posix()))
        text = text.replace('[dependencies]','[dependencies]\npng = "=0.18.1"')
        manifest.write_text(text,encoding='utf-8')
        (consumer/'ui').mkdir()
        (consumer/'build.rs').write_text(BUILD,encoding='utf-8')
        (consumer/'src/main.rs').write_text(RUST,encoding='utf-8')
        (consumer/'ui/main.slint').write_text('export { KitApiProbe } from "api_probe.slint";\n'+SLINT,encoding='utf-8')
        shutil.copyfile(ROOT/'gallery/ui/api_probe.slint',consumer/'ui/api_probe.slint')
        metadata = subprocess.check_output(['cargo','metadata','--offline','--format-version','1'],cwd=consumer,env=env,text=True,encoding='utf-8')
        (output/'resolved.json').write_text(metadata,encoding='utf-8')
        kit = next(p for p in json.loads(metadata)['packages'] if p['name']=='quadrant-kit')
        if Path(kit['manifest_path']).resolve() != (package/'Cargo.toml').resolve() or kit['source'] is not None:
            raise ValueError('Consumer resolved source outside extracted package')
        report['resolved_kit_manifest'] = kit['manifest_path']
        result = execute(['cargo','build','--locked','--offline','--release'],consumer,output/'build.log',env)
        if result['exit_code']:
            raise RuntimeError('Packaged consumer build failed')
        runtime = output/'empty-runtime'
        runtime.mkdir()
        binary = runtime/'packaged-kit-consumer.exe'
        shutil.copyfile(ROOT/'target/release/packaged-kit-consumer.exe',binary)
        for theme in ('light','dark'):
            png = output/(theme+'.png')
            result = execute([str(binary)],runtime,output/(theme+'.log'),dict(env,SMOKE_THEME=theme,SMOKE_PNG=str(png)))
            if result['exit_code'] or not png.is_file():
                raise RuntimeError('Copied consumer rendering failed')
        if source_identity(ROOT) != source:
            raise RuntimeError('Source changed')
        report['status']='PASS'
    except (ValueError,RuntimeError,OSError,subprocess.SubprocessError) as error:
        report.update(status='FAIL',error=str(error))
    (output/'result.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(report['status'],flush=True)
    return int(report['status']!='PASS')


if __name__=='__main__':
    raise SystemExit(main())
