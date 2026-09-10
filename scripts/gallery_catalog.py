# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Gallery-local metadata shared with the Rust route controller."""
import csv
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIELDS = ('id', 'title', 'parent', 'icon', 'alias', 'component', 'file', 'exports', 'keywords')


def load_catalog(path=ROOT/'gallery/catalog.tsv'):
    rows = []
    ids, aliases, owners, files, components = set(), set(), set(), set(), set()
    ancestors = []
    for fields in csv.reader((line for line in path.read_text(encoding='utf-8').splitlines() if line and not line.startswith('#')), delimiter='\t'):
        if len(fields) != len(FIELDS):
            raise ValueError('Catalog requires nine fields')
        row = dict(zip(FIELDS, fields))
        if not row['id'] or any(c not in 'abcdefghijklmnopqrstuvwxyz0123456789-' for c in row['id']) or row['id'] in ids or not row['title']:
            raise ValueError('Invalid or duplicate catalog ID')
        ids.add(row['id'])
        parent = next((r for r in rows if r['id'] == row['parent'] and not r['component']), None)
        if row['parent'] and parent is None:
            raise ValueError('Unknown catalog parent')
        row['depth'] = parent['depth'] + 1 if parent else 0
        depth = row['depth']
        ancestors = ancestors[:depth]
        if depth > 2 or len(ancestors) != depth or (depth and ancestors[-1] != row['parent']):
            raise ValueError('Non-preorder catalog')
        ancestors.append(row['id'])
        if int(row['icon']) not in range(9):
            raise ValueError('Invalid icon')
        if row['alias']:
            alias = int(row['alias'])
            if alias not in range(8) or alias in aliases or not row['component']:
                raise ValueError('Invalid numeric alias')
            aliases.add(alias)
        if row['component']:
            file = row['file']
            if not file.endswith('_page.slint') or '/' in file or '\\' in file or file in files or row['component'] in components:
                raise ValueError('Invalid or duplicate page implementation')
            files.add(file)
            components.add(row['component'])
        elif row['file'] or row['exports']:
            raise ValueError('Category cannot own a page or exports')
        for name in filter(None, row['exports'].split(',')):
            if name in owners:
                raise ValueError('Duplicate visual component owner')
            owners.add(name)
        rows.append(row)
    if aliases != set(range(8)):
        raise ValueError('All eight numeric aliases must be explicit')
    for index, row in enumerate(rows):
        if not row['component'] and (index+1 == len(rows) or rows[index+1]['parent'] != row['id']):
            raise ValueError('Empty category')
    return rows


def resolve_destination(page=None, destination=None, catalog=None):
    catalog = load_catalog() if catalog is None else catalog
    if page is not None and destination is not None:
        raise ValueError('--page and --destination cannot be supplied together')
    if page is not None:
        if page not in range(8):
            raise ValueError('--page must be in 0..7')
        return next(row['id'] for row in catalog if row['alias'] == str(page))
    destination = 'home' if destination is None else destination
    if not any(row['id'] == destination and row['component'] for row in catalog):
        raise ValueError('Unknown Gallery destination: '+destination)
    return destination
