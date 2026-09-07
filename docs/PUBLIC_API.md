# Public API — local NavigationView / Gallery rebuild

`ui/kit.slint` is the only supported Slint entry. All 35 public names below participate in `gallery/ui/api_probe.slint`, compiled through Gallery. Root Rust API is limited to `SLINT_LIBRARY_NAME: &str` and `slint_library_path() -> PathBuf`; generated Slint runtime types belong to the consumer. The navigation rebuild additions and SidebarItem removal are local, unpublished work; the retained extraction source in CONSUMER_GUIDE.md still exposes its original API.

Signatures below are copied from the current sources, including declared defaults. They describe explicitly declared API; inherited Slint element properties still apply. Internal filenames are links for reading implementation, not additional supported import entry points. `scripts/kit_api_v1.json` freezes all 35 exported names, 240 properties, 20 callbacks, seven enum value lists and one ten-field struct. The guard reports signature and default-expression differences separately; see [validation](VALIDATION.md) for its scope and explicit update process.

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
    in property <bool> preview_hover: false;
    in property <bool> preview_pressed: false;
    in property <bool> preview_focus: false;
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
    in property <bool> show_icon: false;
    in property <bool> primary: false;
    in property <bool> accent: false;
    in property <bool> danger: false;
    in property <bool> enabled: true;
    in property <bool> preview_hover: false;
    in property <bool> preview_pressed: false;
    in property <bool> preview_focus: false;
    callback clicked;
}
```

### IconButton

[Source](../ui/primitives/icon_button.slint)

```slint
export component IconButton inherits Rectangle {
    in property <image> icon;
    in property <string> tooltip;
    in property <bool> danger: false;
    in property <bool> enabled: true;
    in property <bool> preview_hover: false;
    in property <bool> preview_pressed: false;
    in property <bool> preview_focus: false;
    callback clicked;
}
```

### SegmentButton

[Source](../ui/primitives/segment_button.slint)

```slint
export component SegmentButton inherits Rectangle {
    in property <string> text;
    in property <bool> selected: false;
    in property <bool> enabled: true;
    in property <bool> preview_hover: false;
    in property <bool> preview_pressed: false;
    in property <bool> preview_focus: false;
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
    in property <bool> preview_focus: false;
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
    in property <string> symbol;
    in property <string> label;
    in property <bool> close_button: false;
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
    in property <string> milestone;
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
Back and pane-toggle buttons clear focus when disabled. Compact tooltips expose
ancestor-qualified labels outside the pane clip.

Native coverage and limits: [Phase 7 report](NAVIGATION_REBUILD_PHASE7.md).
