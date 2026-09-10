# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import base64
import copy
from pathlib import Path
import subprocess
import sys
import tempfile
import contextlib
import io
import json
import os
import re
from unittest.mock import patch
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from capture_gallery_baseline import SCHEMA_VERSION, parse_args, scene_environment, navigation_cells, png_info, reusable, source_identity
from gallery_catalog import ROOT, load_catalog, resolve_destination


class CaptureReuseTests(unittest.TestCase):
    def test_navigation_matrix_covers_each_model_in_both_presentations_and_themes(self):
        cells = navigation_cells()
        self.assertEqual(len(cells), 68)
        self.assertEqual(len(set(cells)), 68)
        for theme in ['light', 'dark']:
            for compact in [False, True]:
                self.assertEqual({case for _, _, t, _, case, c in cells if t == theme and c == compact}, set(range(17)))

    def test_reuse_rejects_other_page_preview_theme_source_and_environment(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'sample.png'
            path.write_bytes(base64.b64decode('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jw1kAAAAASUVORK5CYII='))
            scene = dict(destination='controls', preview=1, theme='light', content_sha256='original', renderer='software', font_policy='system', scale_percent=100, navigation_case=0, navigation_compact=False)
            record = dict(schema_version=SCHEMA_VERSION, scene=scene, image=png_info(path))
            self.assertTrue(reusable(record, scene, path))
            for key, value in [('destination','fluent-button'),('preview',2),('theme','dark'),('content_sha256','edited'),('renderer','skia'),('font_policy','changed'),('scale_percent',200),('navigation_case',7),('navigation_compact',True)]:
                changed = copy.deepcopy(scene)
                changed[key] = value
                self.assertFalse(reusable(record, changed, path), key)
            path.write_bytes(path.read_bytes()+b'changed')
            self.assertFalse(reusable(record,scene,path))
            path.unlink()
            self.assertFalse(reusable(record,scene,path))

    def test_route_options_aliases_and_conflicts(self):
        aliases = ['home','tokens','typography','icons','controls','surfaces','feedback','navigation-view']
        self.assertEqual(parse_args([]).resolved_destination, 'home')
        for index, destination in enumerate(aliases):
            self.assertEqual(parse_args(['--page',str(index)]).resolved_destination, destination)
        for row in load_catalog():
            if row['component']:
                self.assertEqual(parse_args(['--destination',row['id']]).resolved_destination, row['id'])
        for args in [['--page','0','--destination','home'],['--destination',''],['--destination','missing'],['--destination','controls-group'],['--page','8'],['--mode','Navigation'],['--mode','Catalog','--page','0']]:
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                parse_args(args)
        self.assertEqual(parse_args(['--mode','Navigation','--destination','navigation-view']).resolved_destination, 'navigation-view')

    def test_old_schema_and_distinct_destinations_cannot_reuse(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'sample.png'
            path.write_bytes(base64.b64decode('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jw1kAAAAASUVORK5CYII='))
            scene = dict(destination='home')
            record = dict(schema_version=1, scene=scene, image=png_info(path))
            self.assertFalse(reusable(record,scene,path))
            record['schema_version'] = SCHEMA_VERSION
            self.assertTrue(reusable(record,scene,path))
            self.assertFalse(reusable(record,dict(destination='all-components'),path))

    def test_snapshot_environment_removes_inherited_route_and_options(self):
        with patch.dict(os.environ, {'QUADRANT_GALLERY_PAGE':'4','QUADRANT_GALLERY_DESTINATION':'missing','QUADRANT_GALLERY_UNKNOWN':'old'}):
            env = scene_environment(1040,800,'light','home',1,'fresh.png',0,False,'winit-software',100)
        self.assertNotIn('QUADRANT_GALLERY_PAGE',env)
        self.assertNotIn('QUADRANT_GALLERY_UNKNOWN',env)
        self.assertEqual(env['QUADRANT_GALLERY_DESTINATION'],'home')

    def test_catalog_owns_every_public_visual_component_and_typed_route(self):
        catalog = load_catalog()
        api = json.loads((ROOT/'scripts/kit_api_v1.json').read_text(encoding='utf-8'))
        public = {name for name,item in api['exports'].items() if item['signature']['kind'] == 'component'}
        owners = [name for row in catalog for name in row['exports'].split(',') if name]
        self.assertEqual(set(owners), public)
        self.assertEqual(len(owners),len(set(owners)))
        shell = (ROOT/'gallery/ui/gallery.slint').read_text(encoding='utf-8')
        dispatch = re.findall(r'if root.gallery_destination == "([a-z0-9-]+)": (\w+) \{',shell)
        destinations = [row for row in catalog if row['component']]
        self.assertEqual(set(dispatch),{(row['id'],row['component']) for row in destinations})
        self.assertEqual(len(dispatch),len(destinations))
        self.assertEqual({p.name for p in (ROOT/'gallery/ui/pages').glob('*_page.slint')},{row['file'] for row in destinations})
        for row in destinations:
            source = (ROOT/'gallery/ui/pages'/row['file']).read_text(encoding='utf-8')
            self.assertIn('export component '+row['component']+' inherits GalleryPage',source)
            self.assertIn('import { '+row['component']+' } from "pages/'+row['file']+'";',shell)
            self.assertEqual(resolve_destination(destination=row['id']),row['id'])

    def test_catalog_rejects_broken_parent_duplicate_id_export_and_alias(self):
        source = (ROOT/'gallery/catalog.tsv').read_text(encoding='utf-8')
        replacements = [('home\tHome','all-components\tHome'),('tokens\tTheme / Colors\tdesign-guidance','tokens\tTheme / Colors\tmissing'),('\t4\tControlsPage','\t8\tControlsPage'),('\tIconButton\t','\tFluentButton\t')]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'catalog.tsv'
            for old,new in replacements:
                self.assertIn(old, source)
                path.write_text(source.replace(old,new),encoding='utf-8')
                with self.assertRaises(ValueError):
                    load_catalog(path)

    def test_content_identity_tracks_uncommitted_bytes_but_ignores_outputs(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            subprocess.run(['git','init','-q'],cwd=root,check=True)
            (root/'.gitignore').write_text('/target/\n')
            source = root/'sample.slint'
            source.write_text('before')
            original = source_identity(root)
            self.assertIsNone(original['source_full_sha'])
            self.assertTrue(original['dirty'])
            (root/'target').mkdir()
            (root/'target/frame.png').write_bytes(b'output')
            self.assertEqual(original, source_identity(root))
            source.write_text('after')
            self.assertNotEqual(original['content_sha256'],source_identity(root)['content_sha256'])


if __name__ == '__main__':
    unittest.main()
