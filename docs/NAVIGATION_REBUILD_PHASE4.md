# Navigation rebuild Phase 4 — Gallery catalog, search and Back

Date: 2026-09-07. Starting commit:
`1bd5130ac4aa295f6bbcd9dfd0e7ea345cdf1569` (Phase 3).

## Implementation

The existing public NavigationView remains the single main shell. Gallery owns
the catalog, route validation, selected page, Back stack, query and expansion in
`gallery/src/navigation.rs`; `GalleryNavigation` connects that controller to Slint.
The eight original page numbers and stable IDs remain unchanged. Overview is a
root destination, Foundation contains Tokens/ Typography/ Icons, and Components
contains Controls, a nested Surfaces & feedback group, and Navigation. All
destinations resolve to implemented aggregate pages; page splitting waits for
Phase 6. Groups are expansion controls, not placeholder destinations.

Only a different valid destination pushes history. Repeated selection, invalid
IDs, group expansion and query edits do not. Back pops without pushing, reveals
ancestors and disables itself when the stack is empty. It retains the query when
the target is still among the results and clears it otherwise.

Search trims surrounding whitespace and matches titles/keywords without case
sensitivity. Results are flat root-depth destinations with the same IDs. Typing
does not navigate; Enter opens the first match in catalog order, and clicking a
result uses the same route table. No results is static feedback while the page
stays mounted. Clearing restores normal hierarchy and saved expansion; selecting
a filtered result does not overwrite expansion. Back intentionally reveals its
target's ancestors. Compact search requests the existing pane expansion action.

The toolbar, native decorations, Fluent surface and transparent pages remain.
Main-pane width stays 304 px. Native minimum-size checking exposed two existing
layout constraints: Overview now stacks its catalog tiles below 600 px of page
width, and the Navigation specimen separates crowded control rows and reserves
200 px for its inner content. The shell's scroll child has an explicit zero
minimum width to avoid a width/layout binding cycle. Scroll ownership and page
padding remain in the shell until the atomic Phase 5 transfer.

## Validation

Four Gallery tests cover catalog/startup validation, forward/repeated/invalid
selection and Back, flat keyword results and expansion preservation, and
submission/no-results/query retention. They supplement the existing tests.

All six final commands passed serially after the layout corrections. Logs, exit
codes and durations are under ignored `target/navigation-phase4/final-layout/`;
earlier successful checks are retained separately in `final/`.

| Command | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS — 13 tests: one root, 12 Gallery |
| `python scripts/check_ui_boundaries.py` | PASS — 35 exports, 65 static files, 32 SVGs; resolved Cargo checked |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS — 36 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS — Gallery and facade API probe |

Smoke capture ran for all pages 0–7, producing eight final PNG/JSON pairs at
1040×800 Light, 100%. Their PNG hashes, dimensions, page coverage and common
source/binary identities were checked; `capture-checks.json` records success.
The snapshot interface and capture implementation did not change.

Native observations `native/00-*` through `25-*` cover the controller before the
layout-only corrections: initial disabled Back, Tokens → repeated Tokens →
Feedback, collapsed Foundation revealed by Back, Enter returning to Overview
with an empty stack, collapsed Components preserved across MODAL search,
submission and clearing, no-result submission preserving Feedback, Back clearing
the hidden-target query, result clicks for Controls/Feedback, Back retaining the
matching components query, and compact search expanding the pane.

The 760×520 Dark run verifies the fixed toolbar/search and independently
scrollable catalog. Observations `26-*` through `29-*` recheck Overview's single
column and reaching Navigation by scrolling the pane. Final observations `30-*`
through `35-*` verify Navigation's readable control rows and inner content at
minimum size, Dark → Light, compact mode, Tab focusing compact Search and Enter
expanding the full search pane. Gallery was closed after validation.
All native runs use Windows, Slint 1.17.1, winit-software, 100% scale and the
existing Segoe UI Variable Text policy. Accessibility trees are interaction
evidence, not screen-reader acceptance. JSON observations are retained for all
steps; local JPEG screenshots start at observation 15.

Final captures and native observations 30–35 share source content identity
`3488541a1914a5d50d6cf80997879f70ebbfdbe5cdef70edd9c3645e9f44ade6`
and executable SHA-256
`130cae6a4d438bee716bccd6d45cf5476832d7175bf8e0bf30375f482317c63c`.
The checked HEAD is the Phase 3 commit above; content hashes are not Git
revisions. The earlier controller and Overview-only native passes have their
own source metadata. This report was finalized after checks and captures.

## Scope and limits

Kit source and its public baseline remain unchanged: 35 exports, 240 declared
properties, 20 callbacks, seven enums and one ten-field struct. No baseline was
refreshed. Root Rust, Cargo manifests/lockfile, toolchain, SVGs and licensing are
unchanged. No Tasks changes, publication, push or consumer retargeting occurred.

NOT_RUN: package/archive, publication/remote CI, fresh external consumer, MSRV,
incremental token/SVG mutation, Linux/macOS runtime, full DPI/backend matrix,
IME, live OS theme transitions and screen-reader action dispatch. Previous
focus/accessibility limitations remain. Phase 5 has not started.

Phase 4 exit criteria are satisfied; Phase 5 can transfer scroll/padding ownership
and rebuild GalleryPage/Specimen on this same working shell.
