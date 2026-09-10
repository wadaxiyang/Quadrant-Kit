# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import io
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from verify_package_consumer import extract_files
from run_motion_bench import parse_memory


class PackageConsumerTests(unittest.TestCase):
    def test_extraction_rejects_escape_and_links(self):
        for name, link in (('../outside',False),('/absolute',False),('package/link',True)):
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                root=Path(directory)
                archive=root/'case.crate'
                with tarfile.open(archive,'w:gz') as package:
                    member=tarfile.TarInfo(name)
                    if link:
                        member.type=tarfile.SYMTYPE
                        member.linkname='../outside'
                    package.addfile(member,io.BytesIO(b''))
                with self.assertRaises(ValueError):
                    extract_files(archive,root/'extracted')
                self.assertFalse((root/'outside').exists())

    def test_lifecycle_memory_requires_all_checkpoints(self):
        lines=[f'MEMORY sample={i} private_bytes=100 working_set_bytes=200' for i in range(19,200,20)]
        lines.append('MEMORY_FINAL private_bytes=100 working_set_bytes=200')
        self.assertEqual(len(parse_memory('\n'.join(lines))['cycles']),10)
        with self.assertRaises(ValueError):
            parse_memory('\n'.join(lines[1:]))
