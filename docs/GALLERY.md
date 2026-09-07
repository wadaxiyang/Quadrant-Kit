# Gallery

The Gallery is a development, verification, and learning application with no Product dependencies. Build it with `cargo build --locked -p quadrant-kit-gallery`; run with `cargo run --locked -p quadrant-kit-gallery`. It opens DesignGalleryWindow. The compiled KitApiProbe is not shown during normal startup.

## Pages and controls

The shell uses one public NavigationView with eight destinations under
Overview, Foundation and Components; Surfaces & feedback is a nested group.
Stable IDs overview/tokens/typography/icons/controls/surfaces/feedback/navigation
map to the existing snapshot page numbers 0–7. Only implemented pages are linked.
Catalog filter and the separate Pages heading/list are gone. Theme and C/M/W
preview utilities occupy a top toolbar; pane mode remains host-owned.
Back, Search and group expansion use the Gallery-only Rust controller. A different
valid destination adds history; repeated selection, query editing and expansion
do not. Back pops once, reveals ancestors and clears a query only if it hides the
target. Empty history disables Back.

Search matches titles and keywords case-insensitively, ignoring outer whitespace.
Typing filters a flat list without navigating; Enter opens the first match in
catalog order. Clicking any result uses the same route table. No matches displays
a non-interactive message and keeps the current page. Clear search restores the
normal hierarchy and its saved expansion state. In compact mode the search action
expands the pane. History and search are not Kit runtime responsibilities.
GalleryPage now owns each page's ScrollView and padding; the shell only fills
the current destination into its content host.
[Phase 5 evidence](NAVIGATION_REBUILD_PHASE5.md) records the page/specimen rebuild;
[Phase 4 evidence](NAVIGATION_REBUILD_PHASE4.md) records history and search;
[Phase 3 evidence](NAVIGATION_REBUILD_PHASE3.md) records the cutover and removal.

| Page | Content |
|---:|---|
| 0 | Overview: 35-name local catalog and public import example |
| 1 | Tokens: generic theme, spacing, elevation, motion |
| 2 | Typography: generic text roles and content boundaries |
| 3 | Icons: 32 generic SVGs and action-button states |
| 4 | Controls: buttons, segments, input wrappers, settings, badges |
| 5 | Surfaces: decorative/interactive cards and variants |
| 6 | Feedback: Toast and single confirmation Modal |
| 7 | Navigation: controlled NavigationView, standalone Back/pane toggle/content surface, page framing, metrics, empty state, window controls |

The old page 8 and Task patterns filter are removed. Inbox components and
Product window probes belong to Tasks after cutover. Kit has no Inbox models,
product brand, task navigation aliases, or quadrant colors.

Use Light / Dark / System, Compact / Medium / Wide, and the per-page live settings. Existing preview properties remain specimen inputs; they do not assert that real pointer, focus, IME, or assistive-technology paths have been tested. The modal specimen no longer claims a complete Tab trap.

## Observable action specimens

Every destination uses the same transparent GalleryPage. Gallery-local metrics
provide 40 px horizontal padding (24 px below 700 px of available page width),
32 px top padding and section spacing, and 40 px bottom padding. Each page fills
the available viewport and owns one page-level ScrollView. Navigation menus and
standard text editors retain their independent component-local scrolling.

The common header includes title, description, optional stability, and local
Documentation/Source actions. Documentation expands usage guidance in place;
Source reveals the page's integration fragment. Theme and C/M/W remain in the
shell toolbar. These actions do not open external pages or require named child
slots. New pages supply page metadata and a single content children slot.

All 23 specimens share a live preview card and a separated Show source & details
toggle. Details start collapsed with no body height or hidden editor Tab stops.
The toggle supports Tab, Enter and Space and exposes expanded state. Its body
contains an illustrative static snippet and the existing keyboard/accessibility
notes; live counters remain in the preview. The read-only standard TextEdit
supports selection and normal copy commands; its own scroll area is limited to
the code viewport. Integration fragments may require host state/callbacks and
are not generated from the current live preview values.

Expanding/collapsing details leaves the preview mounted and preserves its state.
Changing destination recreates page-local state as before; host-bound example
text and switches retain their existing lifetime. No page cache is introduced.

After the NavigationView specimens, the Navigation page retains the Phase 1 foundation specimen. Show Back and
Show Toggle conditionally mount the controls and their row. Enabled gates both
buttons; Fluent/Flat/Transparent switches the actual public content surface, and
Compact supplies the pane-toggle state. Back and Toggle counters expose callback
delivery; toggling mode is Gallery-owned. The C/M/W utility constrains the sample
surface to 280/480/720 px, bounded by available width. The main shell uses NavigationView; the hierarchy and defaults specimens precede
this foundation sample.

To verify this specimen, click each enabled button, use Tab/Shift+Tab to focus it,
then press Enter and Space; its counter must advance once per action. Disable
both and attempt pointer activation; counters must stay unchanged and Tab must
skip them. Hide each control separately, then both: the row and its layout gap
must disappear. Inspect all three surface modes in Light/Dark and C/M, including
the top-left curve, straight remaining corners, and absence of right/bottom borders.
The existing source-keyed capture tool can capture its default state with
`--mode Smoke --page 7 --preview 1`. Interactive state evidence is recorded in
[the Phase 1 report](NAVIGATION_REBUILD_PHASE1.md).

The Icons page wires every enabled IconButton state to an action count and last
action label. The disabled specimen has the same callback wiring, so an unexpected
activation would also be visible. Controls provides a separate Add/Edit/Delete
counter. Navigation exposes the PageHeader action count and a shared window-action
count with the last Minimize/Maximize/Close label. Window actions are demonstrations:
they keep Gallery open for repeated testing. Counters reset when the page is recreated.

To reproduce the focused native smoke check, start a fresh Gallery:

1. On Icons, click normal Add: expect `Icon actions: 1 · Last: Add`. Use Tab or
   Shift+Tab to focus Delete, then press Enter and Space: expect counts 2 and 3,
   both with `Last: Delete`. Click disabled Dismiss: count stays 3. Tab from Delete
   skips Dismiss and returns to the Gallery controls.
2. On Controls, scroll to the icon states and click Add, Edit, Delete: expect
   counts 1, 2, 3 and the corresponding last-action label.
3. On Navigation, click Add item, focus it with Tab/Shift+Tab, then press Enter
   and Space: expect `Add item actions` to advance 1, 2, 3.
4. Scroll to Window controls. Click Minimize, Maximize, Close: expect counts
   1, 2, 3 with matching labels. Focus Close and press Enter then Space: expect
   counts 4 and 5. The real Gallery window remains open throughout.
5. Switch Light/Dark and Medium/Compact to inspect the feedback text. Use the
   real title-bar close action to exit Gallery after testing.

All listed input checks passed on native Windows on 2026-09-07 after a locked
Gallery build, using its ordinary backend selection (renderer not separately
instrumented). Screenshots and accessibility-tree snapshots are retained as local
evidence under ignored `target/specimen-interaction-20260907/`. Navigation feedback
was also visually checked in Light/Medium, Dark/Medium and Dark/Compact. This is a
focused interaction check, not the full keyboard/IME/screen-reader/backend matrix.
The disabled check applies to IconButton; PageHeader and WindowControlButton do
not declare an enabled input. Public component code and API baselines are unchanged.

## Theme host

Rust applies the known font, selected theme, and initial system preference before showing Gallery, even when its theme equals the default. On Windows it retains Segoe UI Variable Text; elsewhere Slint's system font fallback remains. Font files are not bundled.

GallerySystemTheme is a separate, never-shown Slint host instance whose standard Palette system binding is never overridden. It observes the backend's system color preference and reports changes to DesignGalleryWindow. DesignGalleryWindow explicitly applies the same effective Light/Dark scheme to Kit Theme and its own standard Palette. Unknown falls back to Light consistently. Only the Gallery owns these host instances; the root helper does not depend on Slint or detect system settings. OS notification support varies by backend and requires native verification; manually toggling Light/Dark does not prove OS theme transitions.

## Snapshot interface

Existing names remain supported:

| Variable | Accepted values |
|---|---|
| QUADRANT_GALLERY_WIDTH / HEIGHT | Both supplied together, finite positive numbers; otherwise use window defaults |
| QUADRANT_GALLERY_THEME | light, dark, system (case insensitive), default light |
| QUADRANT_GALLERY_PAGE | Integer 0–7, default 0; 8 is an error |
| QUADRANT_GALLERY_PREVIEW | Integer 0–2, default 1 |
| QUADRANT_GALLERY_SNAPSHOT | Nonempty output path; absent means interactive run |

Invalid configuration fails before constructing the window. Snapshot mode preserves bounded retries for transparent frames, PNG output, and nonzero errors for output failures. The screenshot tool adds a 30-second subprocess timeout. It only stops its own child on timeout.

```console
python scripts/capture_gallery_baseline.py --mode Smoke --page 4 --preview 1
python scripts/capture_gallery_baseline.py --mode Smoke --page 4 --preview 1 --reuse-existing
```

Windows PowerShell entry:

```powershell
pwsh -NoProfile -File scripts/capture_gallery_baseline.ps1 -Mode Smoke -Page 4 -Preview 1
```

The wrapper invokes the Python standard-library script. PowerShell 7 is the local verification target; Windows PowerShell 5.1 has not been tested. Ordinary Gallery keeps its existing default backend selection. The snapshot tool explicitly defaults to winit-software for reproducibility; its `--backend` option permits winit-skia or winit-femtovg if supported by the build. This does not change Cargo features or normal runtime defaults.

Smoke captures Light at 1040×800 / 100%. Matrix captures four sizes, Light/Dark, and 100/125/150/200/225% simulated scales for the selected page/preview. All combines them. Simulated scales are render checks, not actual monitor DPI transitions.

Outputs default to `target/visual-baselines/<full-sha-or-dirty-content-id>/<os-backend-renderer>/`. Each image has its own JSON manifest; page, preview, theme, size and scale appear in filenames. Manifests record full Git SHA when available, dirty/content identity, OS, architecture, renderer, font policy, Slint version, binary hash, logical and actual pixel dimensions, PNG hash, and UTC time. An unborn repository explicitly records no SHA and uses a dirty content identifier. Custom output directories should be outside the repository or under ignored target output.

Reuse requires complete scenario equality and a matching actual PNG hash/dimensions. Other pages and environments are never merged into a current-scene manifest. A fresh capture uses a new temporary PNG so stale files cannot impersonate successful output. Source changes during build/capture cause an explicit failure. Cross-OS/font/renderer images are not asserted pixel-equal. Font policy is recorded, but installed font binary/version parity still needs control for strict cross-machine comparisons.

## Validation and limits

Phase 1 builds the independent Gallery and API probe on Windows. Local command logs and smoke output are recorded under `target/phase1`; exact successful checks are listed in the Tasks migration ledger at the phase handoff. Do not infer passing remote CI, other platforms, MSRV 1.92, package runtime, or accessibility from source presence.

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/verify_distribution.py
python -m unittest discover -s scripts/tests -p "test_*.py"
```

Phase 2 adds the lexical API/layer guard, declaration/default baseline, fixtures, archive verification, incremental token/SVG checks and CI jobs. Current evidence and remaining platform/native limits are in [VALIDATION.md](VALIDATION.md). The Phase 1 table below is historical; it does not substitute for later checks.

### Phase 1 local result — 2026-09-06

The final implementation checked is `acad992704b75959f1ae2f51304864e919b5a87b`; this result is recorded by a subsequent documentation-only commit. Environment: native Windows x86_64 MSVC, Rust/Cargo 1.94.1, Python 3.13.15, Slint 1.17.1.

| Check | Result | Local evidence under target/phase1 |
|---|---|---|
| fmt / clippy with all targets, all features and warnings denied | PASS | `fmt.log`, `clippy-final.log` |
| Rust workspace tests | PASS: 1 helper + 4 configuration tests | `tests-final.log` |
| Python snapshot identity/reuse tests | PASS: 2 tests | `python-tests.log` |
| Isolated local Git clone, own target directory, Gallery build | PASS; no Tasks checkout or Product packages required | `isolated-build-final.log` |
| Static closure / package list | PASS: 32 SVGs with matching hashes; required source and license files included | `distribution-final.log` |
| Source package verification | PASS: 70 files; root helper compiled from the package | `package-final.log` |
| Eight pages, Light/Dark/System, preview 0/1/2, representative 200%/225% simulated scales | PASS for 12 captured rendering scenes | `native/results.json`; final header fix rechecked in `native-final/result.json` and final Controls smoke |
| Invalid input and output path handling | PASS: 8 invalid configurations rejected with exit 1; output-as-directory rejected with exit 2 | `native/results.json`, `native/output-error-recheck.log` |
| Public PowerShell capture entry and matching-scene reuse | PASS | `capture-final.log`, `capture-reuse.log` |

The full eight-page run preceded a final neutral header icon tint correction; final Light Controls and Dark Navigation at 225% were recaptured afterward. The error-case harness initially could not decode a localized Windows error; its output-error case was repeated with explicit UTF-8 and preserved correctly. The Gallery process correctly returned exit 2 on both runs.

This is Gate 1 evidence, not complete migration acceptance. No remote push/CI/consumer build, real OS theme-toggle test, real monitor DPI transition, or complete native keyboard/IME/a11y matrix is claimed. Raw logs/screenshots and the isolated checkout are private local artifacts; the source/manifests/documentation are versioned.

## Learning order

Start with theme.slint and constants.slint, then FluentIcon / SurfaceCard / Badge. Run Gallery while reading one component's small property/callback surface. Next study buttons and focus/disabled handling, then std-widgets text wrappers, page/navigation/settings composition, and Toast/Modal. Make a small change, build and observe the affected specimen, then restore the experiment or commit an intentional change. Tasks does not need to be open for this workflow.

## First exercise: token to Badge to Gallery

Open only the Kit Git root. Start with a clean `ui/primitives/badge.slint` and
preserve any existing work before experimenting. No Tasks process or source is
required for any of the following steps.

1. Read `UiConstants.space_4: 4px` in `ui/foundation/constants.slint` and the
   `Theme`/`Typography` bindings in `ui/foundation/theme.slint`. These are Slint
   globals; each host component instance owns its initialization.
2. Run `cargo run --locked -p quadrant-kit-gallery`. Overview (page 0) shows
   neutral/accent badges; Controls (page 4) also shows five semantic kinds.
   Inspect Badge's two public input properties and Text child, then close Gallery
   before rebuilding the executable on Windows.
3. Capture the starting state:

   ```console
   python scripts/capture_gallery_baseline.py --mode Smoke --page 0 --preview 1 --output-directory target/learning/before
   ```

4. In `ui/primitives/badge.slint`, temporarily replace only
   `border-radius: 12px;` with `border-radius: UiConstants.space_4;`. The component
   already imports UiConstants. Leave the global token and public baseline alone.

   ```console
   python scripts/check_ui_boundaries.py
   python scripts/capture_gallery_baseline.py --mode Smoke --page 0 --preview 1 --output-directory target/learning/modified
   ```

   The capture command builds and runs Gallery. Compare the two PNGs: pill-shaped
   badges become rounded rectangles. The guard passes because no declared public
   signature/default changed; that does not approve the visual change for users.
5. Restore exactly that line to `12px` (or restore your saved original bytes).
   Run the guard and capture with `--output-directory target/learning/restored`.
   Verify `git diff -- ui/primitives/badge.slint` is empty relative to your starting
   state. If keeping a deliberate improvement instead, review it, add appropriate
   coverage/changelog notes and commit Kit separately before proposing adoption.

On 2026-09-06 this exercise was executed from Kit at
`838ecfbead2d0a1966907ddd742cb6f34516d3f6`: native Windows, Rust 1.94.1,
Slint 1.17.1, winit-software, Light, 1040×800, 100%, page 0/preview 1.
All three guards and build/capture commands exited 0. The modified image was
visually checked and had a different hash. Restored source bytes and the restored
PNG matched their starting hashes. Logs, source/scenario manifests and PNGs are
retained in ignored `target/phase7/learning/`; the experiment was not committed.
This demonstrates the learning loop, not an additional IME/DPI/a11y test.

## NavigationView controlled hierarchy

Page 7 begins with the public NavigationView specimen. Choose three/one/two
primary levels (with a separate two-level footer) or an adversarial model in the
scenario selector. All malformed cases
reject both primary and footer menus without removing the host content.
Accept requests determines whether the host updates selection, expansion and
pane mode; counters increment even when a valid request is declined.

Back, Back enabled, Toggle, Search, Footer and Title switches exercise optional
regions. Compact view hides text/indentation and preserves named icon actions.
The separate Inputs/Settings chevrons expand; their labels invoke destinations.
Collapse Controls can hide a focused child without stealing pointer focus first.
Host sets search assigns text without an edit callback. Select header deliberately
sets a non-navigable ID and must leave all selection indicators absent.

[Phase 2 verification](NAVIGATION_REBUILD_PHASE2.md) records results and limitations.

Phase 2 adds two validated startup selectors to the same Gallery executable:
`QUADRANT_GALLERY_NAV_CASE=0..16` and `QUADRANT_GALLERY_NAV_COMPACT=0|1`.
They initialize the existing specimen; defaults remain case 0 / expanded.
The existing capture script records both values in scene identity, rejects reuse
across different fixtures and resets inherited selectors for ordinary captures.

```console
python scripts/capture_gallery_baseline.py --mode Navigation --page 7 --output-directory target/navigation-phase2/captures
```

This generates 68 raw render scenes at 1040×800, 100%: 17 fixtures × two themes ×
two pane modes. It complements native input checks; it is not the later full DPI
matrix. The default-values specimen follows the controlled hierarchy specimen.
