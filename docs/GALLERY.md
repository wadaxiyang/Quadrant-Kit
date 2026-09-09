# Gallery

## FluentButton P2 verification

The existing live specimen compares a visible std Button and FluentButton with
matching label/enabled/primary state and separate action counters. Danger uses a
neutral native surface with a passive red outline. FluentButton hover/pressed/focus
are driven by input; its old preview properties are removed. IconButton static
previews remain explicitly pending P3.

`python scripts/run_button_checks.py` builds an isolated current consumer under
`target/button-checks/<run>/`, checks public WindowEvent input and writes raw logs,
source identity and state PNGs. `--help` describes supported options; `--build-only`
reports BUILT_NOT_RUN. Running the saved binary without arguments opens its manual
Windows input/accessibility host. It never adds Gallery startup work or Kit runtime
code. PageHeader and finite ModalManager actions are included; full modal focus
containment and screen-reader behavior remain outside this evidence.


P1 keeps Gallery as a consumer of the current facade. New components become
available through static exports, then get a real catalog/specimen entry; the
catalog is not a runtime registration mechanism. Existing API/native probes are
compile-only and do not add normal-startup instances. The current API probe now
checks slot content/explicit child enabled bindings, controlled selection and
programmatic state, focus entry and narrow sizing. When an API changes, update
these current use sites and assertions without preserving legacy pages/aliases.
All input, rendering, IME, accessibility and performance results retain separate
coverage in COMPONENT_STATUS.md and the phase reports. P1 changes no Gallery page
behavior, production preview input or platform chrome.

The Gallery is a development, verification, and learning application with no Product dependencies. Build it with `cargo build --locked -p quadrant-kit-gallery`; run with `cargo run --locked -p quadrant-kit-gallery`. It opens DesignGalleryWindow. The compiled KitApiProbe is not shown during normal startup.

## Native window chrome and shared toolbar

The main Gallery shares application controls through `shared/gallery_toolbar.slint`:
Back, pane toggle, title, theme and preview. It accepts native button exclusion
insets and draws no minimize/maximize/close buttons. Kit's NavigationView API is
unchanged: Gallery disables its internal operation row and composes the public
standalone navigation controls above it. Search starts directly below this row.
The WindowControlButton page remains a component demonstration with counters.

The host's `src/window_chrome.rs` selects the pinned winit backend on Windows and
macOS; platform adapters live in `src/window_chrome/`. The configured renderer
is retained. Native decorations stay enabled on every platform.

- Windows extends the client into the native caption and keeps DWM-drawn window
  controls. The 48 px application toolbar reserves their measured physical bounds.
  Only their area is left transparent for DWM composition; the application body
  remains opaque. DWM colors follow the Gallery theme. Windows owns button actions,
  title dragging, double-click and the system menu; the host supplies resize hit
  regions and maximized work-area constraints for the extended client.
- macOS uses a transparent native titlebar with a full-size content view and the
  real AppKit traffic lights. Their live view bounds determine the leading inset;
  AppKit appearance follows the Gallery theme. The adapter invokes native dragging
  and zoom/minimize actions. Apple Silicon cross-checks are available; native Mac
  rendering, Retina, Spaces/full-screen and accessibility remain **NOT_RUN** locally.
- Other platforms retain the normal system titlebar and one application toolbar.

Window behavior belongs to the Gallery host, not Kit. Only Gallery enables the
pinned `unstable-winit-030` feature and platform bindings; no dependency versions
or public Kit APIs change. The opt-in navigation validation host remains independent.
Slint snapshots contain the application surface, so the DWM button area is transparent
in their PNGs. Use a desktop/window capture to inspect the actual native buttons.

Current evidence and limits: [native window chrome](GALLERY_NATIVE_CHROME_VALIDATION.md).
The [earlier hand-drawn caption report](GALLERY_TITLE_BAR_VALIDATION.md) is historical.

## Pages and controls

The shell uses one public NavigationView driven by `gallery/catalog.tsv`.
This Gallery-local metadata supplies the hierarchical navigation, search,
canonical string destinations, explicit snapshot aliases, titles and public
visual component links in All components. The typed Slint dispatcher is checked
against every metadata row by the route coverage test.

There are 25 destinations under Home, All components, Design guidance, Controls,
Surfaces & data display, Feedback & overlays, Navigation, Page & layout and Window.
All components lists exactly the 21 public visual components; globals, enums and
NavigationEntry stay in conceptual guidance or their owning component's docs.
The complete Kit facade remains 35 public names.

Home is the starting point. Controls overview retains the cross-component input
and selection comparison and the page-4 CI smoke contract. SurfaceCard retains
its interactive/state overview. Feedback overview compares transient outcomes
with explicit decisions. Dedicated pages reuse extracted Button, IconButton,
Toast, Modal and window specimens. NavigationView has its own composable
configuration page. The three standalone navigation primitives stay together
on Navigation foundations to demonstrate their composition. Theme, typography
and the FluentIcon/resource catalog remain grouped design guidance.

Selecting the current destination does not add history. Back restores the last
visited destination and reveals its ancestors. Search matches titles, public
visual names and keywords case-insensitively; typing filters without navigating.
Enter opens the first match in catalog order. Clicking an All components card
uses the same controller as a navigation item. No results preserves the page;
clearing search restores the hierarchy and its expansion state. Compact search
expands the pane. These behaviors belong to Gallery, not Kit.

All destinations fill the shared GalleryPage viewport. The page owns scrolling
and padding; the shell owns Theme and C/M/W preview actions. Only public
`@quadrant-kit` imports supply Kit components.

[Phase 6 evidence](NAVIGATION_REBUILD_PHASE6.md) records the catalog migration;
[Phase 5 evidence](NAVIGATION_REBUILD_PHASE5.md) records the page/specimen contract;
[Phase 4 evidence](NAVIGATION_REBUILD_PHASE4.md) records history and search.

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

The specimens share a live preview card and a separated Show source & details
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

The Navigation foundations destination retains the Phase 1 foundation specimen. Show Back and
Show Toggle conditionally mount the controls and their row. Enabled gates both
buttons; Fluent/Flat/Transparent switches the actual public content surface, and
Compact supplies the pane-toggle state. Back and Toggle counters expose callback
delivery; toggling mode is Gallery-owned. The C/M/W utility constrains the sample
surface to 280/480/720 px, bounded by available width. The main shell uses NavigationView; hierarchy and defaults specimens live on its dedicated page.

To verify this specimen, click each enabled button, use Tab/Shift+Tab to focus it,
then press Enter and Space; its counter must advance once per action. Disable
both and attempt pointer activation; counters must stay unchanged and Tab must
skip them. Hide each control separately, then both: the row and its layout gap
must disappear. Inspect all three surface modes in Light/Dark and C/M, including
the top-left curve, straight remaining corners, and absence of right/bottom borders.
The existing source-keyed capture tool can capture its default state with
`--mode Smoke --destination navigation-foundations --preview 1`. Interactive state evidence is recorded in
[the Phase 1 report](NAVIGATION_REBUILD_PHASE1.md).

The IconButton page wires every enabled IconButton state to an action count and last
action label. The disabled specimen has the same callback wiring, so an unexpected
activation would also be visible. Controls provides a separate Add/Edit/Delete
counter. PageHeader exposes its action count; WindowControlButton exposes a window-action
count with the last Minimize/Maximize/Close label. Window actions are demonstrations:
they keep Gallery open for repeated testing. Counters reset when the page is recreated.

To reproduce the focused native smoke check, start a fresh Gallery:

1. On IconButton, click normal Add: expect `Icon actions: 1 · Last: Add`. Use Tab or
   Shift+Tab to focus Delete, then press Enter and Space: expect counts 2 and 3,
   both with `Last: Delete`. Click disabled Dismiss: count stays 3. Tab from Delete
   skips Dismiss and reaches the source/details toggle.
2. On Controls, scroll to the icon states and click Add, Edit, Delete: expect
   counts 1, 2, 3 and the corresponding last-action label.
3. On PageHeader, click Add, focus it with Tab/Shift+Tab, then press Enter
   and Space: expect `Add actions` to advance 1, 2, 3.
4. Open WindowControlButton. Click Minimize, Maximize, Close: expect counts
   1, 2, 3 with matching labels. Focus Close and press Enter then Space: expect
   counts 4 and 5. The real Gallery window remains open throughout.
5. Switch Light/Dark and Medium/Compact to inspect the feedback text. Use the
   real title-bar close action to exit Gallery after testing.

The original aggregate-page versions of these input checks passed on native
Windows on 2026-09-07 before the Phase 6 split, after a locked Gallery build,
using its ordinary backend selection (renderer not separately
instrumented). Screenshots and accessibility-tree snapshots are retained as local
evidence under ignored `target/specimen-interaction-20260907/`. Navigation feedback
was also visually checked in Light/Medium, Dark/Medium and Dark/Compact. This is a
focused interaction check, not the full keyboard/IME/screen-reader/backend matrix.
The disabled check applies to IconButton; PageHeader and WindowControlButton do
not declare an enabled input. That historical specimen-only change did not alter
public component code or API baselines; later focus fixes are recorded below.

## Navigation keyboard and polish verification

Navigation labels and their independent chevrons support Tab/Shift+Tab and
Enter/Space. Right requests expansion; Left requests collapse, or moves focus
to a visible enabled ancestor when the row is already collapsed or is a leaf.
These keys never select a destination. Up/Down/Home/End item traversal is not
implemented; use Tab/Shift+Tab. Expansion and selection remain host-controlled.
Disabling a focused row recovers to an enabled ancestor or the pane focus scope;
disabling an IconButton clears focus and its focus ring. Compact search expands
the pane and focuses the new editor. Back, toggle, compact search and row tooltips
use popup presentation to escape pane clipping.

The opt-in native validation window uses the same public NavigationView and the
Gallery's existing hierarchy fixtures. It is not a catalog destination. Launch
with `QUADRANT_GALLERY_NAV_VALIDATION=0` for manual input checks; do not supply
PAGE/DESTINATION, and use Light/Dark (the default is Light). Other validation
variants are 1=no Back, 2=no Search, 3=no footer, 4=no optional regions,
5=flat surface, 6=transparent surface, 7=180 px expanded pane. NAV_CASE and
NAV_COMPACT still select the hierarchy and compact state. Each native host
initializes its own Theme, Palette and font.

```console
python scripts/capture_navigation_polish.py
```

This builds once and captures 184 source/binary-identified scenes: 20 real Gallery
Home scenes and 164 full-viewport public NavigationView scenes. The matrix covers
minimum/default sizes, Light/Dark, expanded/compact, simulated 100/125/150/200/225%
scale, optional regions, three content modes, 180/304 px panes, long branch labels,
iconless entries, deep selection and one/two/three-level or empty models. Optional
variants use 100/225%; standard configurations use all five scales. `--smoke`
captures only one validation scene and does not satisfy the full matrix.
The dedicated manifest schema/suite name keeps these captures separate from
normal Gallery snapshot reuse. There is no automatic pixel-diff acceptance rule.

[Phase 7 evidence](NAVIGATION_REBUILD_PHASE7.md) separates native input observations,
simulated rendering and unavailable platform/monitor/accessibility coverage.

## Theme initialization

Rust applies the known font, selected theme, and initial system preference before showing Gallery, even when its theme equals the default. On Windows it retains Segoe UI Variable Text; elsewhere Slint's system font fallback remains. Font files are not bundled.

GallerySystemTheme is a separate, never-shown Slint host instance whose standard Palette system binding is never overridden. It observes the backend's system color preference and reports changes to DesignGalleryWindow. DesignGalleryWindow explicitly applies the same effective Light/Dark scheme to Kit Theme and its own standard Palette. Unknown falls back to Light consistently. Only the Gallery owns these host instances; the root helper does not depend on Slint or detect system settings. OS notification support varies by backend and requires native verification; manually toggling Light/Dark does not prove OS theme transitions.

## Snapshot interface

Existing names remain supported:

| Variable | Accepted values |
|---|---|
| QUADRANT_GALLERY_WIDTH / HEIGHT | Both supplied together, finite positive numbers; otherwise use window defaults |
| QUADRANT_GALLERY_THEME | light, dark, system (case insensitive), default light |
| QUADRANT_GALLERY_PAGE | Optional legacy alias 0–7; 8 is an error |
| QUADRANT_GALLERY_DESTINATION | Stable catalog destination; default Home when both route variables are absent |
| QUADRANT_GALLERY_PREVIEW | Integer 0–2, default 1 |
| QUADRANT_GALLERY_SNAPSHOT | Nonempty output path; absent means interactive run |

Numeric aliases remain explicit and are never renumbered:

| Alias | Destination |
| --- | --- |
| 0 | home |
| 1 | tokens |
| 2 | typography |
| 3 | icons |
| 4 | controls |
| 5 | surfaces |
| 6 | feedback |
| 7 | navigation-view |

The runtime rejects PAGE and DESTINATION supplied together, even when they name
the same page. Python `--page`/`--destination` and PowerShell `-Page`/`-Destination`
follow the same rule. CLI captures discard inherited Gallery environment options
and explicitly set their selected route. No option means Home. Group IDs and
unknown/empty destinations are rejected before opening a window.

```console
python scripts/capture_gallery_baseline.py --mode Smoke --destination home
python scripts/capture_gallery_baseline.py --mode Smoke --destination all-components
python scripts/capture_gallery_baseline.py --mode Smoke --destination navigation-view
python scripts/capture_gallery_baseline.py --mode Catalog
```

Catalog mode captures every metadata destination at normal/minimum sizes in both
Light and Dark (100% scale); omit route options for this mode. Navigation mode
accepts `--destination navigation-view` or legacy `--page 7` and keeps the 17 model
fixtures in both pane modes/themes. This does not add runtime configuration APIs
to Kit. PowerShell accepts the same modes and forwards its explicit route only.

Capture schema 2 includes the resolved stable destination in each scene and its
filename, and rejects reuse from older schemas. Numeric aliases and their string
successors describe the same scene; distinct destinations cannot share reuse.
The page-4 smoke still passes the numeric alias to the native executable.

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

Outputs default to `target/visual-baselines/<full-sha-or-dirty-content-id>/<os-backend-renderer>/`. Each image has its own JSON manifest; destination, preview, theme, size and scale appear in filenames. Manifests record full Git SHA when available, dirty/content identity, OS, architecture, renderer, font policy, Slint version, binary hash, logical and actual pixel dimensions, PNG hash, and UTC time. An unborn repository explicitly records no SHA and uses a dirty content identifier. Custom output directories should be outside the repository or under ignored target output.

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
2. Run `cargo run --locked -p quadrant-kit-gallery`. Open Badge from All components
   to see neutral/accent badges and the success, warning and danger kinds.
   Inspect Badge's two public input properties and Text child, then close Gallery
   before rebuilding the executable on Windows.
3. Capture the starting state:

   ```console
   python scripts/capture_gallery_baseline.py --mode Smoke --destination badge --preview 1 --output-directory target/learning/before
   ```

4. In `ui/primitives/badge.slint`, temporarily replace only
   `border-radius: 12px;` with `border-radius: UiConstants.space_4;`. The component
   already imports UiConstants. Leave the global token and public baseline alone.

   ```console
   python scripts/check_ui_boundaries.py
   python scripts/capture_gallery_baseline.py --mode Smoke --destination badge --preview 1 --output-directory target/learning/modified
   ```

   The capture command builds and runs Gallery. Compare the two PNGs: pill-shaped
   badges become rounded rectangles. The guard passes because no declared public
   signature/default changed; that does not approve the visual change for users.
5. Restore exactly that line to `12px` (or restore your saved original bytes).
   Run the guard and capture with `--output-directory target/learning/restored`.
   Verify `git diff -- ui/primitives/badge.slint` is empty relative to your starting
   state. If keeping a deliberate improvement instead, review it, add appropriate
   coverage/changelog notes and commit Kit separately before proposing adoption.

On 2026-09-06 the original Overview-based version of this exercise was executed from Kit at
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


## Fluent evolution P3

Migrated icon/segment/card/field specimens use actual pointer and keyboard state;
production preview flags are removed. Gallery disclosure buttons use native Button
with expandable semantics. SettingRow demonstrates one host enabled binding shared
with its native Switch. Existing typed catalog routes remain unchanged.

`python scripts/run_button_checks.py --suite foundation` runs the isolated P3 host;
`--build-only` produces an interactive executable. The default button suite continues
to test FluentButton/Modal/PageHeader. These hosts never enter ordinary Gallery startup.
Source, generated consumer, logs, binary and software images are retained under target/.
See the P3 report for actual counts, catalog captures and native input limits.

P4A: `native-selection` is a conditional reachable page for all four selection
wrappers, sharing checked state and demonstrating disabled, empty and long models.

P4B: conditional `native-numeric` page demonstrates all four numeric/progress exports.

P4C: `native-containers` conditionally demonstrates five exports and 10,000 list rows.

P4D: `native-pickers` conditionally shows table sort requests and native date/time popups.

P5A: Toast interactive specimens now offer Show/auto-dismiss and actually close from
the host callback. Tooltip page separates native service from passive presentation.

P5B: modal-manager and feedback pages remember their actual opener button and
restore it on ModalManager.restore_focus_requested. A page-local epoch transports
that host event through the specimen; it is not a Kit runtime focus service.
