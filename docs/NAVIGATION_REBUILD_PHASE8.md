# Navigation rebuild Phase 8 — final local validation

Date: 2026-09-07. Predecessor: `ec7685b98044328ecda03ce3e99f05a6484de0d1`.
Authority: the root NavigationView/Gallery rebuild SPEC, Phase 8.

## Finalization

README, PUBLIC_API, GALLERY, CONSUMER_GUIDE, VALIDATION and CHANGELOG now describe
the current catalog, keyboard behavior and unpublished navigation API. The
published extraction candidate and its historical platform/consumer evidence
remain distinct from the current local implementation.

The public baseline is unchanged from the explicit Phase 3 review: 35 names,
240 properties, 20 callbacks, 21 components, six globals, seven enums and one
ten-field struct. No candidate baseline was generated or adopted in this phase.
Two repository tests now compare documented declarations/defaults and probe
coverage with the facade, reconcile counts, and prevent live legacy navigation
identifiers from returning. The existing catalog test covers all 21 visual
exports and their typed routes; there are 25 destinations and eight numeric aliases.

Legacy-name searches found no live SidebarItem implementation, import, instance,
public declaration or runnable example, and no old Catalog filter shell. Remaining
references in the SPEC, changelog, historical phase reports, removal descriptions
and extraction manifest are intentional historical provenance.

No Slint, Gallery runtime, public API, Cargo manifest/lockfile, toolchain, root
Rust locator, resource bytes, asset baseline or licensing changes are included.
The Phase 7 native input observations and 184 render scenes therefore cover the
same UI/runtime source; Phase 8 does not claim a new independent native run.

## Verification

**PASS — required local Phase 8 gates are satisfied.**

All twelve commands below exited 0 against clean committed source
`e933c8bed57c257dd7d7d8ae382a74e3cc9d5899`, on Windows 11 x86_64 MSVC.
Development Rust remains 1.94.1, MSRV is 1.92.0, and Slint remains 1.17.1.
The checkout was clean before and after each command; incremental verification
was run serially with exclusive checkout/build access and no open Gallery.

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS: 17 tests (1 Kit, 16 Gallery) |
| `python scripts/check_ui_boundaries.py` | PASS: 35 exports, 65 static files, 32 SVG assets |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS: 46 tests, no skips |
| `cargo build --locked -p quadrant-kit-gallery` | PASS, including KitApiProbe |
| `python scripts/verify_distribution.py --package` | PASS: Git-tracked static closure included |
| `cargo package --locked -p quadrant-kit --list` | PASS |
| `cargo package --locked -p quadrant-kit` | PASS: 87 files, package verification build passed |
| `python scripts/verify_distribution.py --package --archive target/package/quadrant-kit-0.1.0.crate` | PASS: 65 required files byte-verified |
| `cargo +1.92.0 build --locked -p quadrant-kit -p quadrant-kit-gallery --target-dir target/msrv-1.92` | PASS: actual Windows helper/Gallery build, 185 seconds |
| `python scripts/verify_incremental.py` | PASS: token and SVG invalidation, 278 seconds |

No failed required gate was waived and no `--allow-dirty` package was used.
Command arrays, timings, exit codes and clean-source checks are retained in
ignored `target/navigation-phase8/results.json`, with individual command logs.

### Archive and licenses

The archive at the checked source has SHA-256
`21253b60ce831e4e03e8f912010d130c332e8e08820c4cc47f9c6d8417be2db8`.
In addition to the distribution guard, a local audit compared all 84
source-derived archive files with their tracked working-tree bytes, including
the original Cargo manifest, root locator, API baseline, documentation, GPL text,
MIT text, notices and assets. All matched. The remaining three files are Cargo's
generated Cargo.toml, Cargo.lock and .cargo_vcs_info.json; the latter identifies
the exact clean commit above. Cargo's package verification compiled the locator.
No Gallery, target output, local machine configuration or bundled system font
was added to the package.

This report is finalized in a subsequent documentation-only commit. Package
generation, archive verification and the 84-file audit are repeated on that final
clean snapshot so the package also contains the completed report. Final package
identity is stored outside the commit in ignored
`target/navigation-phase8/archive-audit-<full-commit>.json`; the earlier hash above
continues to identify the twelve-command gate's source, not the final report bytes.

### Incremental and existing interaction evidence

Changing the deep Theme accent token and changing the referenced navigation SVG
each reran the Gallery build script and changed its executable hash. Each original
file was restored byte for byte and the restored source rebuilt. The final Git
status was clean. Logs and build-output timestamps are in
`target/incremental-verification/`; the result is also retained in
`target/navigation-phase8/incremental-result.json`. This verifies invalidation
and restoration, not reproducible binary bytes or pixel equivalence.

The initial Phase 8 Gallery executable hash was
`df0c9ca7a60b874621fecdb2d5535084fbf6efadd362a579d3c8197be4652303`,
identical to the executable used for the [Phase 7](NAVIGATION_REBUILD_PHASE7.md)
184-scene render matrix. A diff from `ec7685b` confirmed no UI, Gallery runtime,
manifest/lockfile, asset or public-baseline changes. Those identified native input
and render observations remain the available-host acceptance evidence for this
implementation; they are not counted as 184 newly executed Phase 8 captures.

The common gate, actual clean archive, current-host MSRV and exclusive incremental
checks are complete. Documentation, probe, catalog and unchanged API baseline
agree. Local construction is complete within the limits below.

## Scope

This is local construction, not a release or a Tasks adoption. Remote CI, retained
references, anonymous external-consumer verification and publication are NOT_RUN
for this rebuild. Linux/macOS native runtime, real monitor DPI transitions, full
backend coverage, IME and screen-reader action dispatch remain NOT_RUN for the
current UI. Up/Down/Home/End item traversal is not implemented; modal focus
containment/restoration is unclaimed. Native decorations are retained; custom
chrome remains deferred. Historical extraction evidence is not relabeled.
