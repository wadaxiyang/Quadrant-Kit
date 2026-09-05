# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import base64
import copy
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from capture_gallery_baseline import png_info, reusable, source_identity


class CaptureReuseTests(unittest.TestCase):
    def test_reuse_rejects_other_page_preview_theme_source_and_environment(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)/'sample.png'
            path.write_bytes(base64.b64decode('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jw1kAAAAASUVORK5CYII='))
            scene = dict(page=4, preview=1, theme='light', content_sha256='original', renderer='software', font_policy='system', scale_percent=100)
            record = dict(scene=scene, image=png_info(path))
            self.assertTrue(reusable(record, scene, path))
            for key, value in [('page',5),('preview',2),('theme','dark'),('content_sha256','edited'),('renderer','skia'),('font_policy','changed'),('scale_percent',200)]:
                changed = copy.deepcopy(scene)
                changed[key] = value
                self.assertFalse(reusable(record, changed, path), key)
            path.write_bytes(path.read_bytes()+b'changed')
            self.assertFalse(reusable(record,scene,path))
            path.unlink()
            self.assertFalse(reusable(record,scene,path))

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
