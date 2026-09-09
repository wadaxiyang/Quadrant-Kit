# Public API — local NavigationView / Gallery rebuild

`ui/kit.slint` is the only supported Slint entry. All 35 public names below participate in `gallery/ui/api_probe.slint`, compiled through Gallery. Root Rust API is limited to `SLINT_LIBRARY_NAME: &str` and `slint_library_path() -> PathBuf`; generated Slint runtime types belong to the consumer. The navigation rebuild additions and SidebarItem removal are local, unpublished work; the retained extraction source in CONSUMER_GUIDE.md still exposes its original API.

Signatures below are copied from the current sources, including declared defaults. They describe explicitly declared API; inherited Slint element properties still apply. Internal filenames are links for reading implementation, not additional supported import entry points. `scripts/kit_api_v1.json` records the reviewed current contract (P3: 35 names, 234 properties, 20 callbacks, seven enums and one struct). These counts and historical signatures are not permanent invariants. APIs can be added, removed, renamed or adjusted with a concrete reason, synchronized current callers/docs/probes and an explicitly reviewed snapshot. Each version keeps one implementation, without old aliases or compatibility branches. The guard reports signature and default-expression differences separately; see [validation](VALIDATION.md) for its scope and explicit update process.

Phase 3 removes SidebarItem, Theme.sidebar_bg and the two UiConstants.sidebar_* widths.
The new pane tokens retain their resolved transparent/54 px values; content_radius
and other shared tokens remain. See [Phase 3 cutover](NAVIGATION_REBUILD_PHASE3.md).

## Coverage and behavior

- Home and All components share the Gallery catalog: 25 destinations cover all 21 public visual components. Theme / Colors, Typography and Icons provide conceptual guidance for globals and resources.
- Controls overview compares inputs and commands. Dedicated FluentButton, IconButton, SegmentButton, FluentTextField and FluentTextArea pages provide their specimens. Text editing remains delegated to std-widgets.
- SurfaceCard, Badge, MetricCard and SettingRow have dedicated pages. Non-interactive reference specimens are labeled Reference.
- Feedback overview compares outcomes; ToastHost, ModalManager and TooltipHost have dedicated pages. ModalManager is one confirmation overlay, with Escape/Return handling; complete focus containment, restoration, nested modal stacks and screen-reader behavior remain unverified.
- NavigationView demonstrates controlled three-level primary and two-level footer models. Navigation foundations composes NavigationBackButton, NavigationPaneToggleButton and NavigationContentSurface. PageHeader, SectionHeader, EmptyState and WindowControlButton have dedicated destinations.
- IconButton, PageHeader and WindowControlButton actions have visible counters in the running Gallery. [Reproduction steps and native results](GALLERY.md#observable-action-specimens) cover mouse/Enter/Space activation and IconButton disabled suppression; the compile-only probe is separate evidence.
- Navigation keyboard/focus polish and its Windows input/render evidence are recorded in [Phase 7](NAVIGATION_REBUILD_PHASE7.md). Up/Down/Home/End item traversal is not implemented; full IME/screen-reader/platform coverage remains unverified. Screenshot rendering is not interaction or accessibility proof.

The historical extraction removed Branding, TaskRowShell, InboxItem and InboxPane, plus Q1–Q4 colors, Typography.timer, UiConstants.focus_wide_breakpoint and 11 product icon aliases. The subsequent local navigation rebuild intentionally replaces SidebarItem with the controlled NavigationView API. Its reviewed 35-name contract remains unchanged since Phase 3; Phase 7 refines focus and keyboard behavior without changing signatures/defaults.

## Declarations

### Elevation

[Source](../ui/foundation/theme.slint)

```slint
export global Elevation {
    out property <color> card_surface: Theme.card_bg;
    out property <color> card_border: Theme.border_color;
    out property <color> card_hover_surface: Theme.card_hover;
    out property <color> card_pressed_surface: Theme.selected_bg;
    out property <color> card_selected_surface: Theme.selected_bg;
    out property <color> card_selected_border: Theme.accent;
    out property <length> card_blur: 4px;
    out property <color> card_shadow: Theme.dark_mode ? #00000038 : #00000018;
    out property <length> popup_blur: 12px;
    out property <color> popup_shadow: #00000044;
    out property <color> dialog_surface: Theme.card_bg;
    out property <color> dialog_border: Theme.border_color;
    out property <color> dialog_overlay: #00000080;
    out property <length> dialog_blur: 20px;
    out property <color> dialog_shadow: #00000055;
}
```

### Motion

[Source](../ui/foundation/theme.slint)

```slint
export global Motion {
    out property <duration> fast: 100ms;
    out property <duration> standard: 160ms;
    out property <duration> slow: 200ms;
}
```

### Theme

[Source](../ui/foundation/theme.slint)

```slint
export global Theme {
    in-out property <ThemeMode> mode: ThemeMode.system;
    in-out property <bool> system_dark: false;
    in-out property <string> ui_font_family: "";
    out property <bool> dark_mode: mode == ThemeMode.dark || (mode == ThemeMode.system && system_dark);
    out property <color> background: dark_mode ? #202020 : #f3f3f3;
    out property <color> content_bg: dark_mode ? #1e1e1e : #ffffff;
    out property <color> card_bg: dark_mode ? #2d2d30 : #ffffff;
    out property <color> card_bg_muted: dark_mode ? #27272a : #fafafa;
    out property <color> text_primary: dark_mode ? #eeeeee : #1a1a1a;
    out property <color> text_secondary: dark_mode ? #bbbbbb : #666666;
    out property <color> text_disabled: dark_mode ? #777777 : #8a8a8a;
    out property <color> text_tertiary: dark_mode ? #9d9d9d : #707070;
    out property <color> border_color: dark_mode ? #3e3e42 : #e6e8eb;
    out property <color> divider: dark_mode ? #38383c : #e8e8e8;
    out property <color> main_color: dark_mode ? #55555a : #c6c6c6;
    out property <color> hover_bg: dark_mode ? #2a2d2e : #eaeaea;
    out property <color> selected_bg: dark_mode ? #37373d : #e6e6e6;
    out property <color> navigation_pane_bg: transparent;
    out property <color> navigation_content_bg: content_bg;
    out property <color> navigation_content_border: border_color;
    out property <color> navigation_item_hover_bg: hover_bg;
    out property <color> navigation_item_selected_bg: selected_bg;
    out property <color> navigation_item_foreground: text_secondary;
    out property <color> navigation_item_foreground_selected: text_primary;
    out property <color> card_hover: dark_mode ? #323237 : #f7f7f7;
    out property <color> accent: dark_mode ? #60cdff : #005fb8;
    out property <color> accent_foreground: dark_mode ? #000000 : #ffffff;
    out property <color> accent_hover: dark_mode ? #4cc2ff : #005a9e;
    out property <color> accent_pressed: dark_mode ? #47b1df : #004578;
    out property <color> focus_ring: dark_mode ? #60cdff : #005fb8;
    out property <color> control_bg: dark_mode ? #2d2d30 : #ffffff;
    out property <color> control_bg_hover: dark_mode ? #35353a : #f7f7f7;
    out property <color> input_bg: dark_mode ? #252526 : #ffffff;
    out property <color> input_border: dark_mode ? #4b4b50 : #d3d3d3;
    out property <color> danger: dark_mode ? #ff6b6b : #c42b1c;
    out property <color> danger_bg: dark_mode ? #5a1d1d : #fdf3f2;
    out property <color> success: dark_mode ? #4ade80 : #107c10;
    out property <color> warning: dark_mode ? #f5c242 : #9d5d00;
    out property <color> icon_neutral: dark_mode ? #c7c7c7 : #5f5f5f;
    out property <color> scrollbar_bg: dark_mode ? #2a2d2e : #f0f0f0;
    out property <color> scrollbar_thumb: dark_mode ? #5d5d5d : #c1c1c1;
    out property <color> scrollbar_thumb_hover: dark_mode ? #6d6d6d : #a8a8a8;
    out property <color> icon_color_navigation: dark_mode ? #38bdf8 : #0284c7;
    out property <color> icon_color_success: dark_mode ? #4ade80 : #16a34a;
    out property <color> icon_color_info: dark_mode ? #60a5fa : #2563eb;
    out property <color> icon_color_indigo: dark_mode ? #818cf8 : #4f46e5;
    out property <color> icon_color_orange: dark_mode ? #fb923c : #f97316;
    out property <color> icon_color_emerald: dark_mode ? #34d399 : #059669;
    out property <color> icon_color_slate: dark_mode ? #94a3b8 : #475569;
}
```

### ThemeMode

[Source](../ui/foundation/theme.slint)

```slint
export enum ThemeMode { system, light, dark }
```

### Typography

[Source](../ui/foundation/theme.slint)

```slint
export global Typography {
    out property <length> caption: 12px;
    out property <length> body: 14px;
    out property <length> body_large: 18px;
    out property <length> subtitle: 20px;
    out property <length> title: 28px;
    out property <int> regular: 400;
    out property <int> semibold: 600;
}
```

### UiConstants

[Source](../ui/foundation/constants.slint)

```slint
export global UiConstants {
    out property <length> space_2: 2px;
    out property <length> space_4: 4px;
    out property <length> space_6: 6px;
    out property <length> space_8: 8px;
    out property <length> space_10: 10px;
    out property <length> space_12: 12px;
    out property <length> space_16: 16px;
    out property <length> space_20: 20px;
    out property <length> space_24: 24px;
    out property <length> space_28: 28px;
    out property <length> space_32: 32px;
    out property <length> space_40: 40px;
    out property <length> title_bar_height: 40px;
    out property <length> navigation_item_height: 40px;
    out property <length> navigation_item_padding: 11px;
    out property <length> navigation_icon_slot_width: 24px;
    out property <length> navigation_icon_size: 20px;
    out property <length> navigation_pane_compact_width: 54px;
    out property <length> navigation_pane_default_width: 260px;
    out property <length> navigation_item_indent: 28px;
    out property <length> navigation_content_radius: content_radius;
    out property <length> button_height: 32px;
    out property <length> icon_button_size: 32px;
    out property <length> input_height: 36px;
    out property <length> content_radius: 8px;
    out property <length> control_radius: 4px;
    out property <length> page_padding_wide: 28px;
    out property <length> page_padding_medium: 20px;
    out property <length> page_padding_compact: 16px;
    out property <length> page_padding: page_padding_wide;
    out property <length> compact_breakpoint: 680px;
    out property <length> wide_breakpoint: 960px;
}
```

### FluentIcons

[Source](../ui/foundation/fluent_icons.slint)

```slint
export global FluentIcons {
    out property <image> menu: @image-url("../../assets/icons/navigation-20-regular.svg");
    out property <image> settings_regular: @image-url("../../assets/icons/settings-20-regular.svg");
    out property <image> settings_filled: @image-url("../../assets/icons/settings-20-filled.svg");
    out property <image> about_regular: @image-url("../../assets/icons/about-20-regular.svg");
    out property <image> about_filled: @image-url("../../assets/icons/about-20-filled.svg");
    out property <image> add: @image-url("../../assets/icons/add-16-regular.svg");
    out property <image> dismiss: @image-url("../../assets/icons/dismiss-16-regular.svg");
    out property <image> minimize: @image-url("../../assets/icons/subtract-16-regular.svg");
    out property <image> maximize: @image-url("../../assets/icons/maximize-16-regular.svg");
    out property <image> restore_window: @image-url("../../assets/icons/restore-window-16-regular.svg");
    out property <image> theme_system: @image-url("../../assets/icons/theme-system-16-regular.svg");
    out property <image> theme_light: @image-url("../../assets/icons/theme-light-16-regular.svg");
    out property <image> theme_dark: @image-url("../../assets/icons/theme-dark-16-regular.svg");
    out property <image> delete: @image-url("../../assets/icons/delete-16-regular.svg");
    out property <image> edit: @image-url("../../assets/icons/edit-16-regular.svg");
    out property <image> complete: @image-url("../../assets/icons/complete-16-regular.svg");
    out property <image> chevron_up: @image-url("../../assets/icons/chevron-up-16-regular.svg");
    out property <image> chevron_down: @image-url("../../assets/icons/chevron-down-16-regular.svg");
    out property <image> more_horizontal: @image-url("../../assets/icons/more-horizontal-16-regular.svg");
    out property <image> folder_open: @image-url("../../assets/icons/folder-open-16-regular.svg");
    out property <image> backup: @image-url("../../assets/icons/backup-16-regular.svg");
    out property <image> warning: @image-url("../../assets/icons/warning-16-regular.svg");
    out property <image> error: @image-url("../../assets/icons/error-16-regular.svg");
    out property <image> info: @image-url("../../assets/icons/info-16-regular.svg");
    out property <image> calendar: @image-url("../../assets/icons/calendar-16-regular.svg");
    out property <image> clock: @image-url("../../assets/icons/clock-16-regular.svg");
    out property <image> recurrence: @image-url("../../assets/icons/recurrence-16-regular.svg");
    out property <image> heart_filled: @image-url("../../assets/icons/heart-16-filled.svg");
    out property <image> status_success: @image-url("../../assets/icons/success-24-regular.svg");
    out property <image> status_info: @image-url("../../assets/icons/info-24-regular.svg");
    out property <image> status_warning: @image-url("../../assets/icons/warning-24-regular.svg");
    out property <image> status_error: @image-url("../../assets/icons/error-24-regular.svg");
}
```

### FluentIcon

[Source](../ui/primitives/fluent_icon.slint)

```slint
export component FluentIcon inherits Rectangle {
    in property <image> source;
    in property <length> size: 16px;
    in property <brush> icon_color: Theme.icon_neutral;
    in property <length> optical_offset_x: 0px;
    in property <length> optical_offset_y: 0px;
}
```

### TooltipHost

[Source](../ui/primitives/tooltip_host.slint)

```slint
export component TooltipHost inherits Rectangle {
    in property <string> text;
    in property <bool> shown: false;
}
```

### SurfaceCard

[Source](../ui/primitives/surface_card.slint)

```slint
export component SurfaceCard inherits Rectangle {
    in property <bool> interactive: false;
    in property <bool> elevated: false;
    in property <bool> selected: false;
    in property <bool> enabled: true;
    in property <string> accessible_name;
    callback clicked;
}
```

### Badge

[Source](../ui/primitives/badge.slint)

```slint
export component Badge inherits Rectangle {
    in property <string> text;
    in property <BadgeKind> kind: BadgeKind.neutral;
}
```

### BadgeKind

[Source](../ui/primitives/badge.slint)

```slint
export enum BadgeKind { neutral, accent, success, warning, danger }
```

### FluentButton

[Source](../ui/primitives/fluent_button.slint)

```slint
export component FluentButton inherits Rectangle {
    in property <string> text;
    in property <image> icon;
    in property <string> accessible_name;
    in property <bool> primary: false;
    in property <bool> danger: false;
    in property <bool> enabled: true;
    out property <bool> has-focus: command.has-focus;
    out property <bool> pressed: command.pressed;
    callback clicked;
}
```

The visible std Button owns pointer, keyboard, focus, disabled visuals, animation
and the sole accessible button node. `clicked` forwards once per native activation
only when enabled; programmatic changes emit no command. Native key-repeat policy
is retained. `focus()` forwards to the native owner. `has-focus` is the native
focus flag and can stay true while disabled; native focus visuals/commands remain
disabled. `pressed` is the native pointer-pressed output, not a synthetic keyboard
or preview state. There is no checkable/selected application state in this wrapper.

The empty native control is 32×32 logical pixels. Defaults use native content
minimums; explicit width/height remain host inputs. Long labels need sufficient
host width (no automatic wrapping or truncation API); abbreviate the visible label
and set `accessible_name` to the full action if necessary. The P2 negative sizing
case confirms that forcing 160px on a longer label can paint text outside both
std Button and Kit bounds; this is not a supported automatic-elision contract.
An empty/invalid image
renders no icon; valid icons use native 20px geometry and native text-color tint.
For an icon-only command, provide `accessible_name`; otherwise the text is its
accessible label. The component has no supported arbitrary-content slot.

**P2 Breaking changes:** `show_icon` is removed (image presence decides), `accent`
is removed (choose normal or primary), and all three `preview_*` inputs are removed.
`has-focus`, `pressed`, and `accessible_name` are added. Text/icon/primary/danger/
enabled and clicked retain their directions; the public Rectangle base is retained.
`danger` now chooses a neutral native button even when primary is true, with a
passive red outline 2px outside its bounds. It does not recreate the old red fill
or modify the shared Palette. Leave 2px surrounding space and use an explicit
hazard label. Disabled danger outlines use Theme.text_tertiary. Font,
focus and hover/press visuals follow native Fluent and the host Palette/font.

### IconButton

[Source](../ui/primitives/icon_button.slint)

```slint
export component IconButton inherits Rectangle {
    in property <image> icon;
    in property <string> tooltip;
    in property <string> accessible_name;
    in property <bool> danger: false;
    in property <bool> enabled: true;
    out property <bool> has-focus: command.has-focus;
    out property <bool> pressed: command.pressed;
    callback clicked;
}
```

### SegmentButton

[Source](../ui/primitives/segment_button.slint)

```slint
export component SegmentButton inherits Rectangle {
    in property <string> text;
    in property <string> accessible_name;
    in property <bool> selected: false;
    in property <bool> enabled: true;
    out property <bool> has-focus: command.has-focus;
    out property <bool> pressed: command.pressed;
    callback clicked;
}
```

### FluentTextField

[Source](../ui/primitives/text_field.slint)

```slint
export component FluentTextField inherits Rectangle {
    in-out property <string> text;
    in property <string> placeholder_text;
    in property <bool> enabled: true;
    in property <string> error_text;
    callback accepted(string);
    callback edited(string);
}
```

### FluentTextArea

[Source](../ui/primitives/text_area.slint)

```slint
export component FluentTextArea inherits Rectangle {
    in-out property <string> text;
    in property <string> placeholder_text;
    in property <bool> enabled: true;
    callback edited(string);
}
```

### SettingRow

[Source](../ui/patterns/settings/setting_row.slint)

```slint
export component SettingRow inherits Rectangle {
    in property <string> title;
    in property <string> description;
    in property <bool> enabled: true;
}
```

### Navigation types

[Source](../ui/patterns/navigation/navigation_types.slint)

```slint
export enum NavigationPaneMode { expanded, compact }
export enum NavigationContentSurfaceMode { fluent, flat, transparent }
export enum NavigationEntryKind { destination, group, destination_group, separator, header }

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

These declarations establish the Phase 2 model contract; Phase 1 does not render
or validate a tree. Supply complete depth-first preorder trees with depth 0–2,
parent links, nonempty unique IDs across primary/footer models, and explicit
`enabled`/`expanded` values. Struct fields have no assumed `enabled = true` default.
Selection and expansion belong to the host; these types create no route/history API.

### NavigationBackButton

[Source](../ui/patterns/navigation/navigation_back_button.slint)

```slint
export component NavigationBackButton inherits Rectangle {
    in property <bool> enabled: true;
    in property <string> accessible_name: "Back";
    callback clicked();
}
```

A 40 px button reusing IconButton input, with a popup tooltip and focus forwarded
to the inner button on activation. Its geometric
arrow is drawn in Slint source, with no new static asset or font glyph. The host
handles `clicked`; there is no back stack. `accessible_name` also supplies the tooltip.

### NavigationPaneToggleButton

[Source](../ui/patterns/navigation/navigation_pane_toggle_button.slint)

```slint
export component NavigationPaneToggleButton inherits Rectangle {
    in property <bool> enabled: true;
    in property <NavigationPaneMode> pane_mode: NavigationPaneMode.expanded;
    in property <string> accessible_name: "Toggle navigation pane";
    callback clicked();
}
```

The 40 px menu button uses the 20 px navigation icon, forwards focus on activation,
reports the supplied expanded state and emits `clicked`
without changing it. Its name is also its tooltip. Both navigation buttons gate
callback delivery when disabled. Hosts remove hidden controls with conditional
composition, including the enclosing row if empty; setting only `visible: false`
on a fixed-size container does not establish zero-space layout.

### NavigationContentSurface

[Source](../ui/patterns/navigation/navigation_content_surface.slint)

```slint
export component NavigationContentSurface inherits Rectangle {
    in property <NavigationContentSurfaceMode> mode: NavigationContentSurfaceMode.fluent;
}
```

`fluent` paints the content background, 8 px top-left radius and 1 px top/left
border. Other corners are square; an open Path draws only the top/left edges
and their connecting arc. `flat` paints only the content background. `transparent`
shows the host background. The clipped `@children` viewport retains the host's
logical width/height; the component adds no page padding, scrolling or routing.
Pages should remain transparent so they do not cover the frame.

### WindowControlButton

[Source](../ui/patterns/window/window_control_button.slint)

```slint
export component WindowControlButton inherits Rectangle {
    in property <image> icon;
    in property <string> label;
    in property <bool> close_button: false;
    in property <bool> enabled: true;
    callback clicked;
}
```

### SectionHeader

[Source](../ui/patterns/page/section_header.slint)

```slint
export component SectionHeader inherits Rectangle {
    in property <string> title;
    in property <string> description;
    in property <string> badge_text;
    in property <BadgeKind> badge_kind: BadgeKind.neutral;
}
```

### PageHeader

[Source](../ui/patterns/page/page_header.slint)

```slint
export component PageHeader inherits Rectangle {
    in property <string> title;
    in property <string> subtitle;
    in property <bool> show_action: false;
    in property <string> action_text;
    callback action_clicked;
}
```

### EmptyState

[Source](../ui/patterns/page/empty_state.slint)

```slint
export component EmptyState inherits SurfaceCard {
    in property <string> title;
    in property <string> message;
    in property <image> icon: Icons.status_info;
}
```

### MetricCard

[Source](../ui/patterns/page/metric_card.slint)

```slint
export component MetricCard inherits SurfaceCard {
    in property <string> label;
    in property <string> value: "—";
    in property <string> hint;
}
```

### ToastHost

[Source](../ui/overlays/toast.slint)

```slint
export component ToastHost inherits Rectangle {
    in property <bool> shown: false;
    in property <string> message;
    in property <ToastKind> kind: ToastKind.info;
    in property <bool> auto_dismiss: true;
    callback dismissed;
}
```

### ToastKind

[Source](../ui/overlays/toast.slint)

```slint
export enum ToastKind { info, success, error }
```

### ModalKind

[Source](../ui/overlays/modal.slint)

```slint
export enum ModalKind { info, warning, danger }
```

### ModalManager

[Source](../ui/overlays/modal.slint)

```slint
export component ModalManager inherits Rectangle {
    in property <bool> shown: false;
    in property <string> title;
    in property <string> message;
    in property <string> primary_text: "OK";
    in property <string> secondary_text: "Cancel";
    in property <bool> show_secondary: false;
    in property <ModalKind> kind: ModalKind.info;
    in property <bool> danger_primary: false;
    callback accepted;
    callback dismissed;
}
```

## NavigationView

```slint
export component NavigationView inherits Rectangle {
    in property <[NavigationEntry]> items: [];
    in property <[NavigationEntry]> footer_items: [];
    in property <string> selected_id: "";
    in property <NavigationPaneMode> pane_mode: NavigationPaneMode.expanded;
    in property <length> pane_width: UiConstants.navigation_pane_default_width;
    in property <string> pane_title: "";
    in property <bool> show_back_button: false;
    in property <bool> back_enabled: false;
    in property <bool> show_pane_toggle: true;
    in property <bool> show_search: false;
    in-out property <string> search_text: "";
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

}
```

Models are complete depth-first preorder trees with unique nonempty IDs across both
regions, depth 0–2, matching parent links and child flags. Each model supports up
to 256 entries. A malformed or oversized model rejects both menus, with no row
selection or callbacks; optional controls and host content remain available.
Selection and expansion never mutate locally. Group labels request expansion;
destination-group labels invoke and their separate chevrons request expansion.
Headers/separators cannot select. Empty selected icons fall back to regular icons;
compact iconless entries use the generic About icon and ancestor-qualified labels.

Optional controls are conditionally mounted. Compact mode hides pane title and
replaces the search field with a labeled expansion-request button. After the host
accepts expansion, focus transfers to the new editor. Edits update
search_text and emit search_changed; Return submits once, while host assignments
do not echo. Primary and footer menus scroll independently; content scrolling is
host-owned. Pane geometry clamps to available width.

Private implementation: [navigation_view.slint](../ui/patterns/navigation/navigation_view.slint).
Tab/Shift+Tab traverse labels and independent chevrons; Enter/Space activate them.
Right requests expansion. Left requests collapse or focuses an enabled visible
ancestor. These direction keys never invoke a destination. Disabling or hiding
a focused row recovers to a visible enabled ancestor or the pane focus scope.
Back and pane-toggle buttons suppress commands and visible native focus when disabled; native logical focus may remain. Compact tooltips expose
ancestor-qualified labels outside the pane clip.

Native coverage and limits: [Phase 7 report](NAVIGATION_REBUILD_PHASE7.md).

## Current-version change protocol (P1)

For any component/type/member change, explain the concrete benefit and scope, then
update its sole implementation, facade when names change, internal callers,
Gallery/catalog, this document and the existing API probe. Review the candidate
signature/default diff (including direction, type, base, callback order and state
ownership), deliberately adopt it only after reconciliation, and document Breaking
changes and current use. Remove replaced code; no old/new consumer matrix or
no-op compatibility properties. P1 adopts no new snapshot and changes no public API.

Define inputs/defaults, outputs/action timing, programmatic updates, empty/invalid
models, focus/keyboard/disabled/read-only, sizing/long text and slot ownership before
implementing each new contract. SettingRow's slot controls explicitly bind enabled;
NavigationView/SegmentButton retain host-controlled state. API probe focus methods,
slot children, narrow editor and host setters compile without claiming runtime
behavior. Native wrapper decisions and public type limits are in NATIVE_REUSE.md.


## P3 current contracts and Breaking changes

- IconButton and SegmentButton remove all three preview inputs and add
  `accessible_name`, read-only `has-focus` and `pressed`. Bind the name on the
  wrapper, rather than inherited `accessible-label` on its non-interactive root.
  IconButton falls back to tooltip; SegmentButton falls back to text. Empty icon
  commands require an explicit name. Each has one visible native Button owner;
  Return/Space or a completed pointer activation forwards one enabled command.
- SegmentButton keeps `selected` as a host-controlled input. The native child uses
  `checked: root.selected`, `checkable: false`, and accessible checkable/checked
  bindings. Native activation cannot toggle the checked value. Programmatic
  selection emits no clicked; the host can accept, reject or defer a request.
  Selection now uses native checked fill. This is a button, not a RadioGroup.
- IconButton uses native 20px icon tint and content sizing: the default is 44×32px
  for a populated icon in pinned Fluent, and 32×32px for no icon. Explicitly forcing
  less than native min-width can displace/clip icon content. Native Tooltips own
  popup timing/position; TooltipHost remains passive content. Danger is the P2
  neutral native surface plus passive outline, not a red-filled template.
- WindowControlButton removes unused `symbol`, adds `enabled: true`, and defaults
  to 46×40px instead of inheriting arbitrary parent height. Supply an image and
  descriptive label. It composes IconButton; `close_button` selects the passive
  danger outline. It never performs an OS action. Actual Gallery chrome stays native.
- FluentTextField removes preview_focus. The native editor is top-aligned; the
  wrapping error caption increases preferred/minimum height. Parent layouts should
  honor that height. Text remains two-way, edited means user editing and accepted
  means native single-line Return. FluentTextArea retains native multiline editing,
  wrapping and its 76px minimum. Disabled is not read-only; no read-only API is added.
- SurfaceCard removes preview inputs and whole-subtree opacity. Non-interactive
  cards omit the pointer and accessibility helper subtrees; a disabled focus anchor
  remains for inherited focus forwarding. Interactive arbitrary-child cards retain
  a scoped custom action exception because public Button has no content slot.
  Use passive children in an interactive card; embedded independent controls use a
  non-interactive card. `enabled` gates the card action, not arbitrary child input;
  the host controls child enabled and text colors. Default elevation remains off.
- SettingRow.enabled styles its own title/description through Theme.text_disabled;
  the slot control must bind enabled to the same host value. The row does not
  recursively disable children or dim an already-disabled native control. Long
  labels wrap and preferred height follows layout; callers must honor content size.
- PageHeader places its optional native action below text below 420px width.
  Titles/subtitles wrap. SectionHeader, MetricCard and EmptyState wrap their long
  labels; EmptyState removes unused `milestone`. Badge and FluentIcon keep their
  already-small presentation structures; icon optical offsets still need the wrapper.

All command state is native-owned except documented host-controlled selection.
Native logical focus may survive disable/re-enable; commands and disabled visuals
are suppressed, and re-enabling emits no action. Forced undersized native text
buttons still need sufficient width or abbreviated visible text and a full name.
See [P3 evidence and limits](implementation/kit-fluent-v1/P3.md), including the
unrun full IME, reader, modal lifecycle and WinUI reference comparison.

## P4A native selection contracts

These wrappers use the pinned public native API without extra input objects or
shadow state. `checked` on CheckBox/Switch is native in-out bool; `toggled()` is a
user action, not a programmatic assignment event. Share state with `<=>`. No three-state
checkbox is offered. ComboBox inherits model, enabled, current-index/current-value,
has-focus and selected(string); model replacement is normalized by native code.
FluentRadioGroup is a static public native re-export: an extra subclass loses the
compiler's RadioButton lowering. Its snapshot records the inherited native contract,
not a second implementation. It accepts native RadioButton children with text/checked/enabled and exposes
title/enabled/orientation/current-value/has-focus/selected(string). It has no public
current-index property. Native RadioGroup selected(string) reports selection
transitions, including programmatic checked=true and initial selection; it is not
a user-only command. Setting the selected child false reverts; deselect-all is not
supported. Static children are verified. Dynamic RadioButton repeaters currently
produce invalid generated Rust in 1.17.1 and are NOT_SUPPORTED in this contract.
Tab visits enabled radio children; Space/Enter selects. Arrow-key selection is not
implemented by this pinned native control and is not added by Kit.
Native keyboard, focus, accessibility and sizing pass through the public base.
These are current static exports; no registration or compatibility path is added.

```slint
export component FluentCheckBox inherits CheckBox { }
export component FluentSwitch inherits Switch { }
export { RadioGroup as FluentRadioGroup } from "std-widgets.slint";
export component FluentComboBox inherits ComboBox { }
```

## P4B numeric and progress contracts

FluentSlider inherits native float value/minimum/maximum/step, orientation, enabled,
has-focus and changed(float)/released(float). FluentSpinBox inherits native int
value/minimum/maximum/step-size, read-only, enabled, horizontal-alignment, has-focus
and edited(int). Host assignments do not emit edit/change callbacks; hosts must
supply ordered ranges and in-range programmatic values. Native input applies its
own boundary rules. Slider has no read-only property. This is not a floating or
expression NumberBox. No native drag/edit implementation is copied.

Progress wrappers clamp presentation to 0..1. running=false makes indeterminate
progress static; visible=false removes the native child. animating describes the
requested native indeterminate state, not measured CPU. Hosts keeping an entire
ancestor hidden/resident must bind running to their active state or unload it.
There is no Kit Timer or duplicate animation; Spinner keeps native motion.

```slint
export component FluentSlider inherits Slider { }

export component FluentSpinBox inherits SpinBox { }

export component FluentProgressBar inherits Rectangle {
    in property <float> progress: 0;
    in property <bool> indeterminate: false;
    in property <bool> running: true;
    out property <bool> animating: root.visible && root.running && root.indeterminate;
    height: 3px;
    preferred-height: 3px;
    horizontal-stretch: 1;
    vertical-stretch: 0;
    if root.visible: ProgressIndicator {
        width: 100%; height: 100%;
        progress: min(1, max(0, root.progress));
        indeterminate: root.animating;
    }
}

export component FluentProgressRing inherits Rectangle {
    in property <float> progress: 0;
    in property <bool> indeterminate: false;
    in property <bool> running: true;
    out property <bool> animating: root.visible && root.running && root.indeterminate;
    width: 32px;
    height: 32px;
    preferred-width: 32px;
    preferred-height: 32px;
    horizontal-stretch: 0;
    vertical-stretch: 0;
    if root.visible: Spinner {
        width: 100%; height: 100%;
        progress: min(1, max(0, root.progress));
        indeterminate: root.animating;
    }
}

```

## P4C container contracts

ListView and TabWidget preserve the exact public native identity through verified
static re-exports. ListView requires a single direct for child; TabWidget requires
fixed Tab children (title only). No arbitrary slot wrapper obscures virtualization.
TabWidget retains native in-out current-index/orientation; no close, reorder or
multi-window TabView API. Native inactive tab content may remain instantiated:
hosts coordinate expensive content lifecycle and timers.

ScrollView inherits enabled (scrollbars), visible-width/height, viewport geometry,
scrollbar policies, mouse-drag-pan-enabled and scrolled. enabled is not a recursive
content disable switch. GroupBox similarly has title/enabled/content-padding and
its native child slot; hosts bind each interactive child to the same enabled source.

StandardListView's native enabled only governs scrollbars, so the Kit composition
names it scrollbars-enabled. There is deliberately no misleading whole-control
enabled property. Rows still support native selection. Native current-item storage
is two-way shared, with current-item-changed and item-pointer-event forwarded once.
Model replacement and selection reconciliation remain host responsibilities;
programmatic current-item assignment is not a user selection event. Builtin
StandardListViewItem, Point and PointerEvent are language types, not Kit aliases.

```slint
export component FluentScrollView inherits ScrollView { }
export { ListView as FluentListView } from "std-widgets.slint";
export component FluentStandardListView inherits Rectangle {
    in property <[StandardListViewItem]> model <=> view.model;
    in-out property <int> current-item <=> view.current-item;
    in property <bool> scrollbars-enabled <=> view.enabled;
    callback current-item-changed(int);
    callback item-pointer-event(int, PointerEvent, Point);
    min-width: 50px;
    min-height: 50px;
    preferred-width: 100%;
    preferred-height: 100%;
    horizontal-stretch: 1;
    vertical-stretch: 1;
    forward-focus: view;
    view := StandardListView {
        width: 100%; height: 100%;
        current-item-changed(index) => { root.current-item-changed(index); }
        item-pointer-event(index, event, position) => { root.item-pointer-event(index, event, position); }
    }
}
export component FluentGroupBox inherits GroupBox { }
export { TabWidget as FluentTabWidget } from "std-widgets.slint";
```

## P4D table and date/time contracts

StandardTableView keeps builtin TableColumn/StandardListViewItem data. columns and
current-row share native storage; sorting callbacks request host work and do not
sort rows. scrollbars-enabled only controls native scrollbars, not rows or headers.
No editing, frozen columns or generalized DataGrid behavior is promised. Hosts
reconcile selection when replacing models and keep cell/column shapes consistent.

Date and Time are direct public native structs. Pickers compose visible native
Button and public native popup. Host input date/time is the sole committed value;
accepted(value) requests a host update, canceled() leaves it unchanged. open() is
gated by enabled/visible; close() is imperative and emits neither callback. Native
OK/Cancel closes before forwarding once and returning focus to the opener. Setting the picker's own visible=false
or enabled=false closes the popup. Before hiding a retained ancestor, hosts close
the popup explicitly or unload the picker. Hosts supply valid calendar dates/times; no Kit
calendar, locale parser, range, clock, timezone or business sorting is added.
The default text is a simple numeric label and can be overridden for localization.
Native popup editing/validation and positioning limits remain the pinned contract.
There is no synthetic shown mirror or promise of a full WinUI picker.

```slint
export struct Date { day: int, month: int, year: int }
export struct Time { hour: int, minute: int, second: int }
export component FluentStandardTableView inherits Rectangle {
    in property <[[StandardListViewItem]]> rows <=> view.rows;
    in-out property <[TableColumn]> columns <=> view.columns;
    in-out property <int> current-row <=> view.current-row;
    out property <int> current-sort-column: view.current-sort-column;
    in property <bool> scrollbars-enabled <=> view.enabled;
    out property <bool> has-focus: view.has-focus;
    callback sort-ascending(int);
    callback sort-descending(int);
    callback current-row-changed(int);
    callback row-pointer-event(int, PointerEvent, Point);
    min-width: 400px; min-height: 200px;
    horizontal-stretch: 1; vertical-stretch: 1;
    forward-focus: view;
    view := StandardTableView {
        width: 100%; height: 100%;
        sort-ascending(index) => { root.sort-ascending(index); }
        sort-descending(index) => { root.sort-descending(index); }
        current-row-changed(index) => { root.current-row-changed(index); }
        row-pointer-event(index, event, position) => { root.row-pointer-event(index, event, position); }
    }
}
export component FluentDatePicker inherits Rectangle {
    in property <Date> date: { year: 2026, month: 1, day: 1 };
    in property <string> text: root.date.year + "-" + root.date.month + "-" + root.date.day;
    in property <string> title: "Select date";
    in property <bool> enabled: true;
    out property <bool> has-focus: command.has-focus;
    callback accepted(Date);
    callback canceled();
    public function open() { if root.enabled && root.visible { popup.show(); } }
    public function close() { popup.close(); if root.enabled && root.visible { command.focus(); } }
    changed enabled => { if !root.enabled { popup.close(); } }
    changed visible => { if !root.visible { popup.close(); } }
    preferred-width: command.min-width; preferred-height: command.min-height;
    width: command.min-width; height: command.min-height;
    horizontal-stretch: 0; vertical-stretch: 0;
    forward-focus: command;
    command := Button {
        width: 100%; height: 100%; text: root.text; enabled: root.enabled;
        clicked => { root.open(); }
    }
    popup := DatePickerPopup {
        x: 0px; y: root.height;
        title: root.title; date: root.date;
        accepted(value) => { command.focus(); root.accepted(value); }
        canceled => { command.focus(); root.canceled(); }
    }
}
export component FluentTimePicker inherits Rectangle {
    in property <Time> time: { hour: 0, minute: 0, second: 0 };
    in property <string> text: root.time.hour + ":" + (root.time.minute < 10 ? "0" : "") + root.time.minute;
    in property <string> title: "Select time";
    in property <bool> enabled: true;
    out property <bool> has-focus: command.has-focus;
    in property <bool> use-24-hour-format: true;
    callback accepted(Time);
    callback canceled();
    public function open() { if root.enabled && root.visible { popup.show(); } }
    public function close() { popup.close(); if root.enabled && root.visible { command.focus(); } }
    changed enabled => { if !root.enabled { popup.close(); } }
    changed visible => { if !root.visible { popup.close(); } }
    preferred-width: command.min-width; preferred-height: command.min-height;
    width: command.min-width; height: command.min-height;
    horizontal-stretch: 0; vertical-stretch: 0;
    forward-focus: command;
    command := Button {
        width: 100%; height: 100%; text: root.text; enabled: root.enabled;
        clicked => { root.open(); }
    }
    popup := TimePickerPopup {
        x: 0px; y: root.height;
        title: root.title; time: root.time;
        use-24-hour-format: root.use-24-hour-format;
        accepted(value) => { command.focus(); root.accepted(value); }
        canceled => { command.focus(); root.canceled(); }
    }
}
```

## P5A transient lifecycle

ToastHost shown is host-owned. Each false-to-true shown cycle permits at most one
dismissed request, shared by native close input and the automatic timer. Hosts set
shown=false in dismissed. If the host ignores it, the timer stops and close input
is disabled until a new cycle. Programmatic hide/unload emits no dismissed event.
Changing message/kind within a cycle does not restart it. Reopening should span a
real state update; false/true assignments coalesced within one event are not a new
observed cycle. The fixed 56px message area clips/elides beyond its two-line budget.

Auto dismissal waits four seconds. Passive hover pauses it; leaving starts a fresh
four-second interval. shown=false or own visible=false destroys the native close
button/tooltip and timer subtree. Retained ancestor pages must set shown=false or
unload the toast. No focus acquisition and no Kit animation in this P5A behavior
baseline; a user may focus the native close command normally.

TooltipHost is a passive presenter, not a hover service. Put it in public native
Tooltip, as IconButton does. Native Tooltip owns delay, pointer positioning, clipping
escape and dismissal. Keyboard help and screen-reader announcement are not promised
by the passive presenter or inferred from hover screenshots. No Kit tooltip timer.

Pinned native Tooltip placement can extend beyond the containing window; an
embedded software-window snapshot may crop edge content. Kit keeps this native
placement boundary; TooltipHost does not promise work-area clamping or keyboard
activation. Prefer short supplementary text and keep essential information inline.
