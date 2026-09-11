# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
"""Keep docs/ as the source; link repository-only files to the corresponding Git revision."""
from pathlib import Path
import os
import re
from urllib.parse import quote, unquote, urlsplit


def on_page_markdown(markdown, page, config, **kwargs):
    docs = Path(config['docs_dir']).resolve()
    root = docs.parent
    source = docs / page.file.src_uri
    revision = os.environ.get('GITHUB_SHA', 'main')
    if not re.fullmatch(r'[a-f0-9]{40}|main', revision):
        raise ValueError('Expected a full Git SHA or main for repository source links')

    def rewrite(match):
        link = urlsplit(match[2])
        if link.scheme or link.netloc or not link.path:
            return match[0]
        target = (source.parent / unquote(link.path)).resolve()
        if not target.is_relative_to(root) or not target.exists():
            raise ValueError(f'{page.file.src_uri}: missing/outside repository link {match[2]}')
        if target.is_relative_to(docs):
            return match[0]
        relative = quote(target.relative_to(root).as_posix(), safe='/')
        url = f'{config["repo_url"]}/blob/{revision}/{relative}'
        if link.fragment:
            url += '#' + link.fragment
        return f'[{match[1]}]({url})'

    # Fenced examples stay byte-for-byte intact, including any illustrative links.
    segments = re.split(r'(^```[^\n]*\n.*?^```[^\n]*$)', markdown, flags=re.M | re.S)
    for index in range(0, len(segments), 2):
        segments[index] = re.sub(r'\[([^\]\n]*)\]\(([^)\n]+)\)', rewrite, segments[index])
    return ''.join(segments)
