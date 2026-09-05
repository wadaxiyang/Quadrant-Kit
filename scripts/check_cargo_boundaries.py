# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Cargo manifest and resolved-edge checks; Tasks mode is fixture-tested only.

Tasks' final cutover guard must integrate this policy with its Product contracts,
config/build-source checks and target-specific Agent/GUI graph rules in Phase 4.
"""
import re
import tomllib

from slint_contract import ContractError

PRODUCT_PACKAGES = {'quadrant-domain', 'quadrant-application', 'quadrant-storage',
                    'quadrant-platform', 'quadrant-protocol', 'quadrant-agent',
                    'quadrant-ui', 'quadrant-ui-gallery', 'quadrant-app'}
KIT_URL = 'https://github.com/wadaxiyang/Quadrant-Kit.git'


def dependencies(manifest):
    for kind in ('dependencies', 'build-dependencies', 'dev-dependencies'):
        for name, value in manifest.get(kind, {}).items():
            yield kind, name, value
    for target in manifest.get('target', {}).values():
        yield from dependencies(target)


def check_manifest(manifest, workspace, mode='kit'):
    if manifest.get('patch') or manifest.get('replace'):
        raise ContractError('Source patch/replace requires explicit review; forbidden in this candidate')
    for kind, alias, value in dependencies(manifest):
        value = {'version': value} if isinstance(value, str) else value
        if value.get('workspace'):
            inherited = workspace.get('dependencies', {}).get(alias)
            if not isinstance(inherited, (dict, str)):
                raise ContractError(f'Unresolved workspace dependency: {alias}')
            inherited = {'version': inherited} if isinstance(inherited, str) else inherited
            value = {**inherited, **value}
        name = value.get('package', alias)
        if mode == 'kit' and name in PRODUCT_PACKAGES:
            raise ContractError(f'Product dependency: {alias} -> {name}')
        if name == 'quadrant-kit':
            if mode == 'tasks':
                if 'path' in value or value.get('git') != KIT_URL or not re.fullmatch(r'[0-9a-f]{40}', value.get('rev', '')) or 'branch' in value or 'tag' in value:
                    raise ContractError(f'Tasks Kit dependency must use public Git + full SHA: {alias}')
            elif not (manifest.get('package', {}).get('name') == 'quadrant-kit-gallery' and kind == 'build-dependencies' and value.get('path') == '..' and 'git' not in value):
                raise ContractError('Only Gallery same-repository Kit path build dependency is allowed')


def reachable(metadata, package_id, kinds=(None,)):
    nodes = {n['id']: n for n in metadata['resolve']['nodes']}
    visited, pending = set(), [package_id]
    while pending:
        current = pending.pop()
        if current in visited:
            continue
        visited.add(current)
        for edge in nodes[current]['deps']:
            if any(k['kind'] in kinds for k in edge['dep_kinds']):
                pending.append(edge['pkg'])
    return visited - {package_id}


def check_metadata(metadata, mode='kit'):
    packages = {p['id']: p for p in metadata['packages']}
    kit = [p for p in packages.values() if p['name'] == 'quadrant-kit']
    if len(kit) != 1:
        raise ContractError('Expected exactly one resolved quadrant-kit')
    if mode == 'tasks':
        source = kit[0].get('source') or ''
        if not re.fullmatch(re.escape('git+' + KIT_URL) + r'\?rev=([0-9a-f]{40})#\1', source):
            raise ContractError('Resolved Kit is not the pinned public Git source')
    else:
        if kit[0]['id'] not in metadata['workspace_members'] or kit[0].get('source'):
            raise ContractError('Kit helper must be this workspace package')
        if reachable(metadata, kit[0]['id']):
            raise ContractError('Kit helper has runtime dependencies')
        if any(p['name'] in PRODUCT_PACKAGES for p in packages.values()):
            raise ContractError('Product package in Kit resolved graph')
        for p in packages.values():
            if p['name'] in ('slint', 'slint-build') and p['version'] != '1.17.1':
                raise ContractError('Unexpected resolved Slint version')


def manifests(root):
    paths = [root / 'Cargo.toml', root / 'gallery/Cargo.toml']
    values = [tomllib.loads(p.read_text(encoding='utf-8')) for p in paths]
    workspace = values[0]['workspace']
    for value in values:
        check_manifest(value, workspace)
    # Candidate has no legitimate source overrides. Includes untracked local config.
    for directory in [root, *root.parents]:
        for name in ('config', 'config.toml'):
            path = directory / '.cargo' / name
            if path.is_file():
                config = tomllib.loads(path.read_text(encoding='utf-8'))
                if any(config.get(key) for key in ('paths', 'patch', 'source')):
                    raise ContractError(f'Cargo source override in {path}')
