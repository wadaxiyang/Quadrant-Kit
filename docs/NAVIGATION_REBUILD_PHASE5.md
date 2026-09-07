# Navigation rebuild Phase 5 — unified pages and specimens

Date: 2026-09-07. Starting commit:
`b4d6ff08ecc9fd3e3bdbee0fee16549eaa0c57df` (Phase 4).

## Implementation

All eight existing destinations were transferred together to the fill-viewport
GalleryPage contract. GalleryPage owns the only page-level ScrollView and its
padding; the Gallery shell now only sizes the active page under search feedback.
No page supplies a competing full-page background. NavigationView and its content
surface are unchanged, and component-local menus/editors keep their own scrolling.
The page explicitly binds scroll content height to the larger of its viewport
and preferred document height, preventing wrapped cards from being compressed.

Gallery-local metrics define 40 px horizontal padding, reduced to 24 px below
700 px of page width, 32 px top padding and section spacing, and 40 px bottom
padding. Shared headers provide title, description, optional stability, and
Documentation/Source actions. The actions reveal local guidance and the existing
page integration fragment; Theme and C/M/W remain in the shell toolbar. Page
metadata and one children slot avoid assumptions about multiple named slots.

All 23 specimens use one preview-card layout and a separated source/details row.
Each has an illustrative snippet; keyboard/accessibility evidence moves into
the collapsed body while live counters remain visible. Conditional construction
removes the body and editor Tab stops when collapsed. The toggle supports
pointer, Tab, Enter and Space, focuses itself and exposes expanded state.
CodeSample uses a read-only standard TextEdit with selection and a bounded
180 px editor viewport. These are static integration fragments, not generated
representations of the current live values or a new syntax editor.

The preview remains mounted across detail toggles. Navigation still recreates
page-local state; host-bound text/switch state retains its prior lifetime. No
page cache, router change, new destination or snapshot/configuration interface
was introduced.

Native narrow-window review also corrected a three-column Controls property
layout and wrapping of ModalTextSample titles. Feedback's two text samples stack
below 500 px of available specimen content width. Specimen headers now derive height
from their wrapping text, preventing descriptions from overlapping previews.
These are Gallery layout corrections; the Kit public contract is untouched.

## Verification

All six common gate commands passed against the final implementation, serially:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS, 13 tests (1 root + 12 Gallery) |
| `python scripts/check_ui_boundaries.py` | PASS, 35 exports, 65 static files, 32 SVGs |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS, 36 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS |

Local logs and `results.json` are under ignored
`target/navigation-phase5/final-approved/`. Final runtime captures are in
`final-approved-captures/`: all eight pages at 1040×800 and 760×520 in Light/Dark
(32 scenes), plus Typography at 1040×1200 to check natural short-page height.
All 33 processes exited successfully; PNG dimensions, hashes, unchanged source
identity and binary identity were verified. Visual review covered every minimum
page, representative normal pages, long content and the short-page case. This is
render/interaction evidence, not an automated pixel-diff acceptance threshold.

Environment: Windows 11 build 26200, Slint 1.17.1, `winit-software`, scale 1,
Segoe UI Variable Text host font policy. Capture metadata records the starting
commit above with a dirty tree, content SHA-256
`85f39b69e5cdf0adac7bbe35c50c00309781cdc870de924ac6e5ca1136ebbc96`
and executable SHA-256
`cf115bb76e76f4f971592848f3d50f768100dbf60c1b0bc10ae8128667330ed2`.
This report was finalized afterward; no implementation changed after capture.
Earlier `initial-captures/`, `final-captures/` and `final-height-captures/` retain
intermediate observations and do not replace the final matrix.

`ownership-checks.json` checks eight GalleryPage destinations, one shared
page-level ScrollView, no shell/destination scroll wrapper, and 23 snippets.
It is a focused source-structure check, complemented by Slint compilation and
native interaction rather than a claim of a complete Slint parser.

Native observations `native/00-*` through `18-*` cover 1040×800 Light/Dark:
IconButton counter activation and preservation across details, pointer expansion,
Tab into source, Ctrl+A selection, attempted edits leaving source unchanged,
reverse Tab, Space collapse, Tab skipping the removed editor, Enter expansion,
switching to Controls at the top of its own viewport, live text changing the
button label, page Documentation/Source actions, Dark source contrast, Back
recreating the Icons counter at zero, and host-bound text surviving page changes.

Observations `19-*` through `26-*` cover the 760×520 Dark viewport: page scrolling,
source expansion and keyboard focus into a wrapped editor, reaching Feedback's
bottom specimen, and opening/dismissing the informational modal. This pass found
the last two narrow layout issues described above; final observations recheck
their corrections. Modal checks are callback/overlay evidence, not proof of
focus containment, restoration or screen-reader behavior.

Observations `27-*` through `33-*` recheck the explicit page content-height
binding, Controls' two-column properties, source expansion/Space collapse, and
page switching at 760×520 Dark. Feedback review identified an uneven two-column
text allocation, then `34-*` and `35-*` verify the final stacked narrow layout
with full readable titles and messages. `native-final-source.json` records the
same source/binary identity as the final 33-scene matrix. Earlier native runs
retain their original identities; the final layout changes do not alter input
handlers, source-editor behavior or callback/state lifetimes.

## Scope and limits

Kit implementation, public API baseline, Rust host/controller, root source
locator, Cargo manifests/lockfile, toolchain, icons and licenses remain unchanged.
The public contract remains 35 names, 240 declared properties, 20 callbacks,
seven enums and one ten-field struct. No baseline was refreshed.

NOT_RUN: package/archive, publication/remote CI, fresh external consumer, MSRV,
incremental token/SVG mutation, Linux/macOS runtime, full DPI/backend matrix,
IME, live OS theme transitions and screen-reader action dispatch. Existing focus
and accessibility limitations remain. No push, consumer retargeting or Tasks
changes occurred. Phase 6 has not started.
