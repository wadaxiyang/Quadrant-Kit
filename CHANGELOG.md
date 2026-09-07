# Changelog

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
