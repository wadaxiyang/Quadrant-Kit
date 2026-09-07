# NavigationView / Gallery rebuild — Phase 1

Date: 2026-09-07. Status: **PASS — standalone navigation foundation complete**.
Authority: [rebuild SPEC](../Quadrant_Kit_NavigationView_Gallery_Rebuild_SPEC.md),
sections 2.3, 18.2 and 23 Phase 1. Predecessor: [Phase 0](NAVIGATION_REBUILD_PHASE0.md).
This is the navigation rebuild phase numbering, not the historical extraction phases.

## Delivered contract

- Added `NavigationPaneMode`, `NavigationContentSurfaceMode`, `NavigationEntryKind`
  and the ten-field `NavigationEntry` in `ui/patterns/navigation/navigation_types.slint`.
- Added seven semantic Theme aliases and four UiConstants declarations. Existing
  sidebar tokens and every previously declared API signature/default are retained.
- Added public `NavigationBackButton` and `NavigationPaneToggleButton`: 40 px
  controls with 20 px source-drawn/existing-asset icons, names, disabled state,
  focus forwarding, mouse and Enter/Space callbacks. Activation focuses the
  underlying IconButton. The host owns pane mode and conditionally mounts controls.
- Added public `NavigationContentSurface`: Fluent draws the content background,
  only a top-left round corner, and an open top/left border; Flat draws a square
  background; Transparent leaves the host background visible. Children retain
  the logical viewport without implicit padding, scrolling or routing.
- Exported all seven names through `@quadrant-kit`, compiled them in the existing
  API probe and demonstrated the real components at the top of Navigation page 7.
  The specimen includes visibility/enabled switches, callback counters, host-owned
  compact mode and three surface modes, with 280/480/720 px preview widths.
- Updated PUBLIC_API, VALIDATION, GALLERY, README, CHANGELOG, Overview counts and
  the exact-count boundary test in the same change.

The main Gallery shell, old SidebarItem implementation, Rust host, Cargo lockfile,
toolchain, SVG assets and licensing are unchanged. There is no NavigationView,
private tree row, hierarchy renderer, history/router state or alternate harness
in this phase. Their implementation starts at Phase 2.

## Reviewed API baseline

The candidate was emitted to `target/navigation-phase1/kit_api_candidate.json`.
The complete old/candidate diff was reviewed before deliberately adopting it as
`scripts/kit_api_v1.json`; the normal guard was then run against that baseline.
The guard implementation was not changed.

| Review item | Result |
|---|---|
| Old 28 exports | All signatures and defaults preserved |
| Additions | Seven exports, seven Theme aliases, four UiConstants properties |
| Current counts | 35 names: 21 components, six globals, seven enums, one struct |
| Declared members | 234 properties, 15 callbacks; NavigationEntry has ten fields |
| Static SVG closure | Same 32 assets |
| Old baseline SHA-256 | `8fbb4514c1d6afbf891e519456240660cbc539790d8bc1728d01b41db29e3702` |
| Adopted baseline SHA-256 | `f005fc82fe9691d6b44dc97a81e0abb0952399c07f2455b23eea6c9304784579` |

Machine-readable review evidence: `target/navigation-phase1/api-review.json`;
the original baseline is retained alongside the candidate for local comparison.
These ignored target artifacts are local evidence, not published package files.

## Validation

The final common gate ran serially after the last implementation correction.
Logs and per-command exit codes/timestamps are in `target/navigation-phase1/final/`.

| Command | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS — five Rust tests |
| `python scripts/check_ui_boundaries.py` | PASS — 35 exports, 63 static files, 32 SVGs |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS — 35 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS |

Native verification used Windows 11, Slint 1.17.1, `winit-software`, scale 1,
1040×800 logical client size and the host-selected Segoe UI Variable Text font.
Actions used the actual existing Gallery window; snapshots include UIA state
and, where appropriate, native window images.

| Acceptance | Evidence / result |
|---|---|
| Back mouse, Enter, Space | Counter 0 → 1 → 2 → 3; each action invokes once |
| Toggle mouse, Enter, Space | Each action increments once and host pane mode alternates |
| Tab / reverse Tab | Focus reaches Back and Toggle; focus ring visible in Light and Dark |
| Disabled mouse and keyboard | Neither callback fires; Tab skips disabled controls |
| Hidden individual controls | Remaining control occupies the first slot; hidden control absent from traversal |
| Both controls hidden | Entire 40 px row and its 12 px layout gap disappear; traversal skips both |
| Surface modes | Fluent / Flat / Transparent inspected in Light and Dark with visible child content |
| Normal / constrained frame | Fluent inspected at 480 and 280 px; top-left cutout only, other corners square |
| Existing shell / SidebarItem | Theme and preview controls remain usable; Preferences row remains visible and focusable |

Interaction evidence is in `target/navigation-phase1/native-final/00-*` through
`23-*`, with source/binary metadata in `native-source.json`. That run predates
the final surface-only border correction; the button implementations are unchanged.
The corrected surface and additional focused-disabled Toggle Enter/Space checks
are in `native-surface-final/00-*` through `14-*`, with metadata and implementation
hashes in `native-surface-source.json`. Use this latter set for final surface images.
UIA text can lag a rendered frame; screenshots and subsequent observations were
used to resolve that lag rather than issuing duplicate input.

The existing capture script also produced final raw Light images for previews
0 and 1 under:

```text
target/navigation-phase1/captures-final/
  dirty-48c30041be8a797335b2b8cd2ceff88b77331e00315a144c91b8ad6593632860/
    windows-winit-software/page-07_preview-{0,1}_light_1040x800_scale-100.png
```

Each image has the capture script's JSON sidecar. `pixel-checks.json` records
passing samples for both widths: top/left equal the border color (230,232,235),
right/bottom equal content white, and the top-left cutout equals the host background.
This is evidence for this renderer/scale, not a cross-platform DPI claim.

Two findings were fixed during verification: mouse activation now explicitly
focuses the button, and Fluent uses an open Path instead of a per-corner rectangle
whose straight borders were absent in the software capture. The Path viewbox
accounts for Slint's half-stroke fitting so it does not introduce aspect-fit insets.

## Source identity and remaining scope

Checked HEAD: `1a55fc971a7cc6415acb86299090fb169cc90ac4`. These changes are local,
uncommitted and unpublished. The user SPEC and Phase 0 report were already present
as known untracked files when Phase 1 began and were preserved.

The final implementation capture content identity is
`48c30041be8a797335b2b8cd2ceff88b77331e00315a144c91b8ad6593632860` (dirty checkout),
and its executable SHA-256 is
`4b19b645c57ce20b6081c582b9c05d041d707e87af9cd39ea886bf6be6c77382`.
This report and its documentation link were added afterward; implementation
hashes remain recorded separately. No dirty-content identifier is a Git revision.

NOT_RUN in this phase: package/archive verification, clean external consumer,
publication, MSRV, incremental rebuild mutation, Linux/macOS runtime, full DPI
matrix, screen-reader validation and complete accessibility/focus restoration.
A currently focused control can retain focus when disabled until Tab moves it;
callback suppression and traversal skipping were verified, automatic focus
restoration was not claimed.

All Phase 1 exit criteria are satisfied. Phase 2 can build the private hierarchy
and public NavigationView using these permanent facade exports; the main shell
switch and old SidebarItem removal remain in Phase 3 after Phase 2 evidence.
