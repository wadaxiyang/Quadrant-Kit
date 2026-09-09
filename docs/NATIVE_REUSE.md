# Native reuse — current P3 policy and pinned capability evidence

Applies to Slint/slint-build **1.17.1**, Fluent style, current Kit HEAD
the pushed P0–P2 source `f479338` plus the current P3 changes
identified in the phase reports.
This is a static audit, not a runtime compliance declaration. The P0 inventory is
[AUDIT.md](implementation/kit-fluent-v1/AUDIT.md). P1 adds the current manifest and
guard described below. P2 migrates FluentButton; P3 migrates IconButton, SegmentButton and window commands.

## Current manifest and guard

`scripts/native_reuse_manifest.json` records every current visual implementation,
including private NavigationItemRow. It is development/CI metadata and is not
loaded at runtime or added to Cargo's distribution include. Facade remains the
public export authority. Component status and Gallery routes are reconciled in
COMPONENT_STATUS.md; public/private mismatches, stale/missing/duplicate records fail.

Run `python scripts/check_native_reuse.py`. The existing boundary command also
invokes it, so existing CI paths enforce it. Its fixtures run in Python discovery.
It reuses slint_contract.parse/lex/balanced bodies, resolves imported aliases and
component composition, and checks actual instance/base references rather than
counting imports or keywords inside comments/strings. Expected native dependencies
must match the computed closure. Native wrappers require one direct imported std
owner and no transitive duplicate TouchArea/FocusScope/TextInput/Flickable or
keyboard/accessibility activation handlers. Literal hidden proxies fail.

Statuses are `native-wrapper` (currently Button/LineEdit/TextEdit wrappers),
`presenter` (passive), `composed` (children own behavior), `reviewed-exception`
(narrow custom behavior), and `custom/pending-migration` (existing ordinary input
debt, never compliant). Component composition may contain pending children; this
does not make the compound component native-compliant. Four exceptions record
SurfaceCard, NavigationView, ToastHost and ModalManager with explicit scope.

P5C removes the last pending-command allowance. New pending commands fail.
Reviewed exceptions and pending implementations have canonical declaration/body
digests; comments/formatting do not change them, new behavior does. There is no
automatic refresh command. Review changes with concrete scope/behavior evidence,
then update the current record explicitly. When a pending component migrates,
remove its allowance from PENDING_COMPONENTS as well as its digest/obsolete
exception fields; this makes subsequent downgrade to pending fail. Do not retain
old component implementations or historic compatibility paths in the manifest.

This is a conservative lexical policy, not dataflow analysis: it cannot prove a
dynamic visibility expression, actual hit-testing, runtime single-action behavior,
focus/a11y or virtualization. It does not establish whether a claimed new exception
is justified; that requires review of the concrete missing native capability.
Do not describe a PASS as full compliance. P2 adds separate real-input verification; the static guard still makes no runtime claim. Unsupported syntax still fails explicitly.

Current scanner handling of upstream exports is intentionally narrow: verified
Slint 1.17.1 Date/Time structs can be resolved through explicit or aliased public
re-exports. TableColumn and StandardListViewItem are builtin property/model type
references; they are not guessed std exports. Unknown upstream re-exports (including
direct component re-exports) fail until deliberately supported with fixtures and
compiler evidence. No native type was added to Kit's facade/API in P1.

The compiler used is the resolved `i-slint-compiler-1.17.1` Cargo source. Audited
files are `widgets/fluent/std-widgets.slint`, `button.slint`, `style-base.slint`,
`radiogroup.slint`, `datepicker.slint`, `time-picker.slint`, `tableview.slint`,
`widgets/common/listview.slint`, date/time/radio bases and `builtins.slint`.
Internal source is evidence only: probes import solely public `std-widgets.slint`.
The public builtin RadioGroup declaration takes precedence over an internal base.

## Executable capability checks

`gallery/ui/native_control_probe.slint` is compiled before Gallery, with the same
locked compiler/style. It is not instantiated by Gallery and adds no Gallery
runtime widget. Independent positive and expected-negative checks run with:

```console
python scripts/run_perf.py --native-probe
```

Generated cases/logs and exit codes remain under `target/perf-harness/<run>/`.
Positive compilation proves type/property/callback/method availability, not actual
keyboard, focus restoration, popup placement, accessibility or list virtualization.
Negative cases must fail with their expected diagnostic; a random build error
does not count as successfully establishing a capability limit.

| Public surface | Verified legal shape / defaults from locked source | Limit and next decision |
|---|---|---|
| Button | in text:string, icon:image, icon-size:length=20px, primary/checkable/colorize-icon:bool=false; enabled binds TouchArea default; in-out checked=false; out has-focus/pressed; clicked(); focus() forwarding | Background/text-color/hover are not public styling inputs. Internals animate at 150ms and own key/touch/accessibility actions. Do not duplicate. No per-instance danger template. Runtime input NOT_RUN in P0. |
| Palette | in-out color-scheme:ColorScheme; out brush background/foreground/control/accent/selection/border variants; probe assigns light and reads accent as brush | Writing accent-background fails with output-property diagnostic. Do not assume `.color`, private FluentPalette or per-button theme writes. |
| RadioGroup | in title:string, enabled:bool=true, orientation:Orientation; out current-value:string and has-focus; selected(string); contextual RadioButton children have text/enabled/checked/toggled | **No public current-index.** Internal RadioGroupBase.current-index is not the public contract. Bind child checked; independent exported RadioButton is not assumed. |
| ListView | Inherits ScrollView; direct single for child compiles | **Ordinary wrapper `ListView { @children }` fails**: only a single for is accepted. Current probe preserves direct for. Alternative inheritance/wrapper structure, virtualization and large-list allocation are NOT_RUN; P4C must resolve before publishing FluentListView. |
| DatePickerPopup / Date | Date has int year/month/day; in title="Select date", date bound to base (today default); accepted(Date), canceled(); show()/close() compile | Uses builtin PopupWindow, no-auto-close; no public range/format template inferred from internal calendar. Date validity, focus, actual accept/cancel NOT_RUN. |
| TimePickerPopup / Time | Time has int hour/minute/second; in title="Select time", time (base hour=12), use-24-hour-format; accepted(Time), canceled(); show()/close() compile | Timezone conversion and business formatting excluded; runtime boundaries NOT_RUN. |
| StandardTableView | in rows:[[StandardListViewItem]]; in-out columns:[TableColumn], current-row:int=-1; out current-sort-column=-1; sort-ascending/descending(int), current-row-changed(int), row-pointer-event; viewport bindings | Model literals with TableColumn.title/width and StandardListViewItem.text compile. Types are builtins, no invented std import. Not editable/frozen DataGrid; behavior/large models NOT_RUN. |
| LineEdit / TextEdit | Current Kit wrappers compile with text two-way, enabled, font-size, placeholder and edited; LineEdit accepted; TextEdit word-wrap | Native editing retained. Real IME, clipboard, selection and a11y require independent runtime checks. |

The additional std exports CheckBox, Switch, ComboBox, Slider, SpinBox,
ProgressIndicator, Spinner, ScrollView, StandardListView, GroupBox and TabWidget
are present in the locked facade. Their exhaustive property/default/behavior
matrix is NOT_RUN in P0, to be verified per P4 batch; SpinBox's integer scope and
TabWidget's contextual children must not be replaced by guessed WinUI contracts.
Tooltip/PopupWindow are public builtins; existing navigation compiles Tooltip.
P5D exercises native PopupWindow and Menu/ContextMenuArea; its finite input/focus contracts are documented below.

## Current exceptions and migration state

NavigationItemRow now uses visible native label/arrow Buttons (P5C). IconButton and
SegmentButton are native Button wrappers; WindowControlButton composes IconButton.
PageHeader and ModalManager now use native Button through FluentButton. Text wrappers already use actual native editors. Badge, FluentIcon,
TooltipHost content, surfaces, headings, metrics and empty state are presentation
or structure candidates: no complete std equivalent exists. Navigation lacks a
complete std hierarchy/navigation container, Toast lacks an equivalent transient
message presenter, and ModalManager lacks a complete std confirmation overlay.
Those exceptions permit composition-specific state only, never a second ordinary
button/editor/scrollbar implementation. SettingRow has a composition/slot protocol,
not implicit authority to disable arbitrary children by opacity.

## P2 FluentButton decision

FluentButton wraps one visible public Button. It has no TouchArea, FocusScope,
key handler, custom accessible action, cloned native implementation or Kit animation.
Native clicked has one enabled-guarded forwarding path: this also rejects direct
native accessibility requests when disabled. The only decoration is a conditional,
non-interactive danger outline outside the native surface. Danger selects the
neutral native surface, including when primary is true; no shared Palette writes.
Native icons render without show_icon, and accessible_name optionally labels the
native node. Preview/accent inputs and the superseded implementation are removed.
Defaults use native content sizing, including the 32px empty minimum. Native
focus can be retained while disabled, but visuals/actions are gated by enabled.

The FluentButton pending allowance and old body digest are removed. ModalManager
and PageHeader records now include their transitive Button dependency, while their
implementation and existing finite overlay limitations remain. See PUBLIC_API for
Breaking changes and implementation/kit-fluent-v1/P2.md for actual input/visual/perf
results. No completed screen-reader, full modal containment or OS-wide validation
is inferred from those scoped checks.


## P3 decisions

IconButton has one visible Button and builtin Tooltip, with no duplicated keyboard,
pointer or accessibility action. SegmentButton uses a non-toggling native Button
with one-way checked binding and accessibility checked semantics. Window commands
compose IconButton. Back retains a passive geometric arrow over the visible native
surface; pane toggle uses native icon/expanded accessibility on its Button.
Navigation search names now target the IconButton native owner. Gallery disclosure
commands use std Button directly, keeping expandable semantics on the input node.

SurfaceCard retains its explicitly reviewed optional content-slot action exception.
Only interactive cards instantiate pointer/a11y helpers; one disabled focus anchor
remains. P3 does not claim zero nodes for a passive card. TooltipHost remains a
presenter; comprehensive Tooltip/Toast lifecycle work is still P5A. Navigation row
input and model work is P5C; P3 does not disguise it as completed native reuse.

Theme's public state is host-controlled. Host sets Theme mode/system state and
Palette.color-scheme from the same decision, once per top-level component. Palette
color outputs are never assigned. Kit-owned surfaces keep semantic Theme colors;
native controls keep their Fluent palette and animation. Typography, spacing and
Elevation recipes remain centralized and unchanged. Theme.text_disabled is the
new shared disabled foreground for custom labels; it is not an upstream palette
setter. Detailed differences and evidence are in P3.md and DESIGN_SYSTEM.md.

P4A adds FluentCheckBox, FluentSwitch, FluentRadioGroup and FluentComboBox as direct
public std controls. RadioGroup retains native compiler identity through a static
re-export because subclassed children fail Rust code generation. The scanner allows
only the verified RadioGroup native component re-export and checks its true owner;
unknown native exports still fail. Native state/input/child grammar stay native-owned;
there is no extra wrapper tree, activation forwarding or private Slint import.

P4B adds direct Slider/SpinBox subclasses and conditional public ProgressIndicator/Spinner
children. running/visible control native indeterminate work; Kit adds no Timer or
drag/edit handler. Native numeric bounds and int/read-only semantics remain native.

P4C preserves ListView/TabWidget native identity through verified static exports;
ScrollView/GroupBox inherit native slots. StandardListView composes one native view
and names its limited enabled behavior scrollbars-enabled. No copied row/scroll/tab
input and no claim that native container enabled recursively disables content.

P4D composes native table and date/time popups. No business sorting/calendar logic.
Table scrollbars-enabled accurately names the native limited disable scope.

P5A reviews ToastHost's single-cycle dismissal guard, conditional native child/Timer
lifetime and passive hover pause. Ordinary close activation remains the visible
native IconButton. Native Tooltip remains the only tooltip service; Gallery now
shows it explicitly alongside the separately labeled passive presenter.

P5B removes Modal Return interception and ordinary activation duplication. Its
reviewed exception is only the fixed action focus cycle, Escape, passive scrim,
conditional lifetime and host restore callback. Each action remains a native Button.

P5C replaces row TouchArea and Return/Space activation with visible native Button
surfaces. Only missing compound direction-key focus/expansion, scroll-into-view
and passive left-aligned/eliding content remain custom. Model validation observes
row notifications; no Rust adapter, router or selection owner was added.

P5D FluentFlyout inherits public PopupWindow, including actual is-open, Escape,
outside-click close and focus restoration. DropDown/Split compose visible native
Button owners with that popup. Slots own native children and scrolling; native
Menu/ContextMenuArea remains direct Gallery composition, with no Kit dispatcher.

P5E Expander uses a native header Button. Slint prohibits conditional @children,
so the host MUST conditionally instantiate its slot on the controlled expanded
value (and retained-page activity); the example verifies timer/input teardown.
InfoBar uses native IconButton closure, no timer, and a per-shown-cycle guard.
Forced programmatic focus on a disabled native Button can still enter its native
key handler; Expander and Split primary suppress the resulting callback when
disabled/hidden. This guard does not implement keyboard activation itself.

P6 adds one private TransientLifetime composition in primitives/private, shared by
Toast and Modal without a patterns/overlays dependency. It owns only cancelable
opacity progress and a bounded cleanup Timer; no input or business callback.
Native controls retain their animation ownership. The two overlay records include
the exact reviewed lifetime/input-disable changes; the private helper is guarded.
