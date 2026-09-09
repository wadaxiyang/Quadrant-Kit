# Changelog

## Fluent evolution P5C — local, unpublished

- NavigationView now rejects invalid row notifications reactively and exposes model_valid.
- Native label/arrow buttons replace custom row activation and painting.
- Up/Down/Home/End move focus without taking host selection; focus scrolls into view.
- Bounded model validation reduces duplicate scans; the 256-entry limit remains.


## Fluent evolution P5B — local, unpublished

- ModalManager Return/Space now activates the focused native button. Initial focus
  is Cancel when present, otherwise primary; Tab/ShiftTab cycles the fixed actions.
- Emit at most one accepted/dismissed request per shown cycle; host owns closing.
- Add restore_focus_requested on logical close and wire actual Gallery openers.
- This remains a finite confirmation, with reader/full-dialog capability unverified.


## Fluent evolution P5A — local, unpublished

- ToastHost now requests dismissal at most once per shown cycle. Host owns closing
  and reopening; changing the message alone does not reset the cycle.
- Unload hidden native Toast input and timer; pause on hover, restart full timeout
  on leave. Remove Toast geometry/opacity motion for the P5 behavior baseline.
- Keep native Tooltip service and document its edge-placement capture limits.


## Fluent evolution P4D — local, unpublished

- Add native table composition and host-controlled date/time pickers with public
  Date/Time types, native text validation and explicit accepted/canceled requests.
- Preserve native row selection and column sort requests; hosts own actual sorting.
- Close pickers on disable/hide and restore opener focus after native confirmation.


## Fluent evolution P4A–P4C — local, unpublished

- Add native selection, numeric/progress and container controls through the facade.
- Preserve RadioGroup/ListView/TabWidget native compiler identity and child grammar.
- Document int SpinBox, host value bounds, static tabs and non-recursive group enable.
- StandardListView exposes scrollbars-enabled for its limited native disable behavior.
- Progress controls unload their native child when hidden and stop indeterminate
  motion with running=false. No root runtime dependency or consumer update.


## Gallery native window chrome — local, unpublished

- Separate the shared GalleryToolbar from Windows/macOS window adapters. Remove
  hand-drawn window controls from the main toolbar; Kit's public API is unchanged.
- Retain the decorated Windows HWND and DWM-rendered caption buttons, measuring
  their exclusion area and matching the Gallery theme. Keep native caption actions
  and handle extended-client resize/work-area geometry in the host.
- Add macOS transparent-titlebar/full-size-content integration with native traffic
  lights, measured leading insets, AppKit appearance and native window actions.
  Native macOS runtime verification remains pending.
- Add macOS bindings using versions already in Cargo.lock; expand Windows API
  features without upgrading dependencies or adding Kit runtime responsibilities.

## Gallery title-bar correction — local, unpublished

- Combine Back, pane toggle, Gallery title, theme/preview and actual window
  controls into one Windows title bar. Remove the decorative menu icon and the
  duplicate NavigationView operation row from the main shell.
- Use native Windows caption hit testing and Slint window minimize,
  maximize/restore, close and resize-border behavior in the Gallery host.
- Enable Gallery's existing pinned Slint winit accessor feature and Windows-only
  windows-sys 0.61.2. The lockfile gains two dependency edges; package
  versions and the public Kit API are unchanged. A scoped Windows subclass lets
  the operating system handle caption dragging, double-click and system menus.

## Navigation rebuild Phase 8 — local, unpublished

- Finalize current Gallery/API/consumer documentation and distinguish the local
  35-name API from the published 28-name extraction candidate.
- Guard documentation declarations and probe coverage against the reviewed API,
  and prevent live legacy navigation/shell identifiers from returning.
- Record clean-source package/archive, Windows MSRV and incremental-build
  verification separately from historical publication and native coverage.

The rebuild intentionally replaces SidebarItem and legacy sidebar tokens with
NavigationView, NavigationEntry, navigation enums and standalone Back, pane-toggle
and content-surface components. Hosts own routing, selection, expansion and pane
mode. The final API baseline preserves the explicit Phase 3 review; Phase 8
does not refresh it, change versions or publish a release.

## Navigation rebuild Phase 7 — local, unpublished

- Add Left/Right expansion and ancestor focus behavior without changing the
  host-owned navigation model or public API.
- Recover focus when a row is disabled; clear disabled IconButton focus and
  focus rings. Focus the editor after compact search expands the pane.
- Use popup tooltips for Back and compact search. Clarify reference specimen
  badges and editor keyboard guidance.
- Add an opt-in full-viewport navigation validation host and a reproducible
  simulated-DPI render matrix alongside real Gallery shell scenes.

## Navigation rebuild Phase 6 — local, unpublished

- Introduce one Gallery catalog for 25 routes, navigation/search and 21 public
  visual component links in Home/All components.
- Split focused component pages while reusing extracted comparison specimens;
  dedicate NavigationView and retain purposeful Controls/Feedback/surface overviews.
- Add stable destination snapshot options, preserve numeric aliases 0–7, reject
  conflicting routes and invalidate old capture reuse with scene schema 2.
- Add route/API coverage and capture argument fixtures; retain Kit API and assets.

## Navigation rebuild Phase 5 — local, unpublished

- Transfer all eight destinations atomically to GalleryPage-owned scrolling,
  responsive page padding and shared header Documentation/Source actions.
- Unify 23 specimens around live preview cards and keyboard-operable source/details
  panels with read-only selectable code, retaining input/counter observability.
- Keep page backgrounds transparent and use Gallery-local documentation metrics;
  preserve Kit's public API, NavigationView and content surface.

## Navigation rebuild Phase 4 — local, unpublished

- Organize the eight implemented Gallery pages into a validated hierarchy while
  retaining stable route IDs and snapshot page numbers.
- Add Gallery-owned Back history, case-insensitive title/keyword search, flat
  results, empty-result feedback and preserved normal-tree expansion.
- Enable the main shell's Back/Search controls and route all selections through
  one Gallery controller; keep the Kit public API and source-only architecture.
- Fix narrow-window overflow in the Overview catalog and Navigation specimen
  while retaining the main pane's specified 304 px width.

## Navigation rebuild Phase 3 — local, unpublished

- Switch the Gallery shell to NavigationView with stable eight-page IDs and a
  separate Theme/preview toolbar; preserve host-owned scrolling and page state.
- Remove SidebarItem and its three legacy sidebar tokens after native cutover
  verification. Preserve all other contracts except two equivalent token rebindings.
- Replace the legacy specimen/probe/sample, update the reviewed 35-name API, and
  keep the pane-toggle tooltip readable at the window edge.

## Navigation rebuild Phase 2 — local, unpublished

- Add controlled NavigationView with three-level primary/footer menus, split
  destination-group actions, optional regions and host-owned content.
- Reject malformed/oversized models safely; support up to 256 entries per model
  with source-only validation on the pinned Slint compiler.
- Add adversarial Gallery fixtures and observable request/search counters.
- Review the additive 36-name API; preserve the old main shell and SidebarItem.

## Navigation rebuild Phase 1 — local, unpublished

- Add NavigationPaneMode, NavigationContentSurfaceMode, NavigationEntryKind and
  NavigationEntry, plus standalone NavigationBackButton, NavigationPaneToggleButton
  and NavigationContentSurface. Add seven semantic Theme aliases and four navigation
  constants. The reviewed facade now has 35 names; old declarations/defaults remain.
- Reuse IconButton input/focus/tooltip behavior; draw the Back arrow in Slint source.
  Content surfaces provide Fluent top-left framing, flat and transparent modes,
  without owning page scrolling or routes. Existing 32 SVG assets are unchanged.
- Add a live Navigation foundation specimen and compile-only API probe coverage.
  Keep SidebarItem and the old main Gallery shell until the later cutover phases.
- This does not change the retained extraction source or authorize publication.

## 0.1.0 — unreleased extraction candidate

- Extract generic Slint source and Gallery into an independent workspace. Root Rust helper exposes only the facade library name and build-time file location.
- Public facade deliberately changes from 32 embedded names to 28: Branding, TaskRowShell, InboxItem, and InboxPane remain with Tasks.
- Remove Q1–Q4 colors, Typography.timer, UiConstants.focus_wide_breakpoint, and 11 product icon aliases from Kit. Preserve other component defaults, behavior, and visual recipes.
- Retain 32 required generic SVGs unchanged with their MIT license and source mapping. Embed static Slint assets as files at build time.
- Gallery now has eight neutral pages (0–7). Remove Inbox routing, task specimens, product branding and workflow text; keep controls, previews, and feedback specimens.
- Reject invalid snapshot configuration before native window creation; explicitly initialize theme/font and follow backend system theme through a separate never-shown host instance.
- Add a compiled probe covering all 28 public names. Replace snapshot filename-only reuse with source, environment, and scenario manifests.
- Add the fail-closed lexical boundary/API guard, explicit signature/default baseline, Cargo policy fixtures, shared static-resource scanner and package archive verification.
- Add incremental token/SVG rebuild verification and push/PR/manual CI for Linux, Windows, macOS and the Rust 1.92 build baseline. Actual execution evidence is tracked separately from workflow presence.

This is not a compatibility-preserving release of the old embedded facade.
The extraction candidate has now been published, retained and remotely consumed;
no stable release tag is claimed. Source/CI references and outstanding limitations
are tracked in docs/VALIDATION.md and docs/CONSUMER_GUIDE.md.

## Documentation handoff — 2026-09-06

- Align candidate publication, consumer setup and Gallery ownership with the
  implemented repositories. Add an executable dependency example using the
  qualified retained source, without changing Tasks' adoption.
- Add the completed token → Badge → Gallery exercise and relocation cache
  recovery. The trial visual change was fully restored; no public API/default,
  asset, runtime behavior, Slint version or MSRV changed in this handoff.
- Keep native accessibility/focus/IME limitations explicit. Documentation
  handoff is not a new component release or a waiver of those follow-ups.

## Gallery interaction follow-up — 2026-09-07

- Wire IconButton, PageHeader and WindowControlButton specimens to visible
  action counts and labels. Window-action specimens keep Gallery open for
  repeated input checks; the window-button specimen row has an explicit height.
- Verify native mouse/Enter/Space feedback and disabled IconButton suppression;
  record reproducible steps in docs/GALLERY.md. This changes only Gallery and
  documentation, with no public component/API/default/asset or consumer pin change.

For future changes, document compatible fixes and reviewed visual impact in a
patch; record compatible additions with facade/docs/probe coverage. Breaking
names/types/callbacks or important behavior require the next minor (for example
0.1.x → 0.2.0) and consumer migration instructions. Review toolchain/Slint/backend
changes separately. Git SHA pinning does not make all pre-1.0 versions compatible.
