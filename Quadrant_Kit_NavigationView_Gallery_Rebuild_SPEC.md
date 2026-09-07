# Quadrant-Kit NavigationView & Gallery Rebuild SPEC

> **Status:** Implementation specification / architecture baseline
> **Target repository:** `wadaxiyang/Quadrant-Kit`
> **Scope:** Quadrant-Kit only
> **Out of scope:** Quadrant-Tasks migration, product routing, generic history engine, page lifecycle/cache, publication
> **Primary visual reference:** WPF UI Gallery / WPF UI `NavigationView`
> **Implementation strategy:** staged, continuously buildable, reviewable after every phase

> **Revision:** 2026-09-07 — executable phase sequencing after Phase 0 review.
> Public API changes are reviewed and adopted in the phase that introduces them;
> private navigation rows are verified through NavigationView in Phase 2;
> the main shell switches before SidebarItem removal in Phase 3.

---

## 0. Purpose

This SPEC defines a focused rebuild of **Quadrant-Kit’s navigation system and Gallery**.

The current Kit Gallery is functional, but its shell is assembled manually from a fixed left rectangle, filters, page buttons, and a separate right content rectangle. The current public navigation abstraction is `SidebarItem`, which is too low-level to serve as the long-term navigation architecture.

The new design shall:

1. Replace the public `SidebarItem`-centric navigation model with a complete **`NavigationView`** system.
2. Support **up to three navigation levels**: level 1, level 2, level 3.
3. Keep `NavigationView` intentionally **non-routing**: it renders navigation UI and emits navigation requests, but it does not own route history or page creation.
4. Make major navigation features **optional and separable** so applications can construct different shells from the same Kit.
5. Rebuild the Kit Gallery around the new `NavigationView`, with a visual and information architecture close to **WPF UI Gallery**.
6. Unify Gallery page structure, page background, content framing, spacing, and specimen presentation.
7. Preserve the existing Kit layering and independent-repository constraints.

This is a **deliberate public API redesign**. Compatibility with the current Quadrant-Tasks usage of `SidebarItem` is not a constraint in this construction phase. Tasks will be migrated later in a separate project.

---

# 1. Normative language

The following words are normative:

- **MUST**: required for acceptance.
- **MUST NOT**: prohibited.
- **SHOULD**: preferred unless there is a documented implementation constraint.
- **MAY**: optional.

When implementation details conflict with this SPEC, the architectural boundaries and acceptance criteria in this SPEC take priority.

---

# 2. Repository constraints and construction rules

## 2.1 Work only in Quadrant-Kit

This construction MUST modify only the **Quadrant-Kit** repository.

Do not modify `Quadrant-Tasks` during this work. Do not add sibling path overrides or temporary compatibility code for Tasks.

The existing repository contract remains authoritative:

- `@quadrant-kit` maps to `ui/kit.slint`.
- Kit implementation layering remains:
  - `foundation`
  - `primitives`
  - `patterns` / `overlays`
- Implementations MUST NOT import back through the public `@quadrant-kit` facade.
- Gallery MAY import `@quadrant-kit`.
- Product branding/domain/storage/IPC/Agent concerns remain outside Kit.

## 2.2 Do not combine unrelated work

This SPEC MUST NOT be used as justification to:

- upgrade Slint;
- upgrade the Rust toolchain;
- change renderer/backend defaults;
- redesign unrelated controls;
- introduce product-specific Quadrant-Tasks APIs;
- add a generic application router framework;
- replace standard text/scroll widgets without a concrete need;
- publish a Kit candidate.

## 2.3 Git and validation discipline

Before editing, inspect repository status.

Do not overwrite unknown changes. Keep one writer per checkout.

At the end of each phase, run the phase-specific validation commands before starting the next phase.

The existing API guard is intentionally fail-closed. **Do not auto-refresh `scripts/kit_api_v1.json` to make failures disappear.** An intentional API change requires a candidate baseline, manual diff review, and deliberate adoption in the phase specified below.

### API review at the point of change

The baseline file remains `scripts/kit_api_v1.json`; its `v1` name identifies the
existing declaration guard, not a promise to retain the old navigation API.
Do not create competing active baselines or postpone adoption until Phase 3.

For each phase that changes public declarations:

1. Finish the intended declaration changes and compile their public Gallery probe.
2. Generate a candidate with the existing `--write-baseline` command into a
   phase-specific ignored directory, for example `target/navigation-phase1/kit_api_candidate.json`.
3. Run `git diff --no-index scripts/kit_api_v1.json <candidate-path>` and inspect
   every signature and default-expression difference. Exit code 1 from this diff
   means differences were found, not a failed compilation or a passed guard.
4. Record the accepted additions/removals/defaults and rationale in the phase
   report. Update `docs/PUBLIC_API.md`, current API counts, Gallery catalog text,
   probe coverage and affected exact-count tests in that same phase. Preserve
   historical evidence and retain exact API checks; do not weaken tests to accept
   arbitrary export counts.
5. Deliberately adopt only the reviewed candidate, then run the complete common
   phase gate in section 21.0 against the normal active baseline.

This SPEC authorizes the scoped API changes and their reviewed local baseline
adoption. It does not require another permission round for each adoption.
Unexpected API differences must be resolved or removed before adoption.
An expected guard failure while editing is not an acceptable phase exit.
CI continues to run only the normal read/check command.

Phase 1 reviews additive types/tokens/standalone components; Phase 2 reviews
NavigationView; Phase 3 reviews legacy removal. Later phases use the same process
only if a concrete fix requires a public contract change.

---

# 3. Current-state baseline

The implementation must begin by reviewing these existing files:

```text
ui/kit.slint
ui/foundation/theme.slint
ui/foundation/constants.slint
ui/patterns/navigation/sidebar_item.slint

gallery/ui/gallery.slint
gallery/ui/shared/gallery_catalog.slint
gallery/ui/shared/gallery_page.slint
gallery/ui/shared/specimen.slint
gallery/ui/shared/preview_container.slint

docs/PUBLIC_API.md
docs/GALLERY.md
docs/VALIDATION.md
scripts/kit_api_v1.json
gallery/ui/api_probe.slint
AGENTS.md
```

Relevant current facts:

- `SidebarItem` is currently a public export from `ui/kit.slint`.
- The current Gallery shell manually lays out the sidebar and content area.
- The current Gallery combines **Catalog filter** and **Pages** as separate navigation concepts.
- Existing `GalleryPage` is already transparent and uses `@children`, which is a useful basis for the new page template.
- Existing `Specimen` already provides a reusable demonstration container, but its layout should be refined for the new Gallery.
- Existing theme tokens include `Theme.background`, `Theme.content_bg`, `Theme.sidebar_bg`, `Theme.hover_bg`, `Theme.selected_bg`, `Theme.border_color`, `Theme.divider`, and `Theme.accent`.
- Existing constants include navigation item dimensions, sidebar widths, content radius, and page paddings.

---

# 4. Reference behavior from WPF UI Gallery

The new design should be **inspired by**, not mechanically clone, WPF UI Gallery.

Important reference characteristics:

1. A complete `NavigationView` owns both the navigation pane region and the content-host region.
2. Back button, pane toggle, search, primary menu, footer menu, and content host are parts of the navigation shell.
3. Navigation items can be hierarchical.
4. The application/window still owns window chrome, routing, pages, dialogs, and application state.
5. The Gallery page body uses a consistent page template and specimen pattern.
6. The right content surface is not a four-corner floating card. In the left-navigation layout, the visual content frame uses approximately:

```text
CornerRadius     = 8,0,0,0
BorderThickness  = 1,1,0,0
```

Therefore the right content surface visually has:

- **rounded top-left corner only**;
- **top border only**;
- **left border only**;
- no right border;
- no bottom border;
- no top-right, bottom-left, or bottom-right corner rounding.

This visual rule is mandatory for the default Fluent content surface in the new Gallery.

---

# 5. Core architecture

## 5.1 Responsibility boundary

The target architecture is:

```text
Window / Gallery Window
│
├── Window chrome / title bar                     application-specific
│
└── NavigationView                                Quadrant-Kit
    │
    ├── Navigation pane                           Quadrant-Kit
    │   ├── optional Back button
    │   ├── optional Pane toggle
    │   ├── optional Pane title/header
    │   ├── optional Search field
    │   ├── Primary navigation items
    │   ├── optional separators
    │   └── optional Footer navigation items
    │
    └── Content surface / content host             Quadrant-Kit
        │
        └── @children                               application-owned page
```

`NavigationView` MUST own the **layout relationship** between navigation pane and content surface.

The application MUST own the actual current page.

## 5.2 NavigationView MUST NOT become a router

`NavigationView` MUST NOT own:

```text
history
route stack
current_route as routing authority
page creation
page cache
page lifecycle
page service / dependency injection
application go_back() implementation
forward navigation
```

`NavigationView` MAY receive the selected destination ID as state and MUST emit callbacks such as `item_invoked()` and `back_requested()`.

The host decides what those callbacks mean.

## 5.3 Controlled-component principle

Where practical, `NavigationView` SHOULD behave as a **controlled UI component**:

- the host provides current selection;
- the host provides whether Back is enabled;
- the host receives invocation callbacks;
- the host may provide expansion state;
- the host decides how route history changes.

This prevents navigation UI from silently becoming application architecture.

---

# 6. Modular composition principles

Modularity is a first-class requirement.

## 6.1 Optional regions

Every major region that is not structurally mandatory MUST be independently hideable.

At minimum, `NavigationView` MUST support optional:

| Region / capability | Required control |
|---|---|
| Back button | `show_back_button` |
| Pane toggle | `show_pane_toggle` |
| Pane title/header | optional / visibility property |
| Search field | `show_search` |
| Primary top separator | `show_top_separator` or automatic collapse |
| Footer separator | `show_footer_separator` or automatic collapse |
| Footer items | empty model = no footer region |
| Content frame styling | configurable content-surface mode |
| Navigation icons | entries may omit icons |
| Hierarchical chevrons | only visible for entries with children |

Hidden regions MUST NOT reserve blank layout space.

Example: if `show_back_button == false`, there MUST NOT be an empty 40 px back-button row.

## 6.2 Optional does not mean duplicated implementations

Do not create separate `NavigationViewWithBack`, `NavigationViewWithoutBack`, etc.

One shell MUST compose optional modules.

## 6.3 High-level and low-level composition

To satisfy both ease of use and modular reuse, the new navigation system SHOULD use two layers:

### A. High-level assembly

**Public:** `NavigationView`

This is the normal application-facing component.

### B. Reusable navigation building blocks

Only modules that have clear standalone semantic value SHOULD become public. Pure implementation mechanics remain private.

Approved public components (standalone contracts are defined in section 18.2):

```text
NavigationView
NavigationBackButton
NavigationPaneToggleButton
NavigationContentSurface
```

Recommended private implementation pieces:

```text
NavigationItemRow
NavigationChevron
NavigationSelectionIndicator
NavigationPaneLayout
NavigationSearchRow internals
NavigationLevelIndent helper
```

**Important:** do not recreate the old public `SidebarItem` under a new name merely to preserve the old architecture. If `NavigationItemRow` is only meaningful inside `NavigationView`, keep it private.

Before exporting any low-level module, verify that it is useful independently and has a stable API.

---

# 7. Public NavigationView model

## 7.1 Navigation depth

The first implementation MUST support exactly these semantic levels:

```text
Level 1 → depth = 0
Level 2 → depth = 1
Level 3 → depth = 2
```

Depth greater than 2 is unsupported in v1.

Do not implement arbitrary recursive depth in this construction.

Deeper information structures should use other controls such as TreeView/Breadcrumb/page-local navigation later.

## 7.2 Flattened hierarchy model

Prefer a flattened tree model over recursively nested Slint component declarations.

Recommended conceptual struct:

```slint
export enum NavigationEntryKind {
    destination,
    group,
    destination_group,
    separator,
    header,
}

export struct NavigationEntry {
    id: string,
    parent_id: string,
    depth: int,
    text: string,
    icon: image,
    selected_icon: image,
    kind: NavigationEntryKind,
    enabled: bool,
    has_children: bool,
    expanded: bool,
}
```

The exact syntax may be adjusted to Slint 1.17.1 limitations, but the semantic model MUST remain equivalent.

### Required semantics

- `id` MUST be stable within one navigation model.
- `parent_id` is empty for root entries.
- `depth` MUST be 0, 1, or 2.
- `group` is non-navigable and expands/collapses.
- `destination` navigates and has no children.
- `destination_group` may navigate when its label region is invoked and separately expand/collapse through its chevron.
- `separator` and `header` are non-selectable.
- `expanded` is UI state, not route history.

### Ordering, visibility and controlled state

Both models contain complete trees in depth-first preorder: a parent precedes
its contiguous descendants. A parent and its children must be in the same model.
Nonempty IDs are unique across `items` and `footer_items` together, including
non-interactive entries. Root entries have empty `parent_id` and depth 0.
`has_children` must agree with actual children; only `group` and
`destination_group` can have children, and depth-2 entries cannot have children.
Hosts must set `enabled` explicitly; a Slint struct is not assumed to default it
to true. Empty `selected_icon` falls back to `icon`.

NavigationView shows a descendant only while every ancestor is expanded. It
computes presentation from the supplied model without deleting entries or
changing `expanded`. `expansion_requested(id, desired_state)` is a request;
selection, expansion and pane mode change only when the host updates their inputs.
Do not treat emitting a callback as host acceptance. An unknown selected ID, or
an ID belonging to a header/separator/group, produces no selection indicator.
Collapsing a selected child's ancestor preserves the supplied selection without
moving the indicator to the parent.

Disabled interactive entries emit no invocation or expansion callbacks through
pointer, keyboard or accessibility actions. Disabling a parent does not rewrite
its descendants' own enabled state. Hosts that want an entire disabled subtree
must mark its entries disabled. Hidden/invalid entries are not focusable.

Phase 2 compiler constraint: Slint 1.17.1 rejects recursive function calls.
The source-only implementation supports up to 256 entries per model using eight
explicit subdivision stages. Larger models reject both menus safely; this is a
model-size bound, not an increase in the three-level depth limit. No Rust runtime
is required by Kit consumers.

## 7.3 Invalid hierarchy behavior

The implementation MUST fail safely or ignore invalid entries rather than render a broken indented tree.

Development-time validation SHOULD detect:

- depth < 0;
- depth > 2;
- missing parent for depth > 0;
- child depth not equal to parent depth + 1;
- duplicate IDs;
- `has_children == false` while children exist;
- separators/headers marked navigable.
- non-preorder/orphaned subtrees, cross-model parents and parent cycles;
- empty IDs, children on leaf kinds, and children below depth 2.

Gallery/model-construction tests MUST cover all cases above. If complete runtime
diagnostics in Slint are impractical, document that limitation and still implement
safe bounded rendering: reject a malformed model or omit invalid entries and
their descendants, never repair depth/parent links by guessing. The chosen
deterministic policy must be documented and exercised through the public Phase 2
specimen. Do not move a Rust UI/model runtime into the root source-locator crate.

---

# 8. Navigation interaction contract

## 8.1 Selection versus expansion

Selection and expansion are independent states.

Example:

```text
▼ Controls               expanded=true, selected=false
    ▌ Button              selected=true
      TextField
```

The accent selection indicator MUST represent the current selected destination only.

Chevron rotation MUST represent expansion only.

## 8.2 Parent invocation

Required behavior:

| Entry type | Main row/label invocation | Chevron invocation |
|---|---|---|
| destination | emit `item_invoked(id)` | none |
| group | toggle/request expansion | toggle/request expansion |
| destination_group | emit `item_invoked(id)` | toggle/request expansion |
| header/separator | none | none |

For `destination_group`, the chevron hit area MUST be distinct enough to avoid accidental navigation while expanding.

## 8.3 Back button

Back button is visually owned by `NavigationView` but history is not.

Required API semantics:

```text
show_back_button: bool
back_enabled: bool
callback back_requested()
```

Rules:

- when hidden, the region collapses completely;
- when visible and disabled, it remains visible but non-invokable;
- when invoked, it only emits `back_requested()`;
- it MUST NOT mutate an internal history stack because no such stack exists.

This allows:

```text
Gallery             show_back_button = true
Quadrant-Tasks      show_back_button = false
Other apps          show_back_button = app-specific
```

## 8.4 Pane toggle

Pane toggle MUST be separately optional from Back.

Recommended API:

```text
show_pane_toggle: bool
pane_mode: NavigationPaneMode
callback pane_toggle_requested()
```

Recommended enum:

```text
expanded
compact
```

Do not add an internal auto-layout state machine in v1. Hosts may bind `pane_mode` to their own window-width expression if they want responsive behavior.

## 8.5 Search

Search UI belongs to the navigation shell; search logic does not.

Recommended contract:

```text
show_search: bool
search_text: string
search_placeholder: string
callback search_changed(string)
callback search_submitted(string)
```

`NavigationView` MUST NOT own fuzzy matching, navigation-result ranking, or application search indexing.

The Gallery will implement its own component catalog filtering/search behavior.

User edits update `search_text` and emit `search_changed`; Return emits
`search_submitted` once. A host assignment to `search_text` must not be echoed as
a user edit or cause a feedback loop. Use the existing text-input wrapper's
edited/accepted contract where possible.

## 8.6 Compact mode and optional-region layout

Compact mode uses `navigation_pane_compact_width`. The full text field and pane
title are not laid out in compact mode. If `show_search` is true, a labeled search
button takes its place and emits `pane_toggle_requested()` to request expanded
mode; Gallery handles that request explicitly. No search callback is fabricated
by changing pane mode. `show_search == false` removes both search representations.

Visible entries follow the same ancestor-expansion rules in both modes. Compact
rows suppress text and indentation but keep the icon slot, a distinct chevron hit
area for parents, and a tooltip/accessibility label that identifies the entry
(including ancestor context when labels repeat). Iconless entries use a neutral
Kit fallback icon in compact mode; they remain valid text-only entries when
expanded. Compact mode requires no flyout, hidden router or auto-layout state machine.

Back and toggle share a wrapping/vertical control region that fits compact width;
only mounted controls consume space. If all controls are hidden the region is
absent. An empty title removes its header; an empty footer removes its separator.
The top separator appears only when enabled and there are visible regions on
both sides. Pane menu scrolling uses std-widgets and must leave footer controls
reachable at the minimum host size. Content-page scrolling remains host-owned.

---

# 9. Navigation visual specification

## 9.1 Pane width

Replace sidebar-specific naming with navigation-pane semantics.

Recommended tokens:

```text
navigation_pane_compact_width
navigation_pane_default_width
navigation_item_height
navigation_item_padding
navigation_item_indent
navigation_icon_slot_width
navigation_icon_size
navigation_content_radius
```

Suggested defaults:

| Token | Suggested value |
|---|---:|
| compact width | 52–54 px |
| default expanded width | 240–280 px |
| Gallery expanded width override | 300–310 px |
| item height | 40 px |
| icon size | 18–20 px |
| level indent | 24–30 px |
| content radius | 8 px |

Gallery MAY intentionally use a wider pane than application defaults.

Lock initial defaults to compact 54 px, expanded 260 px, Gallery override 304 px,
item height 40 px, padding 11 px, indent 28 px, icon slot 24 px, icon 20 px and
content radius 8 px. New navigation names alias existing values where applicable;
do not change old generic defaults. `pane_width` configures expanded width only.
The pane must leave nonnegative content geometry at constrained host sizes;
Gallery's minimum window size must be verified with its 304 px override.

## 9.2 Navigation item appearance

The old `SidebarItem` may be used as a behavioral reference, but its public API MUST NOT survive as the new architecture.

Retain or improve these behaviors:

- hover state;
- selected state;
- left accent selection indicator;
- regular/selected icons;
- focus ring;
- Enter/Space activation;
- tooltip in compact mode;
- smooth width/opacity/chevron animations;
- disabled state.

Second- and third-level entries SHOULD use increasing left indentation while preserving consistent text alignment within each level.

## 9.3 Chevron

Chevron is visible only when `has_children == true`.

Default behavior:

```text
collapsed → downward/rightward neutral chevron
expanded  → rotated expanded-state chevron
```

Use `Motion.standard`-scale animation rather than long decorative animation.

---

# 10. Content surface architecture

## 10.1 Content surface belongs to NavigationView

The right-hand content host MUST be part of `NavigationView`.

The host application supplies the actual page through `@children`.

Conceptual structure:

```slint
NavigationView {
    // navigation shell

    CurrentPage { }
}
```

`NavigationView` MUST NOT create pages based on IDs.

## 10.2 Content surface style modes

To satisfy modularity, expose a small semantic style mode rather than many unrelated raw visual switches.

Recommended enum:

```text
NavigationContentSurfaceMode.fluent
NavigationContentSurfaceMode.flat
NavigationContentSurfaceMode.transparent
```

### `fluent`

Default framed content surface:

- background = navigation/content surface token;
- top-left radius = 8 px;
- top border = 1 px;
- left border = 1 px;
- no visible right/bottom border;
- no other rounded corners.

### `flat`

- content background enabled;
- no frame border;
- no corner treatment.

### `transparent`

- NavigationView provides layout only;
- host background remains visible.

This is preferable to forcing every consumer to use the Gallery-like frame.

## 10.3 Single-corner implementation in Slint

The pinned Slint 1.17.1 compiler defines individual corner properties, including
`border-top-left-radius`, in `i-slint-compiler`'s `builtins.slint` and handles them
in `passes/border_radius.rs`. The shorthand `border-radius` applies to all corners,
but that is not a limitation of the individual-corner API.

Prefer explicit top-left radius 8 px and zero for the other three corners.
The border still needs separate handling: do not assume per-edge border-width
properties exist. A clipped overscan frame is an acceptable first implementation:

1. `NavigationContentSurface` owns the clipped viewport and all frame drawing.
2. Extend its frame rectangle to the right/bottom sufficiently to place those
   border edges outside the viewport; with explicit zero corner radii there,
   overscan only needs to cover the border/antialiasing footprint.
3. Set the frame's top-left radius to 8 px, other corner radii to zero, and border
   to 1 px. Keep the logical content-child viewport at its original size.
4. Verify top/left border continuity and absence of right/bottom borders at the
   required sizes/scales. Flat/transparent modes remove the frame treatment.

If clipping produces artifacts, use an explicit Path for the frame. A uniform
radius rectangle overscanned by the radius is also permitted if visually correct,
but is not required by a presumed lack of per-corner support.

Acceptance is visual, not tied to the implementation trick.

## 10.4 Page backgrounds

Pages inside the content surface MUST be **transparent by default**.

Do not stack a full-page opaque rectangle over the content surface.

Target layering:

```text
Window / navigation shell        Theme.background
Navigation content surface       Theme.navigation_content_bg / Theme.content_bg
GalleryPage                      transparent
Specimen / cards                 card/surface tokens
```

Avoid:

```text
Window background
  → pane background
    → content background
      → page background
        → specimen background
```

Unnecessary opaque layers create muddy visual hierarchy and make corner/border behavior inconsistent.

---

# 11. Theme and token changes

## 11.1 Semantic navigation tokens

The implementation SHOULD introduce semantic navigation aliases where they improve clarity.

Recommended additions:

```text
Theme.navigation_pane_bg
Theme.navigation_content_bg
Theme.navigation_content_border
Theme.navigation_item_hover_bg
Theme.navigation_item_selected_bg
Theme.navigation_item_foreground
Theme.navigation_item_foreground_selected
```

These MAY alias existing generic tokens initially.

Do not duplicate color values if an existing semantic token already expresses the same meaning.

Initial aliases are respectively `sidebar_bg`, `content_bg`, `border_color`,
`hover_bg`, `selected_bg`, `text_secondary`, and `text_primary`. When removing
`sidebar_bg` in Phase 3, preserve its transparent value in `navigation_pane_bg`.
This alias/default change is part of that phase's explicit baseline review.

## 11.2 Constant migration

The current `sidebar_*` constants are a legacy naming artifact.

This rebuild SHOULD move internal navigation code to `navigation_pane_*` naming.

Because this is an intentional pre-1.0 API redesign, removal/rename of sidebar-specific public constants is allowed after API review.

Do not perform unrelated token churn.

---

# 12. Gallery rebuild goals

The Gallery is not merely a test executable. It should become the canonical interactive documentation surface for Quadrant-Kit.

The new Gallery MUST use the new public Kit components rather than duplicate them.

## 12.1 Gallery shell

Target composition:

```text
DesignGalleryWindow
│
├── Gallery title bar / window chrome
│
└── NavigationView
    │
    ├── Back button                         visible
    ├── Pane toggle                        visible
    ├── Search                             visible
    ├── Primary hierarchical catalog
    ├── Footer settings/about              optional
    │
    └── Content host
        └── Current GalleryPage
```

The Gallery MUST NOT manually recreate a second sidebar alongside `NavigationView`.

## 12.2 Gallery navigation information architecture

Remove the current **Catalog filter + Pages** dual-navigation scheme.

Replace it with one hierarchical catalog.

Recommended initial structure:

```text
Home
All components
────────────────

Design guidance
    Theme / Colors
    Typography
    Icons

Controls
    FluentButton
    IconButton
    SegmentButton
    FluentTextField
    FluentTextArea

Surfaces & data display
    SurfaceCard
    Badge
    MetricCard
    SettingRow

Feedback & overlays
    TooltipHost
    ToastHost
    ModalManager

Navigation
    NavigationView

Page & layout
    PageHeader
    SectionHeader
    EmptyState

Window
    WindowControlButton

────────────────
About / Gallery settings
```

Exact category naming may be refined, but:

- one left navigation hierarchy MUST be the source of truth;
- `Catalog filter` MUST be removed;
- component-category hierarchy MUST be readable without a separate filter panel.

## 12.3 One component, one destination where practical

The target Gallery SHOULD move toward **one public visual component per destination page**.

Do not force globals/enums into meaningless visual pages.

For example, `Theme`, `Typography`, `Motion`, and icon catalogs may live under design-guidance pages, while interactive components receive dedicated pages.

The migration can be staged; it does not have to split every current page in the first Gallery phase.

---

# 13. Gallery routing boundary

The Gallery itself MAY own a simple local route/history controller because it is an application.

That controller MUST remain outside `NavigationView`.

Conceptual Gallery state:

```text
selected_page_id
previous_page_ids / simple history
search_text
expanded_navigation_ids
pane_mode
```

Conceptual flow:

```text
NavigationView.item_invoked(id)
    → Gallery controller changes selected_page_id
    → Gallery updates current page
    → Gallery optionally appends to its own history

NavigationView.back_requested()
    → Gallery pops/uses its own previous page state
    → Gallery updates selected_page_id
```

No Gallery history implementation may be moved into Kit’s generic `NavigationView` merely to simplify Gallery code.

---

# 14. Unified Gallery page template

## 14.1 GalleryPage responsibility

`GalleryPage` MUST become the shared page shell for all Gallery destinations.

It should own:

- page scrolling policy;
- standard page padding;
- page title;
- short description;
- optional status/stability metadata;
- optional page actions;
- consistent vertical section spacing;
- transparent page background.

It MUST NOT own:

- window background;
- navigation shell;
- route switching;
- full content-surface background;
- application title bar.

## 14.2 Scroll ownership

`NavigationView` MUST NOT automatically wrap content pages in a `ScrollView`.

Reason: real pages may contain their own scrolling, lists, canvas, or fixed headers.

For the Gallery, **`GalleryPage` owns the page-level ScrollView**.

This avoids forced nested scrolling for future applications.

## 14.3 Page spacing

Recommended Gallery-specific page metrics:

| Metric | Target |
|---|---:|
| left/right padding | 40–42 px |
| top padding | 32–40 px |
| page header to first section | 24–32 px |
| section-to-section spacing | 32–40 px |
| compact horizontal padding | 20–24 px |

These values may be represented by Gallery-local tokens if they are documentation-specific. Do not pollute the public Kit constants with Gallery-only layout values unless they are useful to consumers.

## 14.4 Page header shape

Target hierarchy:

```text
Page title
Short description
[Documentation] [Source] [optional actions]

Section title
Specimen

Section title
Specimen
```

The Gallery header should visually approximate WPF UI Gallery: strong title, restrained description/actions, generous whitespace.

Theme / preview / GitHub-style Gallery utilities belong in a page/tool action region, not in the primary navigation taxonomy.

---

# 15. Specimen redesign

The current `Specimen` is a useful foundation but should be refined.

Target anatomy:

```text
Specimen
├── demo preview area
├── optional description/status
└── collapsible Source code / details area
```

The Gallery SHOULD visually approach the WPF UI example-card pattern:

```text
┌────────────────────────────────────────────┐
│                                            │
│               live preview                 │
│                                            │
├────────────────────────────────────────────┤
│ Source code                             ˅  │
└────────────────────────────────────────────┘
```

Requirements:

- consistent card radius;
- consistent 1 px border;
- preview region background from card/surface token;
- source/details row visually separated;
- source area may initially show generated/static example snippets rather than a full code editor;
- accessibility/keyboard notes should remain available but may move to a lower-emphasis details area rather than dominate every specimen.

Do not remove useful runtime-test observability merely for appearance.

---

# 16. Gallery title bar and window chrome

The WPF UI visual reference uses a custom application title area.

The new Gallery SHOULD create a Gallery-specific title bar using existing Kit primitives where practical.

It may include:

```text
[optional Back at window level only if required by layout]  Kit icon  Quadrant Kit Gallery    ...    Minimize Maximize Close
```

However:

- navigation Back belongs semantically to `NavigationView`;
- minimize/maximize/close remain window chrome;
- do not merge `WindowControlButton` responsibilities into `NavigationView`;
- the Gallery title bar is an application shell, not a new generic router.

If Slint window-decoration limitations make a fully custom draggable title bar risky, preserve native correctness first and implement the visual title bar in a later Gallery polish step. Do not block the core NavigationView on custom chrome.

---

# 17. Proposed file architecture

The exact file names may be adjusted, but the implementation SHOULD converge toward:

```text
ui/
├── foundation/
│   ├── theme.slint
│   ├── constants.slint
│   └── ...
│
├── primitives/
│   └── ...
│
├── patterns/
│   ├── navigation/
│   │   ├── navigation_view.slint
│   │   ├── navigation_types.slint
│   │   ├── navigation_back_button.slint
│   │   ├── navigation_pane_toggle_button.slint
│   │   ├── navigation_content_surface.slint
│   │   └── private/
│   │       ├── navigation_item_row.slint
│   │       ├── navigation_pane.slint
│   │       ├── navigation_chevron.slint
│   │       └── navigation_selection_indicator.slint
│   └── ...
│
└── kit.slint

gallery/ui/
├── gallery.slint
├── shared/
│   ├── gallery_catalog.slint
│   ├── gallery_page.slint
│   ├── gallery_page_header.slint
│   ├── specimen.slint
│   ├── specimen_source_panel.slint
│   └── ...
│
└── pages/
    ├── home_page.slint
    ├── all_components_page.slint
    ├── navigation_view_page.slint
    ├── fluent_button_page.slint
    └── ...
```

Do not create public files solely for organizational aesthetics. Public export count should remain intentional and small.

---

# 18. Public API target

The exact final API must be reviewed against Slint 1.17.1 syntax and the scanner, but the target semantics are:

```slint
export enum NavigationPaneMode {
    expanded,
    compact,
}

export enum NavigationContentSurfaceMode {
    fluent,
    flat,
    transparent,
}

export enum NavigationEntryKind {
    destination,
    group,
    destination_group,
    separator,
    header,
}

export struct NavigationEntry {
    id: string,
    parent_id: string,
    depth: int,
    text: string,
    icon: image,
    selected_icon: image,
    kind: NavigationEntryKind,
    enabled: bool,
    has_children: bool,
    expanded: bool,
}

export component NavigationView inherits Rectangle {
    in property <[NavigationEntry]> items;
    in property <[NavigationEntry]> footer_items;

    in property <string> selected_id;

    in property <NavigationPaneMode> pane_mode: NavigationPaneMode.expanded;
    in property <length> pane_width: UiConstants.navigation_pane_default_width;
    in property <string> pane_title: "";

    in property <bool> show_back_button: false;
    in property <bool> back_enabled: false;

    in property <bool> show_pane_toggle: true;

    in property <bool> show_search: false;
    in-out property <string> search_text;
    in property <string> search_placeholder: "Search";

    in property <bool> show_top_separator: false;
    in property <bool> show_footer_separator: true;

    in property <NavigationContentSurfaceMode> content_surface_mode: NavigationContentSurfaceMode.fluent;

    callback item_invoked(string);
    callback expansion_requested(string, bool);
    callback back_requested();
    callback pane_toggle_requested();
    callback search_changed(string);
    callback search_submitted(string);

    // @children is rendered in the content host.
}
```

This is an **API design target**, not copy-paste code. Adapt property syntax only when required by the pinned Slint compiler.

Defaults are part of the reviewed contract: `items`/`footer_items` are empty,
`selected_id`/`search_text` are empty strings, `pane_mode` is `expanded`, and
`content_surface_mode` is `fluent`. Declare them explicitly where supported.
`pane_title` is the optional text header; v1 does not promise arbitrary named
header slots. The one `@children` insertion point belongs to the content host.

## 18.1 API exclusions

Do not add:

```text
navigate(...)
go_back()
can_go_forward
history
journal
page_type
page_factory
page_cache
route_object
service_provider
```

These are explicitly outside v1.

## 18.2 Standalone public module contracts

These Phase 1 modules are approved public building blocks, not temporary probes:

| Component | Explicit inputs / defaults | Callback / children |
|---|---|---|
| `NavigationBackButton` | `enabled: bool = true`, `accessible_name: string = "Back"` | `clicked()`; no child slot |
| `NavigationPaneToggleButton` | `enabled: bool = true`, `pane_mode: NavigationPaneMode = expanded`, `accessible_name: string = "Toggle navigation pane"` | `clicked()`; no child slot |
| `NavigationContentSurface` | `mode: NavigationContentSurfaceMode = fluent` | content `@children`; no automatic scrolling |

The button labels also provide useful hover tooltips. Both buttons follow the
same enabled/focus/pointer/Enter/Space/accessibility activation contract; they
never mutate navigation state. NavigationView maps `clicked()` to its own
semantic request callbacks. Visibility belongs to conditional composition in the
host/view, avoiding a hidden component that still reserves layout space.

Reuse existing licensed assets where semantically correct. Any necessary new
static icon must be packaged, attributed and deliberately added to the live asset
manifest with its real hash/license in the same phase; do not refresh unrelated
assets or alter historical extraction hashes. Use these actual modules in the
Phase 1 specimen so no test-only facade or private Gallery import is needed.

---

# 19. Accessibility and keyboard requirements

The navigation rebuild MUST not regress existing keyboard/focus behavior.

Minimum acceptance:

- visible destination rows can receive focus;
- Enter and Space invoke the focused destination;
- disabled rows do not invoke callbacks;
- Back button supports keyboard activation;
- Pane toggle supports keyboard activation;
- compact navigation items expose a useful tooltip/label;
- selected state has a non-color-only structural indicator (accent bar + state);
- focus ring remains visible in both light and dark themes.
- hidden/collapsed descendants do not receive focus; when collapsing a subtree
  that contains focus, focus moves to its visible parent control within the view.

P1 target for later phase in this SPEC:

- Up/Down directional movement across visible navigation entries;
- Left collapses an expanded group where appropriate;
- Right expands a collapsed group where appropriate;

Directional movement is Phase 7 polish; hidden-descendant exclusion and safe
focus handling on collapse are mandatory from Phase 2, not deferred polish.

Use only accessibility properties supported by the pinned Slint version. Do not claim screen-reader semantics that have not been verified natively.

---

# 20. Animation requirements

Use existing `Motion` tokens.

Recommended:

| Interaction | Duration |
|---|---:|
| hover/focus color transition | fast/standard |
| pane width | slow |
| label opacity in compact mode | standard |
| chevron rotation | standard |
| child expansion opacity | standard |

Animations MUST NOT delay input or route callback delivery.

Avoid decorative spring/bounce effects.

---

# 21. Phased implementation plan

The work MUST be implemented in phases. **Do not attempt a single giant rewrite.**

Each implementation phase ends with a compilable/reviewable state and a passing
common gate. Incomplete implementation or unavailable required evidence must be
reported honestly; it does not satisfy that phase's exit criteria.

## 21.0 Common phase gate and evidence

Run all six commands below at the end of **every Phase 0–8**, after any scoped
API candidate has been reviewed and adopted:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/check_ui_boundaries.py
python -m unittest discover -s scripts/tests -p "test_*.py"
cargo build --locked -p quadrant-kit-gallery
```

Keep Cargo commands serial when sharing a checkout/build directory. Preserve
command lines, exit codes and logs under `target/navigation-phase<N>/`; the
already completed Phase 0 logs remain under `target/navigation-rebuild-phase0/`.
The phase-specific visual/behavioral checks below are additional requirements,
not substitutes for this gate. Documentation-only amendments between phases need
appropriate document/consistency checks, not a repeated native matrix.

Visual evidence MUST identify source state, page/specimen, theme, pane mode,
window/preview size, backend and scale. Use the existing Gallery capture path;
extend its scenario selection only when needed. A screenshot is rendering
evidence, not proof of keyboard behavior. Record native input observations
separately. Missing required evidence means the corresponding gate is NOT_RUN
and the phase is incomplete; optional platform coverage can remain NOT_RUN with
a stated limitation. Do not refresh old screenshots to conceal a regression.

---

## Phase 0 — Baseline, design lock, and guard preparation

### Goal

Establish a clean baseline before altering public API.

### Tasks

1. Inspect `git status`.
2. Read `AGENTS.md`, `docs/VALIDATION.md`, `docs/PUBLIC_API.md`, and current Gallery docs.
3. Build current Gallery and run current boundary checks.
4. Record the current public navigation API and current `SidebarItem` dependencies.
5. Search Kit source for all `SidebarItem` usages.
6. Search Gallery source for all manual sidebar/catalog-filter code.
7. Do not change behavior yet.

### Required validation

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/check_ui_boundaries.py
python -m unittest discover -s scripts/tests -p "test_*.py"
cargo build --locked -p quadrant-kit-gallery
```

### Exit criteria

- baseline is clean or pre-existing failures are documented;
- all current `SidebarItem` references are known;
- no API baseline has been changed.

### Codex stop point

Stop and summarize baseline findings before Phase 1 if unexpected repository changes or validation failures exist.

---

## Phase 1 — Navigation types, tokens, and standalone public building blocks

### Goal

Establish the public foundation that can be compiled and demonstrated through
the existing facade without replacing the Gallery shell.

### Tasks

1. Add and export the three navigation enums and `NavigationEntry` from section 18.
2. Add the semantic navigation tokens/constants needed by the approved components;
   retain existing sidebar-specific declarations until Phase 3.
3. Implement and export `NavigationBackButton`, `NavigationPaneToggleButton` and
   `NavigationContentSurface` with the standalone contracts in section 18.2.
4. Add isolated specimens to the existing Navigation page (page 7) and compile-only
   coverage to `api_probe.slint`, importing only `@quadrant-kit`.
5. Review/adopt the additive API candidate using section 2.3 and update affected
   live documentation, counts and exact-count tests.
6. Run the common gate and the Phase 1 visual/input checks below.

Private item rows, indentation, chevrons and full hierarchy rendering are
implemented together with NavigationView in Phase 2. Phase 1 MUST NOT introduce
a raw private Gallery import, a temporary public test component, a second facade
or a separate runtime harness. These three standalone modules have independent
consumer value and remain public after the rebuild.

### MUST NOT

- remove old `SidebarItem` yet;
- replace main Gallery navigation yet;
- add history/router logic;
- change all page files;
- implement a temporary navigation shell or expose private row mechanics.

### Visual acceptance

Use the existing Navigation page to show the real public modules:

```text
Back visible / hidden
Back enabled / disabled
Toggle visible / hidden
Toggle expanded / compact presentation
Fluent / flat / transparent content surface
Light / dark theme
```

Show conditional mounting in the sample layout so hidden Back/Toggle consumes
zero space. Demonstrate content-surface children and inspect all four corners
and border edges at normal and constrained preview sizes. Wire visible callback
counters; verify mouse, Tab, Enter and Space for enabled controls and suppression
for disabled controls. Full pane geometry and three-level acceptance belong to
Phase 2, where their actual owning component exists.

### Exit criteria

- all new public modules/types compile through the facade and API probe;
- the common gate passes with the reviewed additive baseline;
- old SidebarItem and main Gallery navigation remain usable;
- Back/Toggle standalone behavior and conditional zero-space removal are verified;
- content surface shows only top-left rounding;
- no routing state is introduced.

---

## Phase 2 — Assemble public NavigationView

### Goal

Introduce the complete high-level `NavigationView` API.

### Tasks

1. Implement private item rows, focus/disabled behavior, indentation, chevrons and
   ancestor visibility, then assemble `NavigationView` from them and Phase 1 modules.
2. Wire `items`, `footer_items`, `selected_id`, optional regions, and callbacks.
3. Render content children in the right content host.
4. Implement group expansion request behavior.
5. Implement `destination_group` split behavior.
6. Ensure hidden regions collapse.
7. Ensure compact mode suppresses text appropriately and keeps tooltips/labels.
8. Add Gallery `api_probe.slint` coverage for the new public exports.
9. Add a live NavigationView specimen to the existing Navigation page using only
   `@quadrant-kit`; use controlled sample state and visible callback counters.
10. Add hierarchy/model-construction tests and runtime interaction checks for
    sections 7–8, including invalid inputs, disabled entries and hidden descendants.
11. Update public documentation and review/adopt the additive NavigationView API
    baseline under section 2.3, then run the common gate.

### Public API policy

At this point, add `NavigationView` and approved public navigation types to `ui/kit.slint`.

Do **not** remove `SidebarItem` until Phase 3 proves the new view can replace all Kit/Gallery usage.

Private rows are tested through this public view, never exported for the specimen.
The first implementation must demonstrate both primary and footer models, their
selection/expansion behavior, all optional regions, and the default property values.

### Test matrix

At minimum exercise:

| Case | Expected |
|---|---|
| no Back | no blank Back row |
| Back disabled | visible but inert |
| no Search | no search gap |
| no Footer | no footer gap/separator |
| 1 level | no unnecessary chevrons |
| 2 levels | correct expand/indent |
| 3 levels | correct second indent |
| compact | icons visible, labels hidden/tooltips available |
| fluent content | only top-left radius |
| transparent content | host background visible |
| flat content | content background, no frame or rounded corners |
| no pane title | no header gap |
| destination_group | label invokes; chevron requests expansion only |
| host declines a request | selection/expansion/pane mode do not change locally |
| disabled row / group / Back | no pointer, keyboard or accessibility callback |
| collapsed descendant | absent from layout and focus order |
| invalid hierarchy / duplicate ID | rejected or omitted safely; no misleading selection |
| long label / absent icon | no chevron overlap; meaningful label remains available |

### Exit criteria

- `NavigationView` is usable independently;
- callbacks are observable in a specimen;
- there is still no internal history/page creation;
- Gallery still compiles with old shell plus the new NavigationView demonstration.
- the common gate passes with the reviewed additive baseline;
- the full matrix above is exercised in Light/Dark and expanded/compact where
  applicable, with native keyboard evidence for the minimum contract;
- Phase 2 evidence is reviewed before changing the main shell in Phase 3.

---

## Phase 3 — Intentional navigation API cutover; remove SidebarItem

### Goal

Make the new navigation architecture canonical inside Kit.

### Tasks

1. Replace the main manual sidebar/content layout in `gallery/ui/gallery.slint`
   with the proven public NavigationView. Remove Catalog filter and the separate
   Pages heading/list. Initially supply one flat model reaching all eight existing
   pages; stable string IDs map to the existing Gallery-local numeric page values.
2. Keep page creation/state in Gallery. Put the existing page ScrollView and page
   switch inside NavigationView's content children, retaining current page padding
   until the atomic scroll transfer in Phase 5. Remove the old full content rectangle
   so the Fluent surface remains visible.
3. Preserve Theme/preview controls in a Gallery-local toolbar outside the view.
   Enable pane toggle with host-owned mode. Keep Back/Search hidden until Phase 4
   supplies their application behavior; do not show inert navigation utilities.
4. Replace the old Navigation page specimen/code sample and API probe instance,
   and every other live SidebarItem consumer, with the appropriate new public API.
5. Build Gallery and verify all eight routes, Theme/preview controls, compact mode
   and the NavigationView specimen before deleting the old component.
6. Remove SidebarItem from the facade, delete its implementation, and remove
   sidebar-specific tokens only after all live uses are migrated. Preserve generic
   `content_radius` and other tokens still used by unrelated controls.
7. Update docs/probe/catalog descriptions and review/adopt the removal baseline
   under section 2.3. Run the common gate again against the final phase state.

There is no exception for the old main shell: all live consumers must switch
before deletion. The minimal flat catalog is a temporary information architecture
inside the real NavigationView, not a compatibility component or second sidebar.
Phase 4 enriches this shell; it does not perform a second shell replacement.

### API baseline procedure

Use section 2.3's deliberate review flow, comparing against the already adopted
Phase 2 baseline:

```console
python scripts/check_ui_boundaries.py --write-baseline target/navigation-phase3/kit_api_candidate.json
git diff --no-index scripts/kit_api_v1.json target/navigation-phase3/kit_api_candidate.json
```

Review:

- removed `SidebarItem`;
- preservation of the already reviewed new navigation exports;
- property directions/types/defaults;
- enum order;
- struct fields;
- any intentionally renamed constants/tokens.

Only after review, deliberately adopt the new baseline.

Do not make the guard green by blindly copying the candidate.

### Exit criteria

- public `SidebarItem` no longer exists;
- new NavigationView API is the only canonical high-level navigation API;
- boundary/API guard is green with an intentionally reviewed baseline;
- old sidebar code is gone rather than hidden as dead compatibility code.
- main Gallery uses NavigationView exclusively and all eight existing pages remain reachable;
- Catalog filter is absent; current snapshot page IDs 0–7 still select their pages;
- the common gate and focused main-shell interaction/render checks pass.

---

## Phase 4 — Hierarchical catalog, search and Back in the new Gallery shell

### Goal

Enrich the working Phase 3 NavigationView shell with Gallery-owned navigation
behavior, without replacing its layout again.

### Tasks

1. Replace Phase 3's flat model with a hierarchical catalog of the existing pages.
2. Keep aggregate pages as real destinations until Phase 6 supplies split pages;
   do not create dead links to unimplemented future destinations.
3. Keep Catalog filter and the manual fixed sidebar absent.
4. Add Gallery-local route/history/search/expansion state and model validation.
5. Continue using `NavigationView` as the sole main navigation shell.
6. Show in Gallery:
   - Back button;
   - Pane toggle;
   - Search;
   - primary navigation hierarchy;
   - footer item(s) if useful.
7. Keep routing/history in Gallery state only.
8. Render current Gallery page inside NavigationView `@children` content host.
9. Use `NavigationContentSurfaceMode.fluent`.
10. Ensure content page is transparent.
11. Keep the Gallery toolbar usable; retain native window decorations for this
    phase. Custom chrome is optional Phase 7 polish under section 16.
12. Run the common gate and verify forward selection, repeated selection, Back,
    empty history, search submission/result selection/clearing and expansion.

### Gallery Back behavior

Implement a small Gallery-local history sufficient for Back demonstration.

Only a successful change to a different valid destination pushes the previous ID.
Selecting the current destination does not create a duplicate. Back pops without
pushing a new entry, updates selection and reveals its ancestors; an empty history
disables Back. Group expansion and search-text edits do not create history entries.
If a search filter hides the Back target, Gallery clears the query when going Back.

It MUST NOT be added to Kit NavigationView.

### Search behavior

Implement Gallery-local case-insensitive catalog search.

Initial acceptable behavior:

- search component/page titles and optional keywords;
- while query is non-empty, show matching destinations or a search-result list;
- selecting a result navigates through Gallery logic;
- clearing search restores normal hierarchy.

Use a flat result model with root-depth destination entries, preserving stable
destination IDs; do not pass orphaned children from a filtered tree. No matches
shows a non-interactive empty-results message and leaves the current page intact.
Search must preserve the normal tree's expansion state, and Back/search results
must resolve through the same Gallery-local route table. Section 8.6 defines how
compact mode makes the full search/catalog available.

Do not add fuzzy-search dependencies.

### Exit criteria

The main Gallery visually has:

```text
Title/window area
Navigation pane
    Back
    Pane toggle
    Search
    hierarchical component catalog
Right content surface
    top-left radius only
    consistent background
    current page
```

The old `Catalog filter + Pages` dual structure no longer exists.
All links resolve to implemented pages, history/search behavior passes the checks
above, and the common gate passes. A full one-component-per-page taxonomy is not
required until Phase 6.

---

## Phase 5 — Rebuild GalleryPage and Specimen system

### Goal

Make every Gallery destination look like part of one documentation application.

### Tasks

1. Refactor `GalleryPage` into the canonical page shell.
2. Move page-level scrolling into `GalleryPage`.
3. Apply standard padding and responsive compact padding.
4. Create/upgrade the page header with:
   - title;
   - description;
   - optional stability/status;
   - actions region.
5. Move Theme/preview/documentation utilities out of primary navigation and into page/shell actions where appropriate.
6. Refactor `Specimen` to WPF-UI-like preview + source/details structure.
7. Preserve keyboard/accessibility evidence in a lower-emphasis details region.
8. Ensure pages do not paint a competing full-page background.

### Atomic scroll ownership transfer

Remove the temporary Gallery-shell content ScrollView introduced in Phase 3 in
the same change that adds scrolling/padding to GalleryPage. Convert every existing
page together to the new fill-viewport/content-height contract; do not leave some
pages depending on shell scrolling. Component-local text/list scrolling remains
valid, but there must be exactly one page-level ScrollView for each destination.

NavigationView and its content surface remain unchanged. Preserve transparent
page backgrounds and specimen callback counters. The source panel must have a
keyboard-operable toggle, collapsed zero-height body and readable selectable text;
it need not be a syntax-highlighting editor. Page actions are Gallery-local: use
explicit page metadata/callbacks or a shared Gallery toolbar rather than assuming
Slint supports multiple named `@children` slots.

Run the common gate and visit all eight existing destinations in Light/Dark,
normal/minimum window sizes. Verify short pages, overflowing pages, source panel
expansion, page switching and retained input/counter observability. Resetting a
page-local counter when a page is recreated remains permitted; no page cache is
introduced to preserve it.

### Acceptance visual

Every migrated page MUST share:

- same content background;
- same left/right page padding;
- same title baseline;
- same section spacing;
- same specimen width behavior;
- same source/details presentation.

No page may invent its own full-screen rectangle unless it is explicitly demonstrating a surface component.
The common gate passes, all destinations use GalleryPage, and no obsolete shell
page-scroll/padding wrapper remains.

---

## Phase 6 — Catalog migration and component-page split

### Goal

Turn Gallery into a real component browser rather than eight broad aggregate pages.

### Tasks

1. Create `Home` page.
2. Create `All components` page.
3. Migrate current aggregate pages into the new taxonomy.
4. Split interactive public components into individual pages where useful.
5. Keep foundation/global concepts grouped where a dedicated visual page would be artificial.
6. Add dedicated **NavigationView** page demonstrating modular configurations.
7. Drive navigation, All components, route resolution and search from the same
   Gallery-local catalog metadata. Keep overview pages only when they have a
   documented purpose; do not duplicate destination IDs or copy component implementations.
8. Migrate the snapshot/configuration interface as described below, then run the
   common gate and route/capture coverage checks.

### Route and snapshot migration

Stable string destination IDs are the canonical Gallery route keys. Keep existing
`QUADRANT_GALLERY_PAGE=0..7` as documented Gallery-only aliases, mapped explicitly
to the retained overview or successor destination. Preserve page 4's Controls
overview for the existing CI smoke scenario; page 7 resolves to NavigationView.
This does not impose SidebarItem compatibility on Kit or create a router API.

Add `QUADRANT_GALLERY_DESTINATION` and capture option `--destination` for all new
pages. When both numeric page and string destination are explicitly supplied,
reject the ambiguous request rather than silently pick one. Apply the same rule
to the Python/PowerShell capture entries. With neither supplied, use Home.
Keep the old numeric validator scoped to aliases; do not silently renumber them
or widen `range(8)` into an unrelated new ordering.

Update `gallery/src/config.rs`, host route initialization, the Python capture
script, its PowerShell wrapper, fixtures, docs and affected CI calls together.
Capture manifests/reuse identity must include the resolved stable destination,
and any changed scenario schema must invalidate old reuse. Add tests for all
aliases, invalid IDs, option conflicts and distinct destination identities. Capture
Home, All components and NavigationView through the new interface; smoke-capture
the retained page-4 alias to prove existing automation still works.

### NavigationView Gallery page MUST show

At minimum:

1. Standard expanded navigation.
2. Navigation **without Back button**.
3. Navigation without Search.
4. Compact navigation.
5. One-level navigation.
6. Two-level navigation.
7. Three-level navigation.
8. `destination_group` behavior.
9. Footer on/off.
10. Fluent/flat/transparent content-surface modes.
11. Enabled/disabled Back state.
12. Light/dark rendering.

This page is the canonical proof that NavigationView features are optional and composable.

### Exit criteria

- catalog can reach every intended Gallery destination;
- no duplicate navigation system exists;
- broad old pages are either intentionally retained as overview pages or removed;
- All Components accurately reflects public visual components.
- every catalog destination and snapshot alias resolves, no stale 28-name/eight-page
  live claims remain, and all new destinations have the unified page template;
- the common gate and the route/snapshot checks above pass.

---

## Phase 7 — Keyboard, accessibility, responsive and visual polish

### Goal

Raise the new shell from “works” to “Kit-quality.”

### Tasks

1. Verify Tab/Enter/Space behavior on all navigation controls.
2. Add/verify directional navigation where practical.
3. Ensure hidden descendants cannot receive focus.
4. Verify compact-mode tooltips.
5. Verify focus ring against both content/pane backgrounds.
6. Verify 100%, 125%, 150%, 200%, and unusual DPI where feasible.
7. Verify minimum Gallery window size.
8. Verify pane widths and long labels.
9. Verify long localized-like strings do not overlap chevrons.
10. Verify scrollbar placement does not visually collide with content frame.
11. Compare light/dark screenshots.
12. Check only-left-top content radius at multiple sizes.

### Responsive policy

Keep `NavigationView` itself simple.

If Gallery wants responsive pane behavior, bind the view’s `pane_mode` from Gallery/window width.

Do not add a complex internal adaptive state machine unless concrete tests prove it is necessary.

Run the common gate after polish. Required local visual/input evidence covers
Light/Dark, expanded/compact, minimum/default window sizes, long labels, iconless
entries, deep selected entries, disabled controls, all content modes and hidden
regions. Recheck focus after collapse and the source-panel toggle. Use simulated
100/125/150/200/225% rendering where supported and label it as simulated;
real monitor transitions and unavailable platforms remain separately NOT_RUN.

Native window decorations remain the accepted default. Attempt custom chrome
only if dragging, resizing, minimize/maximize/restore/close and keyboard behavior
can be verified on the available host; otherwise retain native chrome and report
custom chrome as deferred. This does not block NavigationView acceptance.

### Exit criteria

- the common gate and required local visual/input matrix pass;
- hidden/disabled entries cannot invoke actions or retain inaccessible focus;
- content edges, long labels, minimum size and compact controls are verified;
- directional-key support and unavailable native/platform coverage are reported
  explicitly, without presenting optional/deferred work as tested;
- native chrome remains correct, or any replacement has the evidence above.

---

## Phase 8 — Documentation, guard finalization, and full validation

### Goal

Finish the standalone Kit construction with truthful documentation and reproducible checks.

### Tasks

1. Update `README.md` Gallery description/screenshots if applicable.
2. Update `docs/PUBLIC_API.md`.
3. Update `docs/GALLERY.md`.
4. Update `CHANGELOG.md` with the intentional navigation API redesign.
5. Update API baseline only to the reviewed final contract.
6. Ensure `api_probe.slint` compiles all new public exports.
7. Ensure live `SidebarItem` imports, exports, instances, current API declarations
   and runnable examples are absent. Preserve the SPEC, changelog, historical
   phase reports and extraction provenance; classify their references as historical.
8. Search for old catalog-filter shell remnants.
9. Reconcile current API counts/probe coverage and exact-count tests with the
   reviewed active baseline. Do not rewrite earlier extraction evidence as current.
10. Run full validation and record results against the actual checked source.

### Full validation

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/check_ui_boundaries.py
python -m unittest discover -s scripts/tests -p "test_*.py"
cargo build --locked -p quadrant-kit-gallery
python scripts/verify_distribution.py --package
cargo package --locked -p quadrant-kit --list
```

The two package/list checks above do **not** build or verify a `.crate` archive.
The final distribution gate additionally requires a clean committed local source
snapshot containing the reviewed implementation and required static files:

```console
cargo package --locked -p quadrant-kit
python scripts/verify_distribution.py --package --archive target/package/quadrant-kit-0.1.0.crate
```

Use the actual package version/path if an explicitly authorized version change
has occurred; this rebuild itself does not require one. Prepare the clean snapshot
from reviewed phase changes only, without committing private files or unknown
work. Do not use `--allow-dirty` to claim the clean archive gate passed. Local
commits/package generation do not authorize a push, retained tag or publication.
If a clean snapshot cannot yet be prepared, mark archive verification NOT_RUN and
leave the distribution gate incomplete rather than claim full completion.

Where exclusive checkout/build access is available:

```console
python scripts/verify_incremental.py
```

Run MSRV validation where required by current repository policy:

```console
cargo +1.92.0 build --locked -p quadrant-kit -p quadrant-kit-gallery --target-dir target/msrv-1.92
```

Report unavailable platform/native tests as **NOT_RUN**, not PASS.

Final local acceptance requires the common gate, actual package/archive checks,
the available-host interaction/render matrix and an actual Windows MSRV build
when running on the current supported Windows environment. Run incremental
verification only with exclusive access; if unavailable, record the missing
condition. Remote CI/consumer/publication remains outside this construction.
Report local implementation completion separately from any unavailable optional
platform checks; required local gates cannot be waived by a NOT_RUN label.

### Exit criteria

- current docs, runnable examples, public probe, catalog and active API baseline agree;
- all required local gates above pass against identified source states;
- archived package bytes match the reviewed source closure and preserve licensing;
- live legacy imports/implementations and obsolete shell wrappers are absent;
- final evidence distinguishes PASS, FAIL and NOT_RUN, and historical publication
  evidence is not claimed for the new local implementation;
- no Tasks changes, remote publication or unreviewed baseline refresh occurred.

---

# 22. Phase dependency map

Codex should follow this dependency order:

```text
Phase 0  Baseline
   ↓
Phase 1  Public types/tokens/standalone modules + additive API review
   ↓
Phase 2  Private hierarchy + public NavigationView + additive API review
   ↓
Phase 3  Minimal main-shell switch → remove SidebarItem → API review
   ↓
Phase 4  Hierarchical Gallery catalog / search / Back
   ↓
Phase 5  Unified GalleryPage / Specimen
   ↓
Phase 6  Catalog/page migration
   ↓
Phase 7  Interaction + visual polish
   ↓
Phase 8  Docs + full validation
```

Do not switch the main shell in Phase 3 before the Phase 2 NavigationView
demonstration and common gate pass and their evidence has been reviewed.

Do not delete SidebarItem before every live consumer, including the main shell,
uses the replacement and Gallery compiles. API review/adoption occurs in each
phase that changes public declarations, never as deferred guard repair.

Do not split all Gallery pages before the new Gallery shell and page template are stable.

---

# 23. Codex implementation discipline

For each phase, Codex MUST:

1. State the phase being executed.
2. Inspect relevant files before modifying them.
3. Make only changes necessary for that phase.
4. Avoid speculative infrastructure for future phases.
5. Run the required phase checks.
6. Report:
   - files changed;
   - public API changes;
   - validation commands run;
   - PASS / FAIL / NOT_RUN;
   - known limitations;
   - whether exit criteria are satisfied.
7. Stop at the phase boundary unless explicitly instructed to continue.

This staged behavior is intentional: it makes architecture review possible before later work depends on it.

---

# 24. Forbidden shortcuts

The following shortcuts are explicitly prohibited:

- Keep `SidebarItem` public “just in case Tasks needs it.”
- Put route history into `NavigationView` because Gallery needs Back.
- Make Back permanently visible.
- Make Search permanently visible.
- Reserve empty space for hidden modules.
- Implement only a flat list and fake hierarchy with section labels.
- Support unlimited recursion in v1.
- Create a normal four-corner rounded content rectangle and call it close enough.
- Give every Gallery page its own opaque full-window background.
- Keep Catalog Filter next to hierarchical navigation.
- Add a ScrollView automatically around all NavigationView content.
- Duplicate NavigationView shell code inside Gallery.
- Export every private row/chevron helper as public API.
- Refresh API baselines without reviewing the diff.
- Change Slint/toolchain versions as part of this rebuild.
- Modify Quadrant-Tasks in this construction.

---

# 25. Acceptance criteria / Definition of Done

The construction is complete only when all of the following are true.

## 25.1 NavigationView

- [ ] `NavigationView` is a public canonical Kit component.
- [ ] Public `SidebarItem` is removed.
- [ ] Level 1 / Level 2 / Level 3 navigation works.
- [ ] Expansion and selection are separate.
- [ ] Destination, group, and destination-group semantics are observable.
- [ ] Back button is optional.
- [ ] Back disabled state works.
- [ ] Pane toggle is optional.
- [ ] Search is optional.
- [ ] Footer is optional.
- [ ] Hidden modules reserve no empty space.
- [ ] Expanded and compact pane modes are usable.
- [ ] Content surface supports fluent/flat/transparent modes.
- [ ] Fluent content surface has only a top-left rounded corner and top/left border.
- [ ] NavigationView does not own history, routes, page creation, or page cache.

## 25.2 Gallery

- [ ] Gallery uses the public `NavigationView` as its only main navigation shell.
- [ ] Catalog Filter is removed.
- [ ] Hierarchical navigation replaces the old filter/page split.
- [ ] Back is demonstrated using Gallery-local history.
- [ ] Search is demonstrated using Gallery-local catalog search.
- [ ] Right content background is consistent across all pages.
- [ ] Page background is transparent.
- [ ] `GalleryPage` is the canonical page template.
- [ ] GalleryPage owns page-level scrolling.
- [ ] Specimen visual structure is unified.
- [ ] A dedicated NavigationView page demonstrates modular configurations.
- [ ] Home and All Components destinations exist.
- [ ] Gallery visually approximates the WPF UI Gallery layout without copying application-specific internals.

## 25.3 Quality / repository

- [ ] `ui/kit.slint` exports only reviewed new public APIs.
- [ ] The active `scripts/kit_api_v1.json` is deliberately reviewed in each phase that changes the API.
- [ ] `gallery/ui/api_probe.slint` exercises all new public exports.
- [ ] docs are updated.
- [ ] boundary tests pass.
- [ ] Gallery builds.
- [ ] package/distribution checks pass.
- [ ] unavailable platform checks are reported honestly as NOT_RUN.
- [ ] no Quadrant-Tasks compatibility code is introduced.

---

# 26. Final architectural statement

The final intended boundary is:

```text
Quadrant-Kit NavigationView
    = navigation layout
    + optional Back UI
    + optional pane toggle
    + optional search UI
    + optional footer UI
    + three-level hierarchy rendering
    + selection/expansion presentation
    + content-surface framing
    + callbacks

Quadrant-Kit NavigationView
    ≠ router
    ≠ history manager
    ≠ page factory
    ≠ page cache
    ≠ application shell lifecycle
```

And the final Gallery boundary is:

```text
Gallery
    = application-specific routing/history/search
    + NavigationView configuration
    + unified GalleryPage template
    + unified Specimen template
    + component catalog
    + visual/documentation showcase
```

The new Gallery should serve as both the visual reference for Quadrant-Kit and the proof that Kit modules can be composed differently: a full documentation Gallery may enable Back/Search/Footer, while an application such as Quadrant-Tasks may later disable those regions without forking or reimplementing the navigation system.
