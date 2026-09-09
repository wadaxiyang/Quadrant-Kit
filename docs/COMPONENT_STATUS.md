# Component status — P3 current contract

IMPLEMENTED means current code exists; VERIFIED is reserved for a named check at
an identified source/environment. PARTIAL names a limited contract or verification
scope. NOT_RUN means that dimension was not executed in P3. BACKLOG is planned or
explicitly excluded work. Static guard PASS never implies input, visual, performance
or screen-reader acceptance. P0 historical evidence stays at its original source.

## Current visual components

Contracts/defaults are in PUBLIC_API.md; per-component source/custom behavior and
slots are in implementation/kit-fluent-v1/AUDIT.md. The table is reconciled with the
facade, native manifest and real Gallery catalog, not used as a runtime registry.

| Component | Implementation / native basis | Current contract and gap | Gallery scene | Input P3 | Visual P3 | Performance evidence | Accessibility P3 |
|---|---|---|---|---|---|---|---|
| Badge | IMPLEMENTED / presenter; builtin presentation / composition | Retained minimal Rectangle/Text; semantic colors unchanged | badge | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| EmptyState | IMPLEMENTED / composed; builtin presentation / composition | Removed unused milestone; title/message wrap; passive card helpers conditional | empty-state | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| FluentButton | IMPLEMENTED / native-wrapper; Button | P2 visible Button retained; P3 regression suite | fluent-button | P2 regression suite | VERIFIED: catalog render; manual subset in P3.md | P2 report | NOT_RUN: reader
| FluentIcon | IMPLEMENTED / presenter; builtin presentation / composition | Retained Image and optical-offset container; no resource scan | icons | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| FluentTextArea | IMPLEMENTED / native-wrapper; TextEdit | Native TextEdit retained; actual multiline Unicode/disabled edits checked | fluent-text-area | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| FluentTextField | IMPLEMENTED / native-wrapper; LineEdit | Removed preview; top-aligned native editor; error caption reserves wrapping height | fluent-text-field | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| IconButton | IMPLEMENTED / native-wrapper; Button, Tooltip | Native Button and Tooltip; native icon sizing; named action; passive danger outline | icon-button | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | P3 paired Release; steady memory NOT_RUN | PARTIAL: native Windows actions; no reader
| MetricCard | IMPLEMENTED / composed; builtin presentation / composition | Label/value/hint wrap; passive SurfaceCard helper reduction | metric-card | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| ModalManager | IMPLEMENTED / reviewed-exception; Button | Fixed native actions; bounded keyboard cycle and host restore callback; not full ContentDialog | modal-manager | VERIFIED: P5B finite input | VERIFIED: P5B runtime/Matrix scope | NOT_RUN: P5B release | PARTIAL: finite focus; reader NOT_RUN |
| NavigationBackButton | IMPLEMENTED / composed; Button, Tooltip | IconButton forwards name/tooltip; passive arrow; native disabled behavior | navigation-foundations | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | PARTIAL: native Windows actions; no reader
| NavigationContentSurface | IMPLEMENTED / presenter; builtin presentation / composition | Retained clipped host viewport; no routing, implicit padding or new effects | navigation-foundations | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| NavigationPaneToggleButton | IMPLEMENTED / composed; Button, Tooltip | Visible native Button owns icon/action/expanded semantics; pane_mode stays controlled | navigation-foundations | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | PARTIAL: native Windows actions; no reader
| NavigationView | IMPLEMENTED / reviewed-exception; Button, LineEdit, ScrollView, Tooltip | Only migrated search name and button dependencies; row command/model work remains P5C | navigation-view | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| PageHeader | IMPLEMENTED / composed; Button | Native action stacks below 420px; title/subtitle wrap | page-header | P2 regression suite | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| SectionHeader | IMPLEMENTED / composed; builtin presentation / composition | Title/description wrap; retained passive badge and trailing slot | section-header | NOT_RUN: passive/composition scope | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| SegmentButton | IMPLEMENTED / native-wrapper; Button | Native non-toggling Button; selected remains host input; native checked fill | segment-button | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | P3 paired Release; steady memory NOT_RUN | PARTIAL: native Windows actions; no reader
| SettingRow | IMPLEMENTED / composed; builtin presentation / composition | Own text uses disabled token; host binds slot enabled; long title/description wrap | setting-row | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | PARTIAL: native Windows actions; no reader
| SurfaceCard | IMPLEMENTED / reviewed-exception; builtin presentation / composition | No preview or whole-subtree dimming; conditional pointer/a11y; retained optional slot-action exception | surfaces | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | NOT_RUN: reader
| ToastHost | IMPLEMENTED / reviewed-exception; Button, Tooltip | One dismissal per host-visible cycle; conditional native child/Timer; hover pause | toast-host | VERIFIED: P5A runtime | VERIFIED: P5A scoped images | P0 binary attribution debt; CPU/frame NOT_RUN | NOT_RUN: reader
| TooltipHost | IMPLEMENTED / presenter; builtin presentation / composition | Passive content; native Tooltip service; edge snapshot crop documented | tooltip-host | VERIFIED: P5A native hover/focus scope | VERIFIED: inside popup; edge crop P5A | NOT_RUN | NOT_RUN: reader
| WindowControlButton | IMPLEMENTED / composed; Button, Tooltip | Composes IconButton; removed symbol; enabled and explicit 46x40 default; host owns OS action | window-control-button | VERIFIED: scoped P3 runtime | VERIFIED: catalog render; manual subset in P3.md | NOT_RUN | PARTIAL: native Windows actions; no reader

| FluentCheckBox | IMPLEMENTED / native-wrapper; CheckBox | Native inherited selection; P4A contract | native-selection | P4A report | P4A report | P4A report | NOT_RUN: reader |
| FluentSwitch | IMPLEMENTED / native-wrapper; Switch | Native inherited selection; P4A contract | native-selection | P4A report | P4A report | P4A report | NOT_RUN: reader |
| FluentRadioGroup | IMPLEMENTED / native-wrapper; RadioGroup | Native inherited selection; P4A contract | native-selection | P4A report | P4A report | P4A report | NOT_RUN: reader |
| FluentComboBox | IMPLEMENTED / native-wrapper; ComboBox | Native inherited selection; P4A contract | native-selection | P4A report | P4A report | P4A report | NOT_RUN: reader |

| FluentSlider | IMPLEMENTED / native-wrapper; Slider | P4B native numeric/progress contract | native-numeric | P4B report | P4B report | P4B report | NOT_RUN: reader |
| FluentSpinBox | IMPLEMENTED / native-wrapper; SpinBox | P4B native numeric/progress contract | native-numeric | P4B report | P4B report | P4B report | NOT_RUN: reader |
| FluentProgressBar | IMPLEMENTED / native-wrapper; ProgressIndicator | P4B native numeric/progress contract | native-numeric | P4B report | P4B report | P4B report | NOT_RUN: reader |
| FluentProgressRing | IMPLEMENTED / native-wrapper; Spinner | P4B native numeric/progress contract | native-numeric | P4B report | P4B report | P4B report | NOT_RUN: reader |

| FluentScrollView | IMPLEMENTED / native-wrapper; ScrollView | P4C native container contract and limitations | native-containers | P4C report | P4C report | P4C report | NOT_RUN: reader |
| FluentListView | IMPLEMENTED / native-wrapper; ListView | P4C native container contract and limitations | native-containers | P4C report | P4C report | P4C report | NOT_RUN: reader |
| FluentStandardListView | IMPLEMENTED / native-wrapper; StandardListView | P4C native container contract and limitations | native-containers | P4C report | P4C report | P4C report | NOT_RUN: reader |
| FluentGroupBox | IMPLEMENTED / native-wrapper; GroupBox | P4C native container contract and limitations | native-containers | P4C report | P4C report | P4C report | NOT_RUN: reader |
| FluentTabWidget | IMPLEMENTED / native-wrapper; TabWidget | P4C native container contract and limitations | native-containers | P4C report | P4C report | P4C report | NOT_RUN: reader |

| FluentStandardTableView | IMPLEMENTED / native composition; StandardTableView | P4D typed values and request contract | native-pickers | P4D report | P4D report | P4D report | NOT_RUN: reader |
| FluentDatePicker | IMPLEMENTED / native composition; DatePickerPopup | P4D typed values and request contract | native-pickers | P4D report | P4D report | P4D report | NOT_RUN: reader |
| FluentTimePicker | IMPLEMENTED / native composition; TimePickerPopup | P4D typed values and request contract | native-pickers | P4D report | P4D report | P4D report | NOT_RUN: reader |

Private NavigationItemRow is separately recorded by the native guard (ordinary
input pending P5C); NavigationModel is a pure private global, not a visual export.
Existing historical navigation input observations do not become P1 runtime tests.

## Planned names and independently gated batches

P4A selection names above are now implemented; later batches below remain planned.

The following names are the P1 plan, not exports or implemented APIs. Use Fluent
for new wrappers; no cosmetic renaming of current components. Property/direction/
base and supported behavior are defined per batch from the pinned public controls.
No placeholder exports or historical compatibility paths are introduced; compiler-special controls use verified public native re-exports.

| Stage | Planned public components | Native basis / limits | Status |
|---|---|---|---|
| P2 | Existing FluentButton migration | Actual Button; passive danger outline, removed preview/accent/show_icon | PASS — scoped migration; see P2 report and native sizing limits |
| P3 | Remaining current components | Native buttons; tokens/slots/wrapping; all original 21 accounted for | PARTIAL — core gate PASS; WinUI reference NOT_RUN; P3 package PASS after commit |
| P4A | FluentCheckBox, FluentSwitch, FluentRadioGroup, FluentComboBox | CheckBox/Switch/RadioGroup/ComboBox; no assumed three-state or RadioGroup index | PASS — P4A scoped contract |
| P4B | FluentSlider, FluentSpinBox, FluentProgressBar, FluentProgressRing | Slider/int SpinBox/ProgressIndicator/Spinner; no floating NumberBox claim | PASS — P4B scoped contract |
| P4C | FluentScrollView, FluentListView, FluentStandardListView, FluentGroupBox, FluentTabWidget | ScrollView/ListView/StandardListView/GroupBox/TabWidget; verified direct repeater / fixed Tab grammar | PASS — P4C scoped contract |
| P4D | FluentStandardTableView, FluentDatePicker, FluentTimePicker | StandardTableView, DatePickerPopup/TimePickerPopup; actual structs and finite popup contract | PASS — P4D scoped contract |
| P5A | Tooltip service and existing ToastHost | Builtin Tooltip plus content; finite toast lifecycle | PASS — P5A scoped contract |
| P5B | Existing ModalManager | Fixed confirmation, explicit focus protocol; not complete ContentDialog | PARTIAL — finite P5B input PASS; reader NOT_RUN |
| P5C | Existing NavigationView/rows | Measure 16/64/256/257 before optimization; no host router | NOT_STARTED |
| P5D | FluentFlyout, FluentDropDownButton, FluentSplitButton; menu composition examples | PopupWindow/Menu/ContextMenuArea and native Button; no new Menu runtime service | BACKLOG |
| P5E | FluentExpander, FluentInfoBar | Native command plus bounded composition; not a custom input framework | BACKLOG |

## Explicitly outside this evolution's completion claim

| Capability | Reason | Status |
|---|---|---|
| General large TreeView | No full std equivalent; needs distinct hierarchy/virtualization contract | BACKLOG |
| Arbitrary-content ContentDialog | Current fixed confirmation lacks general focus/a11y containment | BACKLOG |
| Rich text editor | Beyond public standard plain-text editing scope | BACKLOG |
| Complete DataGrid | Editable/frozen/custom cell system exceeds StandardTableView | BACKLOG |
| CalendarView | General calendar surface exceeds the public date popup contract | BACKLOG |
| Draggable multiwindow TabView | Requires new interaction/platform ownership beyond TabWidget | BACKLOG |
| WebView | Requires new platform/runtime boundary | BACKLOG |
| Media playback | Requires new platform/runtime boundary | BACKLOG |
| Maps | Requires service/runtime and domain responsibilities | BACKLOG |
| Automatic search service | Host/service responsibility, not component state | BACKLOG |
| System notification center | Platform/service scope beyond inline Toast | BACKLOG |
| Mica/Acrylic material engine | Requires independent rendering/platform work | BACKLOG |
| Lottie/particle effects | Outside restrained common-control scope | BACKLOG |
