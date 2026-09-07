# NavigationView / Gallery rebuild — Phase 2

Date: 2026-09-07. Status: **PASS — public NavigationView assembled and verified**.
Authority: [rebuild SPEC](../Quadrant_Kit_NavigationView_Gallery_Rebuild_SPEC.md),
sections 2.3, 7–8, 18–21 and Phase 2. Predecessor:
[Phase 1](NAVIGATION_REBUILD_PHASE1.md), committed together with the Phase 0/SPEC
sequencing work at `b21ca689e4fa2e06d7ca2c33f7745ab39d4ee27e`.
This is navigation rebuild numbering, not historical extraction numbering.

## Delivered behavior and files

- `ui/patterns/navigation/navigation_view.slint` assembles the Phase 1 controls,
  independent primary/footer ScrollViews and NavigationContentSurface with host
  children. Back, Toggle, Search, title and separators mount conditionally. Pane
  width clamps to available space; compact mode uses the existing 54 px token.
- Private `navigation_item_row.slint` renders depth 0–2 with 28 px indentation,
  selected-icon fallback, separate label/chevron actions, disabled guards, focus
  rings, Tab/Enter/Space and ancestor-qualified labels/tooltips. A collapsed
  descendant leaves layout/traversal; focus moves to a visible enabled ancestor
  or the pane fallback. Selection survives a collapse without selecting a parent.
- Private `navigation_model.slint` validates complete preorder trees in Slint.
  IDs must be nonempty and unique across both regions; parent/depth/child-kind and
  child flags must agree. Any malformed or oversized model rejects **both** menus.
  Optional controls and host content remain usable. Validation emits no diagnostic
  callback and does not repair models or introduce a Rust consumer dependency.
- `gallery/src/navigation_samples.rs` constructs valid/adversarial models only
  for the existing Gallery. Four tests use an independent stack validator to
  establish the fixtures' intended validity. Runtime captures exercise the actual
  Slint rejection policy; the Rust validator is not a substitute for that evidence.
- `gallery/ui/shared/navigation_view_specimen.slint` exposes host acceptance,
  selection/expansion state, six callback counters and optional-region switches.
  A second specimen exercises public defaults. Both use only `@quadrant-kit`.
  `api_probe.slint` compiles NavigationView and its host children through the facade.
- Gallery config/main and the existing capture script accept validated navigation
  case/compact selectors. Capture identity includes them, and reuse rejects a
  different fixture. There is one Gallery executable and one snapshot path.
- PUBLIC_API, GALLERY, VALIDATION, README, CHANGELOG, Overview count, the baseline
  and its exact-count test were updated together. The SPEC now states the actual
  compiler-imposed implementation size bound.

Kit owns no selection mutation, history, route creation or page cache. Gallery
accepts or declines requests. A disabled parent does not disable an enabled child.
Search uses the existing text wrapper: edits notify, Return submits, and host text
assignments do not echo an edit callback.

Slint 1.17.1 rejects recursive function calls. Eight explicit subdivision stages
support **256 entries per model**, independently of the three-level depth limit;
larger models fail closed before rendering. The private algorithms use bounded
scans (quadratic validation at worst). Raising the bound requires implementation
and validation review. Root Rust source-locator code, Cargo manifests/lockfile,
toolchain, SVG bytes and licensing remain unchanged.

## Reviewed public API

The candidate was emitted to `target/navigation-phase2/kit_api_candidate.json`,
compared in full against the retained previous baseline, then deliberately adopted.
The normal guard implementation was not weakened or refreshed automatically.

| Review | Result |
|---|---|
| Existing 35 exports | Every signature and default unchanged |
| Addition | NavigationView: exactly 15 properties and six callbacks |
| Current contract | 36 names: 22 components, six globals, seven enums, one struct |
| Declared members | 249 properties, 21 callbacks; NavigationEntry still ten fields |
| Old baseline SHA-256 | `f005fc82fe9691d6b44dc97a81e0abb0952399c07f2455b23eea6c9304784579` |
| Candidate/adopted SHA-256 | `f9089440eef0e9c2856cf1833e7591f16284d591e6a72e06389b69a257bce036` |

`api-review.json` records this additive comparison. All 36 declarations copied
into PUBLIC_API were parsed and compared successfully with the reviewed baseline.
Static closure is 66 files, including the same 32 SVG assets.

## Final checks

Logs are under ignored `target/navigation-phase2/final-passed/`. Cargo checks ran
serially after the final Rust semicolon correction. A preceding strict Clippy
attempt rejected that missing semicolon; its failed log remains under `final/`.
The final retry passed. Documentation-only corrections and this report followed.

| Command | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS — nine tests: one root, eight Gallery |
| `python scripts/check_ui_boundaries.py` | PASS — 36 exports, 66 static files, 32 SVGs, resolved Cargo checked |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS — 36 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS — existing Gallery and API probe |

## Rendering and native input evidence

Windows 11, pinned Slint 1.17.1, `winit-software`, 100% scale and the existing
Segoe UI Variable Text host font policy were used. No system font was bundled.

The final Navigation capture command generated **68 raw PNG/JSON pairs**:
17 fixtures × Light/Dark × expanded/compact, 1040×800, page 7, preview 1.
Cases 0–2 use three/one/two primary levels and a separate two-level footer;
cases 3–15 cover missing parent, negative/excess depth, depth jump, duplicate ID,
child-flag mismatch, leaf children, non-preorder, cross-model parent, parent cycle,
empty ID, header children and an oversized model. Case 16 has empty models.

All image dimensions and recorded hashes were checked. `render-checks.json`
compares the primary/footer menu pixels with each presentation's empty fixture:
52 malformed scenes match empty menus, four empty scenes stay empty, and 12 valid
scenes differ from empty. Render inspection confirmed indentation, chevron
direction, selected icons, compact fallback icons, and content framing. Pixel
comparisons do not establish input or accessibility behavior.

| Native acceptance | Observed result |
|---|---|
| Destination and destination-group label | Mouse/Enter/Space invoke once; group labels request expansion |
| Separate primary/footer chevrons | Expansion counter only; selection does not change |
| Declined requests | Counters increment; selection, expansion and pane mode stay host-owned |
| Disabled rows/groups/Back | Pointer/keyboard actions suppressed; disabled UIA state exposed; Tab skips disabled rows |
| Disabled parent / enabled child | Child remains invokable and selectable |
| Focused descendant collapse | Returns focus to Controls in Light expanded and Dark compact; hidden children leave traversal |
| Compact mode | Labels hidden, icons and full ancestor-qualified tooltips remain available outside ScrollView clipping |
| Compact Search | Requests pane expansion; edit/submit counters remain unchanged |
| Search field | Host assignment: Edit 0; typing: Edit 1; Return: Submit 1 |
| Optional regions | Back/Toggle independent; all header controls/title can disappear without residual rows; Footer removes its separator/gap |
| Header selection | Host can assign its ID; no navigation row shows a selection indicator |
| Content surfaces | Fluent/Flat/Transparent inspected in both themes with host content preserved |
| Runtime malformed model | Switching to Missing parent removes menu actions without changing selection or callback counts |
| Defaults | Empty models, expanded 260 px pane, Toggle only, Fluent content observed |
| 760×520 window | Control switches/buttons fit; outer scrolling exposes the complete view; About footer invokes; reverse Tab reaches its parent chevron |

Native UIA observations and selected window screenshots are retained in:

- `native-verified/00-*` through `29-*`: controlled requests, disabled behavior,
  search and initial successful collapse recovery at 1280×800 Light. Metadata is
  in `native-verified-source.json`; this run predates the drawing/layout refinements.
- `native-final-layout/00-*` through `25-*`: final chevrons and geometry, Dark
  compact keyboard/collapse, optional removal, both-theme surfaces, invalid runtime
  model and defaults at 1040×800. It uses the final render-matrix implementation.
- `native-minimum/00-*` through `03-*`: 760×520 Light after the final build, with
  source/binary identity in `minimum-source.json`.

Verification found and fixed actual defects: FocusScope swallowed pointer input;
whole-model replacement recreated focused rows; the SVG rotation failed to show
the collapsed arrow in software rendering; compact Back lacked a fixed y origin;
six switches in one row overflowed at minimum width. The final implementation
uses explicit activation focus, stable scalar repeat counts, explicit chevron Path
geometry, fixed Back origin and a three-column switch grid.

## Source identity and remaining scope

Checked HEAD during implementation: `b21ca689e4fa2e06d7ca2c33f7745ab39d4ee27e`.
Phase 2 is a subsequent local commit; no candidate was pushed or consumer retargeted.
The Phase 1 report's uncommitted status describes its historical observation time.

Final render evidence is under:

```text
target/navigation-phase2/captures-final/
  dirty-529d264ac13ca71af18c0246c7fcf8b0b48b98b3331f83cbe320dfb3eae61408/
    windows-winit-software/
```

That capture's executable SHA-256 is
`341adf52a31a9aa2a7daafd072c4ba460213636eab625ce6a9075437c4aa373c`.
The later semicolon correction and documentation changes have no rendering effect;
the final common gate rebuilt the executable, whose minimum-window verification
SHA-256 is `abe57a935853775e91dac0a52b0057406c9c70b06abc9f0088e09b2154691a4a`.
Dirty-content hashes identify actual working-tree bytes, not Git revisions.
Ignored logs and captures are local evidence and are not package/publication files.

NOT_RUN in this phase: package/archive checks, publication/remote CI, a fresh
external consumer, MSRV, incremental token/SVG mutation, Linux/macOS runtime,
the full DPI/backend matrix, IME and screen-reader action dispatch. Native names,
disabled state and pointer/keyboard guards do not certify complete accessibility.
Arbitrary model reorder/removal focus restoration is not claimed. Arrow-key
navigation and further animation polish remain Phase 7 work; chevron direction
updates immediately without an animation or callback delay.

Phase 2 exit criteria are satisfied within the documented pinned-renderer native
coverage. The main shell and SidebarItem remain intact. Phase 3 can use this public
view for the main-shell cutover before deleting the legacy component; it has not
started in this change.
