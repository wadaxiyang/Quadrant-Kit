# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Anonymous retained-ref fetch and isolated Git+SHA neutral consumer."""
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

from check_cargo_boundaries import KIT_URL, check_metadata, PRODUCT_PACKAGES

BUILD = '''// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
fn main() {
    let facade = quadrant_kit::slint_library_path();
    assert!(facade.is_file());
    let libraries = std::collections::HashMap::from([
        (quadrant_kit::SLINT_LIBRARY_NAME.to_owned(), facade)
    ]);
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent".into())
        .with_library_paths(libraries)
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
'''
SLINT = '''// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
import { Theme, ThemeMode, FluentButton, FluentIcon, FluentIcons,
         ModalManager, ModalKind, ToastHost, ToastKind } from "@quadrant-kit";
export component ConsumerWindow inherits Window {
    width: 640px;
    height: 440px;
    title: "Neutral Kit consumer";
    background: Theme.background;
    in-out property <bool> confirmed: false;
    in-out property <ThemeMode> mode: ThemeMode.light;
    changed mode => { Theme.mode = root.mode; }
    init => { Theme.mode = root.mode; Theme.system_dark = false; }
    VerticalLayout {
        padding: 24px;
        spacing: 16px;
        Text { text: "Independent source consumer"; color: Theme.text_primary; }
        FluentIcon { source: FluentIcons.menu; size: 32px; }
        FluentButton { text: "Apply sample"; primary: true; clicked => { root.confirmed = true; } }
        ToastHost { shown: true; auto_dismiss: false; kind: ToastKind.success;
                    message: "Public components loaded"; dismissed => {} }
    }
    ModalManager { shown: false; title: "Apply?"; kind: ModalKind.info;
                   accepted => { root.confirmed = true; } dismissed => {} }
}
'''
RUST = '''// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::ComponentHandle;
slint::include_modules!();
fn capture(ui: slint::Weak<ConsumerWindow>, remaining: u8) {
    slint::Timer::single_shot(std::time::Duration::from_millis(1200), move || {
        let ui = ui.upgrade().expect("window remains alive");
        let pixels = ui.window().take_snapshot().expect("snapshot supported");
        if !pixels.as_bytes().chunks_exact(4).any(|p| p[3] != 0) {
            assert!(remaining > 1, "snapshot is transparent");
            ui.window().request_redraw();
            capture(ui.as_weak(), remaining - 1);
            return;
        }
        let file = std::fs::File::create(std::env::var_os("SMOKE_PNG").unwrap()).unwrap();
        let mut encoder = png::Encoder::new(file, pixels.width(), pixels.height());
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.write_header().unwrap().write_image_data(pixels.as_bytes()).unwrap();
        slint::quit_event_loop().unwrap();
    });
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = ConsumerWindow::new()?;
    ui.set_mode(if std::env::var("SMOKE_THEME").as_deref() == Ok("dark") {
        ThemeMode::Dark
    } else { ThemeMode::Light });
    ui.show()?;
    capture(ui.as_weak(), 3);
    slint::run_event_loop()?;
    Ok(())
}
'''


def validate_request(url, rev, retained_ref):
    if url != KIT_URL or not re.fullmatch(r'[0-9a-f]{40}', rev):
        raise ValueError('Require the verified public Kit URL and full lowercase commit SHA')
    if not re.fullmatch(r'refs/tags/candidate/extraction-[0-9a-f]{7,40}', retained_ref):
        raise ValueError('Require an explicit candidate/extraction-<SHA> retained tag ref')


def isolated_environment(work, original=None):
    original = dict(os.environ if original is None else original)
    # Keep OS/toolchain discovery and explicit network proxy settings. Never copy
    # source overrides, Cargo credentials or injected Git configuration.
    env = {k: v for k, v in original.items() if not k.upper().startswith(('CARGO_', 'GIT_', 'RUSTFLAGS', 'RUSTDOCFLAGS', 'RUSTC_', 'SLINT_', 'QUADRANT_', 'GH_', 'GITHUB_')) and k.upper() not in ('RUSTC', 'RUSTDOC', 'RUSTUP_TOOLCHAIN')}
    env.update(CARGO_HOME=str(work / 'cargo-home'), CARGO_TARGET_DIR=str(work / 'target'),
               GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull,
               GIT_TERMINAL_PROMPT='0', GCM_INTERACTIVE='Never',
               CARGO_NET_GIT_FETCH_WITH_CLI='true', RUSTUP_TOOLCHAIN='1.94.1')
    return env


def check_resolution(metadata, cargo_home, rev):
    check_metadata(metadata, 'tasks')
    kit = next(p for p in metadata['packages'] if p['name'] == 'quadrant-kit')
    expected = f'git+{KIT_URL}?rev={rev}#{rev}'
    if kit['source'] != expected:
        raise ValueError('Consumer resolved another revision')
    manifest = Path(kit['manifest_path']).resolve()
    if not manifest.is_relative_to(cargo_home.resolve() / 'git/checkouts'):
        raise ValueError('Consumer resolved Kit outside its fresh Cargo Git checkout')
    for package in metadata['packages']:
        if package['name'] in PRODUCT_PACKAGES:
            raise ValueError('Product dependency in neutral consumer')
        if package['name'] in ('slint', 'slint-build') and package['version'] != '1.17.1':
            raise ValueError('Unexpected consumer Slint version')
    return {'source': kit['source'], 'manifest_path': str(manifest)}


def generate_consumer(path, url, rev):
    (path / 'src').mkdir(parents=True)
    (path / 'ui').mkdir()
    manifest = f'''[package]
name = "neutral-kit-consumer"
version = "0.0.0"
edition = "2024"
rust-version = "1.92"
publish = false

[workspace]
resolver = "3"

[dependencies]
slint = "=1.17.1"
png = "=0.18.1"

[build-dependencies]
slint-build = "=1.17.1"
quadrant-kit = {{ git = "{url}", rev = "{rev}" }}
'''
    for name, content in [('Cargo.toml', manifest), ('build.rs', BUILD), ('src/main.rs', RUST), ('ui/main.slint', SLINT)]:
        (path / name).write_text(content, encoding='utf-8', newline='\n')


def verify_remote(url, rev, retained_ref, run_gui=False, output=None):
    validate_request(url, rev, retained_ref)
    work = Path(tempfile.mkdtemp(prefix='quadrant-kit-remote-')).resolve()
    env = isolated_environment(work)
    # Cargo also reads ancestor configs, even with an isolated CARGO_HOME.
    for parent in work.parents:
        for name in ('config', 'config.toml'):
            config = parent / '.cargo' / name
            if config.exists():
                raise ValueError(f'Ancestor Cargo config requires a neutral verification location: {config}')
    (work / 'cargo-home').mkdir()
    logs = work / 'logs'
    logs.mkdir()
    result = {'schema_version': 1, 'status': 'IN_PROGRESS', 'kit_url': url, 'rev': rev,
              'retained_ref': retained_ref, 'work_directory': str(work), 'steps': [],
              'runtime': 'NOT_RUN', 'anonymous_git': True, 'fresh_cargo_cache': True}
    report = Path(output).resolve() if output else work / 'result.json'

    def save():
        report.parent.mkdir(parents=True, exist_ok=True)
        data = json.dumps(result, indent=2) + '\n'
        report.write_text(data, encoding='utf-8')
        (work / 'result.json').write_text(data, encoding='utf-8')

    def run(label, args, cwd=work, timeout=1800, extra=None):
        save()
        print(f'Remote verification: {label}', flush=True)
        log = logs / (label + '.log')
        with log.open('w', encoding='utf-8') as stream:
            process = subprocess.run(args, cwd=cwd, env={**env, **(extra or {})}, stdout=stream, stderr=subprocess.STDOUT, timeout=timeout)
        result['steps'].append({'name': label, 'exit_code': process.returncode, 'log': str(log)})
        if process.returncode:
            raise ValueError(f'{label} failed with exit {process.returncode}; see {log}')
        return log.read_text(encoding='utf-8')

    try:
        result['toolchain'] = run('toolchain', ['rustc', '-vV']).strip()
        fetched = work / 'retained-reference'
        run('git-init', ['git', 'init', str(fetched)])
        run('anonymous-fetch', ['git', '-c', 'credential.helper=', 'fetch', '--no-tags', '--depth=1', url, retained_ref], fetched)
        peeled = run('peel-commit', ['git', 'rev-parse', 'FETCH_HEAD^{commit}'], fetched).strip()
        result['fetched_commit'] = peeled
        if peeled != rev:
            raise ValueError(f'Retained reference resolves to {peeled}, expected {rev}')
        consumer = work / 'consumer'
        generate_consumer(consumer, url, rev)
        run('generate-lockfile', ['cargo', 'generate-lockfile'], consumer)
        lock = consumer / 'Cargo.lock'
        result['lock_sha256'] = hashlib.sha256(lock.read_bytes()).hexdigest()
        host = next(line.split(': ', 1)[1] for line in result['toolchain'].splitlines() if line.startswith('host: '))
        raw = run('metadata', ['cargo', 'metadata', '--locked', '--format-version', '1', '--filter-platform', host], consumer)
        # Keep machine JSON separate from stderr progress messages.
        metadata = json.loads(next(line for line in raw.splitlines() if line.startswith('{')))
        result['resolution'] = check_resolution(metadata, work / 'cargo-home', rev)
        run('consumer-build', ['cargo', 'build', '--locked'], consumer)
        if hashlib.sha256(lock.read_bytes()).hexdigest() != result['lock_sha256']:
            raise ValueError('Locked consumer build changed its lockfile')
        if run_gui:
            from capture_gallery_baseline import png_info
            runtime = work / 'runtime'
            runtime.mkdir()
            filename = 'neutral-kit-consumer' + ('.exe' if os.name == 'nt' else '')
            binary = runtime / filename
            shutil.copy2(work / 'target/debug' / filename, binary)
            result['runtime_images'] = []
            for theme in ('light', 'dark'):
                png = runtime / (theme + '.png')
                run('runtime-' + theme, [str(binary)], runtime, 30,
                    {'SMOKE_PNG': str(png), 'SMOKE_THEME': theme, 'SLINT_BACKEND': 'winit-software', 'SLINT_SCALE_FACTOR': '1'})
                result['runtime_images'].append({'theme': theme, 'path': str(png), **png_info(png)})
            result['runtime'] = 'PASS'
        result['status'] = 'PASS'
        return result
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        result['status'] = 'FAIL'
        result['error'] = str(error)
        raise
    finally:
        save()
        print(f'Remote verification report: {report}', flush=True)
