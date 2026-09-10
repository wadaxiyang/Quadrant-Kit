# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
import io
from pathlib import Path
import sys
import tarfile
import tempfile
import tomllib
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from slint_contract import parse
from verify_package_consumer import extract_files
from verify_remote import KIT_URL, SLINT, check_resolution, generate_consumer, isolated_environment, validate_request


class RemoteVerificationTests(unittest.TestCase):
    def test_requires_public_url_full_sha_and_retained_tag(self):
        validate_request(KIT_URL, 'a' * 40, 'refs/tags/candidate/extraction-aaaaaaa')
        for url, rev, ref in [('file:///kit', 'a' * 40, 'refs/tags/candidate/extraction-aaaaaaa'),
                              (KIT_URL, 'main', 'refs/tags/candidate/extraction-aaaaaaa'),
                              (KIT_URL, 'a' * 40, 'refs/heads/main')]:
            with self.subTest(url=url, rev=rev, ref=ref), self.assertRaises(ValueError):
                validate_request(url, rev, ref)

    def test_environment_does_not_inherit_source_or_credential_overrides(self):
        original = {'PATH': 'tools', 'HTTPS_PROXY': 'http://proxy.invalid', 'CARGO_HOME': 'old',
                    'CARGO_REGISTRIES_CRATES_IO_TOKEN': 'fixture', 'GIT_CONFIG_COUNT': '1',
                    'GIT_CONFIG_KEY_0': 'url.local.insteadOf', 'RUSTFLAGS': 'override',
                    'GITHUB_TOKEN': 'fixture', 'RUSTC_WRAPPER': 'wrapper'}
        env = isolated_environment(Path('/isolated'), original)
        self.assertEqual(env['PATH'], 'tools')
        self.assertEqual(env['HTTPS_PROXY'], 'http://proxy.invalid')
        for key in ('CARGO_REGISTRIES_CRATES_IO_TOKEN', 'GIT_CONFIG_COUNT', 'GIT_CONFIG_KEY_0', 'RUSTFLAGS', 'GITHUB_TOKEN', 'RUSTC_WRAPPER'):
            self.assertNotIn(key, env)
        self.assertNotEqual(env['CARGO_HOME'], original['CARGO_HOME'])
        self.assertEqual(env['GIT_CONFIG_NOSYSTEM'], '1')

    def test_generated_consumer_uses_only_pinned_public_source(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'consumer'
            generate_consumer(path, KIT_URL, 'a' * 40)
            manifest = tomllib.loads((path / 'Cargo.toml').read_text(encoding='utf-8'))
            self.assertEqual(manifest['build-dependencies']['quadrant-kit'], {'git': KIT_URL, 'rev': 'a' * 40})
            self.assertEqual(manifest['dependencies']['slint'], '=1.17.1')
            self.assertEqual(parse(SLINT)['imports'][0][0], '@quadrant-kit')

    def test_actual_source_and_manifest_location_must_both_match(self):
        with tempfile.TemporaryDirectory() as temporary:
            cache = Path(temporary)
            kit = {'id': 'kit', 'name': 'quadrant-kit', 'source': f'git+{KIT_URL}?rev={"a" * 40}#{"a" * 40}',
                   'manifest_path': str(cache / 'git/checkouts/hash/Cargo.toml')}
            data = {'packages': [kit]}
            check_resolution(data, cache, 'a' * 40)
            with self.assertRaises(ValueError):
                check_resolution(data, cache, 'b' * 40)
            kit['manifest_path'] = str(cache / 'sibling/Cargo.toml')
            with self.assertRaises(ValueError):
                check_resolution(data, cache, 'a' * 40)


class LocalPackageVerificationTests(unittest.TestCase):
    def test_extraction_rejects_escape_and_links(self):
        for name, link in (('../outside', False), ('/absolute', False), ('package/link', True)):
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                archive = root / 'case.crate'
                with tarfile.open(archive, 'w:gz') as package:
                    member = tarfile.TarInfo(name)
                    if link:
                        member.type = tarfile.SYMTYPE
                        member.linkname = '../outside'
                    package.addfile(member, io.BytesIO(b''))
                with self.assertRaises(ValueError):
                    extract_files(archive, root / 'extracted')
                self.assertFalse((root / 'outside').exists())


if __name__ == '__main__':
    unittest.main()
