# P0 current-source inventory

Audited 2026-09-09 at `737d0aae0975232f520cc1e82640d99e8eb49467`.
The initial working tree deleted the previous root Navigation rebuild SPEC and
contained an untracked Fluent evolution SPEC. Neither user change was overwritten.
HEAD equals the SPEC audit revision: no committed differences in facade, production
components, Cargo versions, directories, chrome adapters, or API snapshot.
Only the root AGENTS.md exists; there are no nested UI/Gallery AGENTS files yet.
Earlier Navigation Phase 0–8 reports describe historical checks, not this P0 run.

## Public surface and resources

The current facade has 35 names: 21 visual components, six globals, seven enums,
one struct; the reviewed snapshot declares 240 properties and 20 callbacks.
The existing API probe and PUBLIC_API.md are the current usage examples. P0 does
not copy them into historical compatibility fixtures or change the snapshot.

| Kind | Names and responsibility |
|---|---|
| Globals | Theme (host mode/system/font and derived colors); Typography (12/14/18/20/28px, weights); Motion (100/160/200ms); Elevation (surface/shadow recipes); UiConstants (spacing/control/navigation dimensions); FluentIcons (static SVG aliases) |
| Enums | ThemeMode; BadgeKind; ToastKind; ModalKind; NavigationPaneMode; NavigationContentSurfaceMode; NavigationEntryKind |
| Struct | NavigationEntry: id, parent_id, depth, kind, text, icon, selected_icon, enabled, expanded, has_children |

Each global is scoped to a top-level component instance. Foundation creates no
visual tree, timers or host services. `src/lib.rs` has only source lookup duties;
the root package has no normal/runtime dependencies. Gallery initializes its own
Theme/Palette/font and owns Win32 DWM/AppKit chrome. No platform code moves to Kit.
The 32 SVGs retain their manifest byte hashes and MIT attribution; the static
distribution closure contains 65 files including licensing metadata. No fonts
are bundled. GPL handwritten-source headers and provenance are preserved.

## Every current visual component

Paths below are relative to `ui/`. “Retained” describes source declarations, not
measured heap object counts: Slint may optimize declarations. Inherited and nested
costs must be added; no static node count is presented as runtime allocation proof.
Unless explicitly stated, the component declares no Timer, animation, shadow or
clip. An implicit child slot is distinguished from an explicit `@children` slot.

| Component / implementation | Native basis and current custom behavior | Lifetime, animation, shadow/clip and slots | Follow-up |
|---|---|---|---|
| FluentButton / primitives/fluent_button.slint | Rectangle/Text/Image via FluentIcon; own TouchArea, FocusScope, accessible action; no std Button | Retained root/layout/text/focus/touch; icon conditional; preview state inputs | P2 native migration; ordinary/danger/controlled semantics and real input |
| IconButton / primitives/icon_button.slint | Own touch/key/accessibility command; focus-visible workaround; inline TooltipHost | Retained FluentIcon, FocusScope, TouchArea, TooltipHost even without tooltip | P3 native command + builtin Tooltip; remove preview API when migrated |
| SegmentButton / primitives/segment_button.slint | Own touch/key/accessibility command; selected is host input | Retained Text/FocusScope/TouchArea, no animation | Preserve controlled selection decision; no native autonomous second state |
| FluentTextField / primitives/text_field.slint | Real LineEdit owns editing; text two-way; accepted/edited forwarded | Root + editor + retained error Text; error/focus rectangles conditional; error already word-wrap | Test long/narrow errors and IME; do not report wrapping as missing |
| FluentTextArea / primitives/text_area.slint | Real TextEdit, two-way text, edited forwarding, word-wrap | Root + full-size TextEdit retained; native editor internal costs not counted | Native scrolling/IME/read-only contract review |
| FluentIcon / primitives/fluent_icon.slint | Image with brush colorize and optical offsets | Rectangle + Image retained; no input | Static resources only; size/optical review |
| TooltipHost / primitives/tooltip_host.slint | Rectangle + Text presenter; shown controls visible, z=500 | Retained when hidden; fixed 28px height; no Timer | Inline z cannot escape ancestor clipping; builtin service needed |
| SurfaceCard / primitives/surface_card.slint | Custom optional touch/key/action card behavior | Root + accessibility Rectangle + TouchArea + FocusScope retained even interactive=false; shadow only elevated; implicit children | Record child interaction and disabled protocol; measure retained helpers |
| Badge / primitives/badge.slint | Rectangle + Text, semantic colors, no input | Retained; 24px height, 12px radius | Static presentation exception |
| SettingRow / patterns/settings/setting_row.slint | Layout/Text, group accessibility; enabled changes opacity only | Retained layouts/text; explicit trailing children slot; description word-wrap | Host must disable actual slot controls; no transparent input shield |
| NavigationView / patterns/navigation/navigation_view.slint | Real ScrollViews and FluentTextField/LineEdit; custom hierarchy, requests, focus recovery; composed buttons still custom | Primary for inside VerticalLayout; footer conditional; rows retained when collapsed via visible/height; pane clip; content slot; nested row background animation 100ms | Each model max 256, depth 0..2; ID counts/parent searches have quadratic paths; runtime 16/64/256 tests still required |
| NavigationBackButton / patterns/navigation/navigation_back_button.slint | IconButton + Path arrow; callback only, no history; builtin Tooltip | Retained command and arrow; TooltipHost in builtin popup | Gains native behavior through IconButton migration |
| NavigationPaneToggleButton / patterns/navigation/navigation_pane_toggle_button.slint | IconButton + FluentIcon; host-owned pane_mode; builtin Tooltip | Retained; accessible expanded semantics | No routing ownership; native command migration inherited |
| NavigationContentSurface / patterns/navigation/navigation_content_surface.slint | Rectangle/Path structural surface | Background conditional on mode; frame conditional on fluent + size; root clip=true; full viewport explicit children | Modes are transparent/flat/fluent, not a hypothetical none mode |
| WindowControlButton / patterns/window/window_control_button.slint | Custom TouchArea/FocusScope/action + FluentIcon; no OS call | Retained; hover close red; symbol property unused in rendering | Migrate command behavior; Gallery already uses actual native caption controls |
| SectionHeader / patterns/page/section_header.slint | Text/layout + optional Badge | Description and badge conditional; explicit trailing children; no shadow | Long text/DPI; no entrance animation |
| PageHeader / patterns/page/page_header.slint | Text/layout + FluentButton action | Button conditional on show_action; subtitle word-wrap, max-height 40px + elide | Narrow actions/text must be observed; no second button implementation |
| EmptyState / patterns/page/empty_state.slint | Inherits SurfaceCard; icon/title/message composition | Retained layout/icon/text plus inherited card helpers; message word-wrap; milestone currently not rendered | No action currently declared; audit API purpose without inventing a missing callback |
| MetricCard / patterns/page/metric_card.slint | Inherits SurfaceCard; label/value/hint Text | Retained layout and three Texts plus inherited card helpers | No business calculations or numeric animation |
| ToastHost / overlays/toast.slint | Rectangle/layout/icon/Text + custom IconButton; dismiss request | Entire child tree retained hidden; clip=true; popup shadow; height 160ms and opacity 100ms; one Timer running only shown && auto_dismiss && !hover, interval 4000ms | Measure hidden cost; repeating timer can repeat request if host leaves shown=true; do not claim hidden timer is active |
| ModalManager / overlays/modal.slint | One confirmation layer; custom FluentButtons; FocusScope captures Return/Escape | FocusScope retained; overlay/card/controls conditional shown; shield TouchArea; dialog shadow; no declared animation/Timer | Capture Return may override secondary action intent; Tab containment/restoration unproven; no full ContentDialog claim |

`NavigationItemRow` is private: two custom command regions (label/chevron), each
with focus/touch/accessibility; conditional text/icon/selection marker; builtin
Tooltip; background animates with Motion.fast. `NavigationModel` is a private
pure-function global with explicit subdivision depth rather than recursive calls.
Gallery pages are conditional `if` instances. Its process cost must not be used
as the per-component cost.

## P0 corrections to the planning baseline

- Error text already wraps; inspect actual sizing rather than adding duplicate wrapping.
- Navigation rows already animate background; Kit is not animation-free.
- EmptyState has no action callback and its milestone input is unused; WindowControlButton.symbol is also unused. These are facts for later API review, not P0 removals.
- SurfaceCard creates input helpers while static; static declaration presence does not establish a leak.
- Public RadioGroup differs from its internal base, and ListView forwarding has compiler constraints; see NATIVE_REUSE.md for actual probe results.
- No production defect is marked fixed in P0. All later migration/visual/performance obligations remain separate.
