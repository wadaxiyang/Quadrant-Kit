# NavigationView / Gallery rebuild — Phase 3

Date: 2026-09-07. Status: **PASS — main-shell cutover and SidebarItem removal**.
Authority: [rebuild SPEC](../Quadrant_Kit_NavigationView_Gallery_Rebuild_SPEC.md),
section 2.3 and Phase 3. Predecessor: [Phase 2](NAVIGATION_REBUILD_PHASE2.md),
commit `19f01eaab2334d442074b2d8cbc332d150d76809`.
This report uses navigation rebuild numbering, not historical extraction numbering.

## Delivered change

`gallery/ui/gallery.slint` now uses the public NavigationView for its main shell.
The old manual sidebar, Catalog filter, separate Pages heading/list and opaque
content rectangle are removed. A Gallery-owned top toolbar retains System/Light/
Dark and C/M/W utilities. Pane mode belongs to the host, while main-shell Back and
Search remain hidden until Phase 4 supplies their application behavior.

`gallery/ui/shared/gallery_catalog.slint` contains eight flat NavigationEntry
destinations and the host's string-ID-to-page mapping:

| Page | Stable ID | Destination |
|---:|---|---|
| 0 | overview | Overview |
| 1 | tokens | Theme tokens |
| 2 | typography | Typography |
| 3 | icons | Icons |
| 4 | controls | Controls |
| 5 | surfaces | Surfaces |
| 6 | feedback | Feedback |
| 7 | navigation | Navigation & shell |

The existing page ScrollView, padding and conditional page creation stay inside
NavigationView's content children. The page switch, sample bindings and modal
overlay logic are unchanged. Host pane width remains 216 px for this cutover;
NavigationView's public default remains 260 px, and compact width remains 54 px.
There is no new router, history, page cache or alternate runtime harness.

The Navigation page's legacy specimen/state was removed in favor of its existing
real NavigationView demonstrations. Its code sample now shows controlled
NavigationView usage; the API probe no longer instantiates SidebarItem. Overview
instructions/counts and the public documentation were updated to match.

Only after the replacement compiled and passed the native checks below were
`ui/patterns/navigation/sidebar_item.slint`, its facade export, Theme.sidebar_bg
and UiConstants.sidebar_collapsed_width/sidebar_expanded_width removed. New
navigation aliases now directly define the same transparent/54 px values.
Generic content_radius and all unrelated tokens are preserved.

Native cutover verification also exposed a pane-toggle tooltip clipped by the
left edge of the main window. NavigationPaneToggleButton now uses the builtin
popup Tooltip with TooltipHost content and preserves the button's accessible
name directly. Its public properties, defaults and callback are unchanged.

## Reviewed API removal

The candidate was emitted to `target/navigation-phase3/kit_api_candidate.json`.
The entire diff was reviewed, then checked against an exact expected transformation
of the Phase 2 baseline before deliberate adoption. The guard was not changed to
accept failures or refresh itself.

| Review | Result |
|---|---|
| Removed export | SidebarItem: six properties and one callback |
| Removed global properties | Three legacy sidebar tokens listed above |
| Equivalent default expressions | navigation_pane_bg → transparent; navigation_pane_compact_width → 54px |
| Other exports | 33 complete signatures/defaults unchanged, including all new navigation components/types |
| Current API | 35 names: 21 components, six globals, seven enums, one ten-field struct |
| Declared members | 240 properties, 20 callbacks |
| Previous baseline SHA-256 | `f9089440eef0e9c2856cf1833e7591f16284d591e6a72e06389b69a257bce036` |
| Adopted candidate SHA-256 | `52c85556dc14fa848bce51b2a0e7803b0221f99b38d2f15594edeb94c25eba22` |

Hashes identify the reviewed working-file bytes. `api-review.json` and a copy of
the old baseline are retained locally. All 35 declarations in PUBLIC_API were
parsed and compared successfully with the adopted baseline. A live-source scan
found no SidebarItem or legacy sidebar token references in Kit/Gallery; historical
phase reports, the SPEC and removal notes intentionally retain their names.

## Verification

Before deletion, `build-cutover.log` passed with the old component/export still
present and all Gallery consumers already migrated. Native evidence in
`target/navigation-phase3/native-cutover/00-*` through `18-*` demonstrates:

- All eight navigation entries open their corresponding page and focus the
  invoked row. Tokens needed a subsequent observation for the UIA tree to settle;
  no duplicate click was sent.
- C/M/W changes the Controls preview between Compact 320, Medium 560 and Wide 840.
- Main-pane expanded/compact changes preserve the current page. Mouse, Enter and
  Space activate Toggle; its accessible name and focus remain available.
- Selecting Field and collapsing Controls inside the specimen change its counters
  and local state without changing the selected main-shell Navigation route.
  Main-pane width and theme changes preserve that specimen state.
- Light, Dark and System theme controls remain usable. Selecting System resolved
  to the current light system state; a live operating-system theme change was not
  performed.

After removal and the tooltip fix, all six final commands passed serially. Logs,
exit codes, durations and timestamps are in `target/navigation-phase3/final/`.

| Command | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS — nine tests: one root, eight Gallery |
| `python scripts/check_ui_boundaries.py` | PASS — 35 exports, 65 static files, 32 SVGs; resolved Cargo checked |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS — 36 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS — existing Gallery and facade API probe |

The existing capture script ran Smoke for each page 0–7, producing eight final raw
PNG/JSON pairs at 1040×800 Light, 100%. All dimensions/hashes, source identities
and page IDs were checked; `capture-checks.json` records success. No screenshot
selector or capture implementation changed in this phase.

Final native observations in `native-final/00-*` through `12-*` cover 760×520:
Dark expanded/compact, full popup tooltip at the left edge, Toggle Enter/Space,
Tab and reverse Tab, Enter opening Tokens, Space opening Overview, switching to
Light, and navigating/scrolling the Navigation specimen until its primary/footer
regions are visible. The toolbar remains visible while page content scrolls.

All native checks used Windows 11, Slint 1.17.1, `winit-software`, 100% scale and
the existing host-selected Segoe UI Variable Text policy. No font or asset was
added. The initial native images record the pre-removal state; use final captures
and `native-final` for the removed API's final implementation.

## Source identity and phase boundary

Pre-removal native metadata is in `cutover-source.json`; its content identity is
`145bc10a2d15c9c55b16345dad13f99e2227e1fcec068777ee11c6a148ed4811`.
The final captures and native run share content identity
`5153aa9b8a178109ae64e01cb4f226108d716e9e0b852e7430599dbe61692328` and executable
SHA-256 `b96bf45ff40c6c1a007995d309e304ff0d5d2b8cafa0bae5754071e59c3f9589`.
Their checked HEAD is the Phase 2 commit above; these dirty-content hashes are
not Git revisions. This report was added after the successful implementation
checks and captures. Artifacts remain ignored under `target/navigation-phase3/`.

Root Rust, Gallery Rust/config, Cargo manifests/lockfile, toolchain, SVG bytes and
licensing are unchanged. This is a local breaking API change; no push, retained
tag update or consumer retargeting occurred. The published extraction source
documented in CONSUMER_GUIDE retains its historical API.

NOT_RUN: package/archive checks, publication/remote CI, a fresh external consumer,
MSRV, incremental token/SVG mutation, Linux/macOS runtime, full DPI/backend matrix,
IME, live OS theme changes and screen-reader action dispatch. Prior Phase 2 focus
and accessibility limitations remain; this cutover does not claim broader coverage.

Phase 3 exit criteria are satisfied. Phase 4 can enrich this same shell with a
hierarchical catalog, search and host-owned Back behavior. Phase 4 has not started.
