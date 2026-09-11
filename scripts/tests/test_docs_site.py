# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import importlib.util
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('docs_site_hooks', ROOT / 'docs-site/hooks.py')
hooks = importlib.util.module_from_spec(spec)
spec.loader.exec_module(hooks)


class DocumentationSiteTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.docs = self.root / 'docs'
        self.docs.mkdir()
        (self.docs / 'PUBLIC_API.md').write_text('# API', encoding='utf-8')
        (self.root / 'README.md').write_text('# Root', encoding='utf-8')
        self.config = {'docs_dir': str(self.docs), 'repo_url': 'https://github.com/owner/kit'}
        self.page = SimpleNamespace(file=SimpleNamespace(src_uri='PUBLIC_API.md'))

    def render(self, source):
        return hooks.on_page_markdown(source, self.page, self.config)

    @patch.dict('os.environ', {'GITHUB_SHA': 'a' * 40})
    def test_repository_links_use_build_revision_and_keep_fragments(self):
        self.assertEqual(self.render('[root](../README.md#root)'),
                         '[root](https://github.com/owner/kit/blob/' + 'a' * 40 + '/README.md#root)')

    def test_docs_external_anchor_and_fenced_examples_stay_unchanged(self):
        source = '[API](PUBLIC_API.md#api) [here](#api) [web](https://example.org)\n```slint\n[example](missing.md)\n```'
        self.assertEqual(self.render(source), source)

    def test_missing_or_outside_repository_targets_fail(self):
        for link in ['missing.md', '../../outside.md']:
            with self.subTest(link=link), self.assertRaises(ValueError):
                self.render(f'[invalid]({link})')
