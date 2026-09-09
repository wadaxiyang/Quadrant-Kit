#!/usr/bin/env python3
# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Check current native reuse records and bounded transition debt; no runtime proof."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import sys

from slint_contract import ContractError, canonical, implementation_facts, local_path, parse, public_exports

ROOT = Path(__file__).resolve().parents[1]
# P1 admits only this already-audited command debt, never future custom buttons.
# Remove each allowance when its migration lands; do not extend to excuse new input.
PENDING_COMPONENTS = frozenset({'NavigationItemRow'})
INPUT_ELEMENTS = frozenset({'TouchArea', 'FocusScope', 'TextInput', 'Flickable'})
NATIVE_CONTROLS = frozenset({'Button', 'LineEdit', 'TextEdit', 'CheckBox', 'Switch', 'RadioGroup', 'ComboBox', 'Slider', 'SpinBox', 'Spinner', 'ProgressIndicator', 'ScrollView', 'ListView', 'StandardListView', 'GroupBox', 'TabWidget', 'StandardTableView', 'DatePickerPopup', 'TimePickerPopup'})
NATIVE_BUILTINS = frozenset({'Tooltip', 'PopupWindow', 'ContextMenuArea', 'Menu'})
STATUSES = frozenset({'native-wrapper', 'presenter', 'composed', 'reviewed-exception', 'custom/pending-migration'})
CLASSIFICATIONS = {'standard-control-wrapper': {'native-wrapper'}, 'presentation': {'presenter'},
                   'composed-missing-standard-control': {'composed', 'reviewed-exception'},
                   'standard-control-pending': {'custom/pending-migration'}}


def implementation_digest(definition, body):
    material = json.dumps(definition, sort_keys=True) + '\n' + canonical(body)
    return hashlib.sha256(material.encode()).hexdigest()


def inventory(root):
    modules, components = {}, {}
    for path in sorted((root / 'ui').rglob('*.slint')):
        parsed = parse(path.read_text(encoding='utf-8'))
        modules[path] = parsed
        for name, definition in parsed['definitions'].items():
            if path.relative_to(root / 'ui').parts[0] == 'foundation' and (definition['signature']['kind'] == 'component' or implementation_facts(parsed['bodies'][name])['instances']):
                raise ContractError(f'Foundation instantiates UI: {name}')
            if definition['signature']['kind'] == 'component':
                components[(path, name)] = (definition, parsed['bodies'][name])
    return modules, components


def verify(root=ROOT, manifest=None):
    from check_ui_boundaries import boundaries
    root = root.resolve()
    boundaries(root)
    manifest = manifest if manifest is not None else json.loads((root / 'scripts/native_reuse_manifest.json').read_text(encoding='utf-8'))
    if manifest.get('schema_version') != 1 or manifest.get('slint_version') != '1.17.1':
        raise ContractError('Unknown native manifest schema or Slint version')
    modules, components = inventory(root)
    records = {}
    for record in manifest.get('records', []):
        required = {'component', 'classification', 'implementation', 'status', 'native_dependencies', 'missing_capability', 'permitted_custom_behavior', 'forbidden_custom_behavior', 'validation', 'review_on_slint_upgrade', 'public'}
        if not required <= record.keys() or record['status'] not in STATUSES:
            raise ContractError('Incomplete native reuse record or unknown status')
        if record['status'] not in CLASSIFICATIONS.get(record['classification'], set()):
            raise ContractError('Classification/status mismatch in native record')
        for field in ('permitted_custom_behavior', 'forbidden_custom_behavior', 'validation'):
            if not isinstance(record[field], list) or not all(isinstance(value, str) and value for value in record[field]):
                raise ContractError(f'Invalid native record list: {field}')
        relative = Path(record['implementation'])
        path = (root / relative).resolve()
        if relative.is_absolute() or not path.is_relative_to(root / 'ui'):
            raise ContractError('Native record implementation escapes Kit')
        key = path, record['component']
        if key in records:
            raise ContractError('Duplicate native record')
        if not isinstance(record['native_dependencies'], list) or not all(isinstance(x, str) for x in record['native_dependencies']):
            raise ContractError('Invalid native dependency list')
        if record['review_on_slint_upgrade'] is not True or not record['validation']:
            raise ContractError('Native review/validation obligations missing')
        records[key] = record
    if records.keys() != components.keys():
        missing = [name for _, name in components.keys() - records.keys()]
        stale = [name for _, name in records.keys() - components.keys()]
        raise ContractError(f'Current component records differ: missing={missing}, stale={stale}')
    public = {(entry['path'], entry['name']) for entry in public_exports(root).values() if entry['definition']['signature']['kind'] == 'component'}

    def resolve(path, name, trail=frozenset()):
        key = path, name
        if key in trail:
            raise ContractError('Native symbol cycle')
        module = modules[path]
        if key in components:
            return key
        matches = [(source, original) for source, pairs in module['imports'] for original, alias in pairs if alias == name]
        # Local re-export may point through an alias chain.
        if name in module['exports'] and module['exports'][name][0] is not None:
            matches = [module['exports'][name]]
        if len(matches) > 1:
            raise ContractError(f'Ambiguous native symbol: {name}')
        if matches:
            source, original = matches[0]
            if source == 'std-widgets.slint':
                if original not in NATIVE_CONTROLS:
                    raise ContractError(f'Unverified native component: {original}')
                return original
            return resolve(local_path(path, source, root), original, trail | {key})
        if name in module['exports'] and module['exports'][name][0] is None:
            return resolve(path, module['exports'][name][1], trail | {key})
        if name in NATIVE_CONTROLS:
            raise ContractError(f'Native control lacks public import: {name}')
        return name  # builtin element; Slint compiler validates unknown elements

    def closure(key, trail=frozenset()):
        if key in trail:
            raise ContractError('Component composition cycle')
        definition, body = components[key]
        facts = implementation_facts(body)
        native, inputs = set(), set(facts['handlers'])
        names = list(facts['instances'])
        if definition['signature']['inherits']:
            names.append(definition['signature']['inherits'])
        for name in names:
            target = resolve(key[0], name)
            if isinstance(target, tuple):
                child_native, child_inputs = closure(target, trail | {key})
                native.update(child_native)
                inputs.update(child_inputs)
            else:
                if target in NATIVE_CONTROLS | NATIVE_BUILTINS:
                    native.add(target)
                if target in INPUT_ELEMENTS:
                    inputs.add(target)
        return native, inputs

    pending = []
    for key, (definition, body) in components.items():
        record = records[key]
        if record['public'] != (key in public):
            raise ContractError(f'Public/private record mismatch: {key[1]}')
        native, inputs = closure(key)
        if set(record['native_dependencies']) != native:
            raise ContractError(f'Native dependencies differ for {key[1]}: actual={sorted(native)}')
        status = record['status']
        if status == 'native-wrapper':
            owner = record.get('native_owner')
            facts = implementation_facts(body)
            direct = list(facts['instances']) + [definition['signature']['inherits']]
            owners = [resolve(key[0], name) for name in direct if name]
            if owner not in NATIVE_CONTROLS or owners.count(owner) != 1 or inputs:
                raise ContractError(f'Native wrapper needs one real owner and no duplicate input: {key[1]}')
            if facts['literal_hidden']:
                raise ContractError(f'Literal hidden native proxy in {key[1]}')
        elif status == 'presenter' and inputs:
            raise ContractError(f'Unreviewed custom input in {key[1]}')
        elif status == 'composed':
            facts = implementation_facts(body)
            direct_inputs = set(facts['instances']) & INPUT_ELEMENTS
            if direct_inputs or facts['handlers'] or definition['signature']['inherits'] in INPUT_ELEMENTS:
                raise ContractError(f'Unreviewed custom input in {key[1]}')
        elif status in ('reviewed-exception', 'custom/pending-migration'):
            if not record['missing_capability'] or not record['permitted_custom_behavior'] or not record['forbidden_custom_behavior']:
                raise ContractError(f'Unbounded custom exception in {key[1]}')
            if record.get('reviewed_implementation_sha256') != implementation_digest(definition, body):
                raise ContractError(f'Custom implementation changed without scoped review: {key[1]}')
            if status == 'custom/pending-migration':
                if key[1] not in PENDING_COMPONENTS:
                    raise ContractError(f'New pending command debt is forbidden: {key[1]}')
                pending.append(key[1])
    return {'component_records': len(records), 'public_components': len(public),
            'statuses': dict(Counter(r['status'] for r in records.values())),
            'pending_migration': sorted(pending),
            'runtime_verified': False}


def main():
    argparse.ArgumentParser(description=__doc__).parse_args()
    try:
        print(json.dumps(verify(), indent=2))
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f'Native reuse check failed: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
