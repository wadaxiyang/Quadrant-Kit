# NavigationView / Gallery rebuild — Phase 0

Date: 2026-09-07. Scope: baseline, design lock and guard preparation only.
Authority: [NavigationView / Gallery rebuild SPEC](../Quadrant_Kit_NavigationView_Gallery_Rebuild_SPEC.md), sections 3, 21 and 23.
This phase numbering belongs to the navigation rebuild, not the historical extraction phases in VALIDATION.md.

## Source identity and working tree

- Checked HEAD: `1a55fc971a7cc6415acb86299090fb169cc90ac4`.
- Initial `git status --short`: only `?? Quadrant_Kit_NavigationView_Gallery_Rebuild_SPEC.md`.
  This is the user-supplied specification. It was preserved during the original
  baseline run and subsequently revised at the user's explicit request below.
  Tracked source was clean.
- Environment: native Windows x86_64 MSVC; Rust/Cargo 1.94.1; Python 3.14.6; pinned Slint/slint-build 1.17.1.
- API baseline SHA-256: `8fbb4514c1d6afbf891e519456240660cbc539790d8bc1728d01b41db29e3702`.
- Original SPEC SHA-256: `62273e3937918c4964b6f59af851cadb2d9e1de2481ee1439049d608e0aa8116`.
- The original Phase 0 run added this report and ignored logs under `target/navigation-rebuild-phase0/`.
  No UI/Rust behavior, dependencies, toolchains, assets, guard code or API baseline changed.
  No commit, publication or Tasks operation was performed.

## Reviewed architecture

All source files required by SPEC section 3 were read, together with AGENTS.md,
README.md, ARCHITECTURE.md, VALIDATION.md, CONSUMER_GUIDE.md, the Cargo manifests,
Gallery build/host code, and the boundary scanner and relevant fixtures.

The root crate only locates Slint sources. Gallery compiles `ui/kit.slint` as
`@quadrant-kit` with Fluent style and embedded resources. It owns windows,
theme/system synchronization, font selection and page state. Implementation
imports remain patterns/overlays → primitives → foundation, with acyclic
same-layer imports. Gallery cannot import private Kit paths.

The existing API baseline contains 28 names: 18 components, six globals and four
enums, with 217 explicitly declared properties and 13 callbacks. The compiled
`KitApiProbe` references all public names but is not instantiated on normal startup.

## Current navigation contract and dependency inventory

`SidebarItem` inherits `Rectangle`. Its complete explicitly declared contract is:

| Member | Direction/type | Declared default |
|---|---|---|
| `icon` | `in image` | none |
| `selected_icon` | `in image` | `root.icon` |
| `text` | `in string` | none |
| `icon_color` | `in color` | `Theme.icon_neutral` |
| `selected` | `in bool` | `false` |
| `collapsed` | `in bool` | `false` |
| `clicked` | callback, no arguments/return value | — |

Source-level behavior includes hover/selected backgrounds, a 3×16 px accent
indicator, regular/selected icons, a focus ring, Enter/Space activation, compact
hover tooltip and spacing/width/opacity animations. Accessibility declares a
labeled, checkable button. There is **no explicit enabled input or callback
suppression for disabled rows** in this component; that is new work required by
the target contract. Native behavior was not re-tested in Phase 0.

Direct implementation imports are `Motion`, `Theme`, `Typography`, `UiConstants`,
`FluentIcon` and `TooltipHost`. Both primitives depend only on foundation.
`SidebarItem` receives images from callers and owns no static SVG references.

Repository-wide search used `rg -n --hidden -g '!.git/**' -g '!target/**' 'SidebarItem|sidebar_item' .`.
At the checked source, the complete non-SPEC inventory is:

| File / baseline lines | Role |
|---|---|
| `ui/patterns/navigation/sidebar_item.slint:14` | Component definition |
| `ui/kit.slint:20` | Public facade export |
| `gallery/ui/gallery.slint:5,137` | Import and repeated main-menu row |
| `gallery/ui/api_probe.slint:6,32` | Import and compiled probe instance |
| `gallery/ui/pages/navigation_page.slint:5,24,32,115` | Import, specimen title, live instance and code sample |
| `docs/PUBLIC_API.md:13,368,370,373` | Coverage note, heading, source link and declaration |
| `scripts/kit_api_v1.json:1276` | Reviewed signature/default entry |
| `scripts/extraction_manifest.json:181,182` | Historical old/new source paths; provenance, not live imports |

The SPEC itself contains intentional references to the old API; this report
also records it historically. Future removal searches must distinguish live
dependencies from specification/provenance evidence rather than rewrite history.

Associated tokens:

- `sidebar_collapsed_width = 54px` and `sidebar_expanded_width = 200px` are declared
  in `ui/foundation/constants.slint:22,23`; their only live consumers are the
  Navigation page width expression at line 33. They do not control the main pane.
- `Theme.sidebar_bg = transparent` at `ui/foundation/theme.slint:34` has no live
  Kit/Gallery consumer. Both globals' declarations also appear in docs/API baseline.
- Existing navigation item height/padding/icon-slot/icon-size are 40/11/24/20 px.
  `content_radius = 8px` is shared with SurfaceCard and PreviewContainer; it must
  not be removed as though it were exclusively a sidebar token.

## Manual Gallery shell inventory

All main-shell mechanics are in `gallery/ui/gallery.slint`:

| Baseline lines | Current responsibility |
|---|---|
| 26–30 | Gallery-local `ShellLabel` |
| 47–48 | Integer `gallery_page` and `catalog_filter` state |
| 53–62 | Eight-entry flat catalog, IDs 0–7 and categories 1–4 |
| 71–86 | Horizontal shell, fixed 216 px left rectangle and right-edge divider |
| 88–156 | Pane-owned ScrollView, title, theme and preview utilities |
| 124–132 | Five category filter buttons; category selection also changes page |
| 134–147 | Separate Pages list; repeated SidebarItem with category visibility |
| 159–194 | Opaque right rectangle, shell-owned page ScrollView/padding and page switching |

`gallery/ui/shared/gallery_catalog.slint` defines the flat integer-ID/category
record. `gallery/src/main.rs:42` sets the initial numeric page; `gallery/src/config.rs`
validates page 0–7. Snapshot tooling and docs use the same page identifiers, so
future catalog migration must account for those entry points.

`GalleryPage` is already transparent, lays out title/description/stability and
`@children`, but owns neither scrolling nor page padding. `Specimen` inherits
SurfaceCard and places its heading, demo children and keyboard/accessibility
notes in one padded layout; it has no collapsible source panel. `PreviewContainer`
intentionally paints nested demonstration surfaces at 320/560/840 px requested
widths. Those preview surfaces are not the application content frame.

## Design lock and phase handoff findings

The target remains the SPEC's controlled, non-routing `NavigationView`: it owns
pane/content layout and optional Back/toggle/search/footer UI, supports depths
0/1/2 with selection independent of expansion, and emits requests. Hosts own
selection, hierarchy state, search logic, history and page creation. Content comes
through `@children`; NavigationView adds no page ScrollView. Fluent framing has
only top-left rounding and top/left borders; GalleryPage stays transparent and
will own page scrolling. Private row mechanics do not become public API.

No scanner extension is justified yet: existing fixtures already cover structs,
arrays, enums, callback arguments, private members, signature/default differences
and rejection of unknown syntax. Slint compilation and runtime checks are still
needed for actual model behavior and visuals.

### Resolved by the authorized SPEC revision — 2026-09-07

After the baseline report, the user explicitly requested that the SPEC itself be
corrected. The following decisions now replace the earlier proposals; they are
normative in the revised SPEC, not unresolved handoff questions:

1. **API baseline timing — resolved in section 2.3.** Review and adopt the scoped
   candidate in every phase that changes public declarations. Phase 1 and Phase 2
   adopt additions; Phase 3 adopts removals. All phases exit with the normal guard
   passing. The earlier suggestion to carry expected guard failures is superseded.
2. **Private specimen access — resolved by Phase 1/2 reallocation.** Phase 1
   exports the approved types/tokens and three independently useful components:
   Back, Pane Toggle and Content Surface. Gallery demonstrates those through the
   facade. Private rows/hierarchy move to Phase 2 and are exercised through public
   NavigationView. No special harness, test export or import-boundary exception
   is needed. Three-level acceptance moves with its real implementation.
3. **Main shell deletion order — resolved in Phase 3.** Switch the complete old
   shell to NavigationView with a temporary flat catalog of the eight real pages;
   remove Catalog filter, preserve utilities in a Gallery toolbar, and migrate
   all specimens/probes before deleting SidebarItem. Phase 4 enriches that shell
   with hierarchy, search and Gallery-owned Back.

The revision also supplies a common six-command gate for every phase; defines
controlled-state/default/compact/invalid-model behavior; keeps hidden descendants
out of focus from Phase 2; transfers page scrolling atomically in Phase 5; migrates
stable route IDs and snapshot aliases in Phase 6; preserves historical references;
and adds actual clean-source package/archive validation to Phase 8. Native chrome
remains a valid default, with custom chrome optional in Phase 7.

Section 10.3 also corrects the original single-corner premise: local pinned
`i-slint-compiler-1.17.1/builtins.slint` and `passes/border_radius.rs` define and
handle individual corner radii. The revised frame guidance permits those directly
and retains clipping/Path options for removing right/bottom border edges. This is
source inspection, not a claim of a newly rendered content-surface implementation.

These are specification changes only. They do not claim that the new components,
scenario selectors or future validation gates have been implemented or passed.
The original documents were saved under ignored `target/navigation-spec-revision/`
before editing so the amendment can be compared without relying on untracked Git
diffs. No unexpected working-tree changes were found.

Revision verification: the boundary/API/dependency check passed again with 28
exports, and all 35 Python tests passed. New diff whitespace was checked against
the saved originals. UI/Rust sources and the API baseline remain unchanged.
Cargo build/tests and native rendering were NOT_RUN for this documentation-only
amendment; the six-command results below belong to the original Phase 0 run.

## Validation results

All six required commands completed with exit code 0 on 2026-09-07, from
11:01:29 through 11:03:27 Asia/Shanghai. Commands ran serially against the current
checkout/build directory. Logs and per-command timestamps/exit codes are in
[`target/navigation-rebuild-phase0/results.json`](../target/navigation-rebuild-phase0/results.json).

| Command | Result | Log under `target/navigation-rebuild-phase0/` |
|---|---|---|
| `cargo fmt --all --check` | PASS | `fmt.log` |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS | `clippy.log` |
| `cargo test --workspace --locked` | PASS: 1 helper + 4 Gallery tests; 0 doctests | `rust-tests.log` |
| `python scripts/check_ui_boundaries.py` | PASS: 28 exports, 59 static files, 32 SVGs, resolved Cargo checked | `boundaries.log` |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS: 35 tests | `python-tests.log` |
| `cargo build --locked -p quadrant-kit-gallery` | PASS: Gallery and compiled API probe | `gallery-build.log` |

NOT_RUN in this phase: standalone package/archive verification and package-list
command, incremental token/SVG mutation, separate MSRV build, Linux/macOS checks,
remote CI/consumer verification, native GUI interaction, screenshots, DPI and
accessibility matrix. Boundary verification reported `package_checked: false`;
it is not evidence of a newly verified `.crate` archive. Earlier documented
native/platform results are historical and are not promoted to Phase 0 results.

Phase 0 exit criteria are satisfied: tracked implementation baseline is clean,
required checks pass, all current SidebarItem references are inventoried, and the
API baseline is unchanged. Work stops here under SPEC section 23.7; Phase 1 has
not begun.
