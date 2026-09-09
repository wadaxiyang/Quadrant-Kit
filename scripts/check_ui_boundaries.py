#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Enforce Kit layering, facade, contracts, provenance and resolved dependencies."""
import argparse
import difflib
import json
from pathlib import Path
import subprocess
import sys

from check_cargo_boundaries import check_metadata, manifests
from slint_contract import ContractError, images, lex, local_path, parse, public_api

ROOT = Path(__file__).resolve().parents[1]
PRODUCT_NAMES = {'Branding', 'TaskRowShell', 'InboxItem', 'InboxPane'}
PRODUCT_MEMBERS = {'Theme': {'q1-accent', 'q2-accent', 'q3-accent', 'q4-accent'},
                   'Typography': {'timer'}, 'UiConstants': {'focus-wide-breakpoint'},
                   'FluentIcons': {f'{n}-{v}' for n in ('quadrants', 'today', 'focus', 'review', 'completed') for v in ('regular', 'filled')} | {'restore-task'}}


def acyclic(graph):
    done, active = set(), []

    def visit(node):
        if node in active:
            raise ContractError('Import cycle: ' + ' -> '.join(map(str, active + [node])))
        if node in done:
            return
        active.append(node)
        for target in graph.get(node, []):
            visit(target)
        active.pop()
        done.add(node)
    for node in graph:
        visit(node)


def boundaries(root):
    root = root.resolve()
    ui, gallery = root / 'ui', root / 'gallery/ui'
    facade = ui / 'kit.slint'
    levels = {'foundation': 0, 'primitives': 1, 'patterns': 2, 'overlays': 2}
    graph, assets, definitions = {}, set(), {}
    for path in sorted([*ui.rglob('*.slint'), *gallery.rglob('*.slint')]):
        source = path.read_text(encoding='utf-8')
        try:
            module = parse(source)
        except ContractError as error:
            raise ContractError(f'{path.relative_to(root)}: {error}') from error
        graph[path] = []
        for target, _ in module['imports']:
            if target == 'std-widgets.slint':
                continue
            if target == '@quadrant-kit':
                if not path.is_relative_to(gallery):
                    raise ContractError(f'Implementation imports facade: {path}')
                graph[path].append(facade)
                continue
            resolved = local_path(path, target, root)
            if resolved.suffix != '.slint':
                raise ContractError(f'Non-Slint import: {target}')
            if path.is_relative_to(gallery):
                if not resolved.is_relative_to(gallery):
                    raise ContractError(f'Gallery bypasses named facade: {path}: {target}')
            else:
                if not resolved.is_relative_to(ui) or resolved == facade:
                    raise ContractError(f'Kit import escapes implementation or imports facade: {path}')
                destination = resolved.relative_to(ui).parts[0]
                if destination not in levels:
                    raise ContractError(f'Unknown Kit layer: {resolved}')
                if path != facade:
                    layer = path.relative_to(ui).parts[0]
                    if layer not in levels or levels[destination] > levels[layer] or (levels[layer] == 2 and destination not in (layer, 'primitives', 'foundation')):
                        raise ContractError(f'Upward/cross-layer dependency: {path}: {target}')
            graph[path].append(resolved)
        for target in images(source):
            resolved = local_path(path, target, root)
            if not resolved.is_relative_to(root / 'assets/icons'):
                raise ContractError(f'Asset outside allowed root: {path}: {target}')
            assets.add(resolved.relative_to(root).as_posix())
        if path.is_relative_to(ui):
            check_product(module['definitions'])
            for name in module['definitions'].keys() & module['exports'].keys():
                if name in definitions:
                    raise ContractError(f'Duplicate exported definition: {name}: {definitions[name]} and {path}')
                definitions[name] = path
            leaked = PRODUCT_NAMES & module['definitions'].keys()
            if leaked:
                raise ContractError(f'Product implementation in Kit: {sorted(leaked)}')
    acyclic(graph)
    return assets


def provenance(root):
    for folder in ('src', 'ui', 'gallery', 'scripts'):
        for path in (root / folder).rglob('*'):
            if path.suffix in ('.rs', '.slint', '.py', '.ps1'):
                text = path.read_text(encoding='utf-8')
                if 'SPDX-License-Identifier: GPL-3.0-only' not in text[:1500] or 'SPDX-FileCopyrightText:' not in text[:1500]:
                    raise ContractError(f'Missing source provenance header: {path}')
    for path in ('LICENSE', 'assets/icons/LICENSE-MIT', 'THIRD-PARTY-NOTICES.md', 'docs/PROVENANCE.md'):
        if not (root / path).is_file():
            raise ContractError(f'Missing license/provenance: {path}')


def check_product(api):
    if PRODUCT_NAMES & api.keys():
        raise ContractError('Public Product export')
    for name, forbidden in PRODUCT_MEMBERS.items():
        if forbidden & api.get(name, {}).get('signature', {}).get('members', {}).keys():
            raise ContractError(f'Public Product member in {name}')


def baseline_findings(actual, expected):
    findings = []
    for category in ('signature', 'defaults'):
        left = {n: v[category] for n, v in expected['exports'].items()}
        right = {n: v[category] for n, v in actual.items()}
        if left != right:
            diff = '\n'.join(difflib.unified_diff(json.dumps(left, indent=2, sort_keys=True).splitlines(), json.dumps(right, indent=2, sort_keys=True).splitlines(), fromfile='baseline', tofile='current', lineterm=''))
            findings.append(f'API {category} changed (explicit review required):\n{diff}')
    return findings


def check(root=ROOT, metadata=True):
    from verify_distribution import verify
    boundaries(root)
    provenance(root)
    api = public_api(root)
    check_product(api)
    baseline = json.loads((root / 'scripts/kit_api_v1.json').read_text(encoding='utf-8'))
    if baseline.get('schema_version') != 1:
        raise ContractError('Unknown API baseline schema')
    findings = baseline_findings(api, baseline)
    if findings:
        raise ContractError('\n'.join(findings))
    from check_native_reuse import verify as verify_native
    native_reuse = verify_native(root)
    distribution = verify(root)
    manifests(root)
    if metadata:
        host = next(line.split(': ', 1)[1] for line in subprocess.check_output(['rustc', '-vV'], cwd=root, text=True).splitlines() if line.startswith('host: '))
        resolved = json.loads(subprocess.check_output(['cargo', 'metadata', '--locked', '--format-version', '1', '--filter-platform', host], cwd=root, text=True, encoding='utf-8'))
        check_metadata(resolved)
    return {'exports': len(api), 'distribution': distribution, 'resolved_cargo_checked': metadata,
            'native_reuse': native_reuse}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--write-baseline', type=Path, help='Explicit candidate output; review diff before adopting; never used by CI')
    args = parser.parse_args()
    try:
        if args.write_baseline:
            api = public_api(ROOT)
            check_product(api)
            args.write_baseline.parent.mkdir(parents=True, exist_ok=True)
            args.write_baseline.write_text(json.dumps({'schema_version': 1, 'exports': api}, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')
            print(f'Wrote review candidate: {args.write_baseline}; no checks were marked passed')
        else:
            print(json.dumps(check(), indent=2))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f'Boundary check failed: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
