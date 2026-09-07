# Navigation rebuild Phase 6 — catalog and component destinations

Date: 2026-09-07. Starting commit: `7aa2e03f6b142de3537fa5518a7599eebaebcd2d`.

## Implementation

`gallery/catalog.tsv` is the single Gallery-local catalog. Rust validates and
embeds it for navigation, titles, search, history and route resolution; Python
reads the same metadata for snapshot selection. All components is populated by
the Rust controller from the catalog's unique public visual export owners. The
25 typed page branches are checked against catalog IDs, files and component names.
The API baseline comparison requires exactly all 21 public visual components,
with no duplicate ownership or unimplemented page. No Kit router was introduced.

Home replaces Overview. All components provides the complete public visual index.
Controls overview keeps cross-component input/selection comparisons and the CI
page-4 contract; SurfaceCard keeps its surface-state overview; Feedback overview
compares outcomes and decisions. Focused component pages reuse extracted existing
Button, IconButton, Toast, Modal and window compositions. Foundation/global
concepts remain grouped. Navigation foundations intentionally groups the three
standalone navigation primitives; NavigationView is a dedicated destination.
The old Navigation aggregate is removed after moving its demonstrations.

The NavigationView page retains configurable Back visibility/enabled state,
Search, footer, toggle, title, compact/expanded mode, one/two/three levels,
destination groups and Fluent/flat/transparent surfaces. Its callback counters,
invalid-model fixtures and Light/Dark host remain intact.

Stable string destination IDs are canonical. Optional numeric aliases are exactly
0=home, 1=tokens, 2=typography, 3=icons, 4=controls, 5=surfaces, 6=feedback,
7=navigation-view. Missing route options select Home. Runtime, Python and
PowerShell reject explicit alias/destination conflicts and unknown IDs. Capture
schema 2 stores the resolved destination and rejects schema-1 reuse. Catalog mode
covers every destination; the page-4 Smoke path still exercises the numeric host
alias. Windows CI retains that call and adds three string-destination smoke calls.

## Verification

All six common gate commands passed against the final implementation, serially:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS, 16 tests (1 root + 15 Gallery) |
| `python scripts/check_ui_boundaries.py` | PASS, 35 exports, 65 static files, 32 SVGs |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS, 42 tests, no skips |
| `cargo build --locked -p quadrant-kit-gallery` | PASS |

Logs and results are under ignored `target/navigation-phase6/gate/`. Route tests
cover every destination, all eight aliases, invalid IDs, category IDs, conflicts,
catalog structure/unique ownership, exact public visual coverage and typed page
dispatch. Capture tests cover distinct identities, schema-1 rejection and inherited
environment isolation. PowerShell 7 negative cases actually ran locally.

The final capture run is under `target/navigation-phase6/final/`:

- Catalog mode: all 25 destinations at 1040x800 and 760x520, Light/Dark, 100% scale
  (100 successful scenes). PNG dimensions and hashes, schema 2, complete destination
  coverage and unchanged source identity were checked.
- New string-interface Smoke: Home, All components and NavigationView.
- Legacy Smoke: the PowerShell wrapper with `-Page 4`, exercising the numeric
  environment alias in the host. A subsequent `--destination controls
  --reuse-existing` reused that exact scene successfully.
- Representative visual review covered Home, the index, navigation, editors,
  buttons, icons, surfaces and new reference pages at normal/minimum sizes.
  The final ModalManager Light/Dark normal captures verify equal-width text
  samples; the minimum capture verifies that they still stack.

This is render/interaction evidence, not a pixel-diff acceptance threshold.
Environment: Windows 11 build 26200, Slint 1.17.1, winit-software, scale 1,
Segoe UI Variable Text host font policy. Final manifests record the starting
commit above with a dirty tree, content SHA-256
`e8200930084c691ffb631f605ce64a6c640812d714c97d640d9c6659c34424dd`
and executable SHA-256
`87d434712de71abb4d8213928dcab3738b4b27f328a5e4e899173a4dadad3b06`.
This report was finalized afterward; implementation did not change after the
final capture. Earlier captures outside `final/` retain intermediate evidence.

Native observations `native/00-*` through `39-*` cover:

- Home to All components, index card to IconButton, and Back through the same
  history controller; the index exposes all 21 component links.
- IconButton callback count retained across source expansion and Space collapse.
- FluentTextField editing and Enter callback; host-bound text retained when
  switching to FluentTextArea.
- Search filtering and Enter navigation to NavigationView and ModalManager.
- Expanded/compact navigation; Back/Search/footer removal; one/two/three levels;
  destination-group selection versus separate chevron expansion; Light/Dark and
  Fluent/flat/transparent surfaces. Enabled Back increments its counter once;
  disabling it exposes disabled state with the counter unchanged.
- The dedicated ModalManager page opens its informational overlay and Close
  dismisses it through the host callback.

`native/native-source.json` records this interaction pass's content hash
`36ddfcda3752eb5ee3e278ccc97ebe1c8c0cef5cd4e0e4a6cd1d819a58c3b985`
and binary hash `a700f67082ca565b11144496dea78a8813c5c4f6753a10687d7d49438e186e22`.
That pass found the uneven normal-width Modal text columns. The final correction
only sizes the two sample cards equally (retaining narrow stacking), with no
callback, state-lifetime or Kit change; all six gates and the complete capture
run were repeated afterward. Icons also use a two-column narrow catalog grid.

## Scope and limits

Kit source, public API baseline, root Rust locator, Cargo manifests/lockfile,
toolchain, resources, attribution and licenses are unchanged. No API baseline
was refreshed. No publication, remote CI execution, consumer retargeting or Tasks
changes are part of this phase.

NOT_RUN: publication/package/archive, fresh external consumer, MSRV, incremental
mutation checks, Linux/macOS native runtime, full DPI/backend/monitor transitions,
IME, Windows PowerShell 5.1 and screen-reader action dispatch. Modal verification does not establish
complete focus containment or restoration. Phase 7 has not started.
