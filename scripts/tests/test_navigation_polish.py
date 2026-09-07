# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import os
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from capture_navigation_polish import cells, environment, key


class NavigationPolishTests(unittest.TestCase):
    def test_complete_unique_matrix(self):
        scenes = cells()
        self.assertEqual(len(scenes), 184)
        self.assertEqual(len({key(c) for c in scenes}), 184)
        for size in [(1040, 800), (760, 520)]:
            for theme in ['light', 'dark']:
                for compact in [False, True]:
                    standard = [c for c in scenes if c['host'] == 'navigation-validation' and
                                (c['width'], c['height']) == size and c['theme'] == theme and
                                c['compact'] == compact and c['variant'] == 0 and c['case'] == 0]
                    self.assertEqual({c['scale'] for c in standard}, {100, 125, 150, 200, 225})
                    optional = [c for c in scenes if c['host'] == 'navigation-validation' and
                                (c['width'], c['height']) == size and c['theme'] == theme and c['compact'] == compact]
                    self.assertEqual({c['variant'] for c in optional}, set(range(8)))

    def test_environment_isolates_harness_from_gallery(self):
        with patch.dict(os.environ, {'QUADRANT_GALLERY_NAV_VALIDATION':'7', 'QUADRANT_GALLERY_PAGE':'4'}):
            for cell in cells():
                env = environment(cell, Path('output.png'))
                self.assertNotIn('QUADRANT_GALLERY_PAGE', env)
                self.assertEqual(env['SLINT_SCALE_FACTOR'], str(cell['scale']/100))
                self.assertEqual(env.get('QUADRANT_GALLERY_NAV_VALIDATION'), str(cell['variant']) if cell['host'] == 'navigation-validation' else None)
