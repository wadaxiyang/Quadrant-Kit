# Component status — P2 current contract

IMPLEMENTED means current code exists; VERIFIED is reserved for a named check at
an identified source/environment. PARTIAL names a limited contract or verification
scope. NOT_RUN means that dimension was not executed in P2. BACKLOG is planned or
explicitly excluded work. Static guard PASS never implies input, visual, performance
or screen-reader acceptance. P0 historical evidence stays at its original source.

## Current visual components

Contracts/defaults are in PUBLIC_API.md; per-component source/custom behavior and
slots are in implementation/kit-fluent-v1/AUDIT.md. The table is reconciled with the
facade, native manifest and real Gallery catalog, not used as a runtime registry.

| Component | Implementation / native basis | Current contract and gap | Gallery scene | Input P2 | Visual P2 | Performance evidence | Accessibility P2 |
|---|---|---|---|---|---|---|---|
| Badge | IMPLEMENTED / presenter; builtin presentation / composition | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | badge | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| EmptyState | IMPLEMENTED / composed; builtin presentation / composition | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | empty-state | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| FluentButton | IMPLEMENTED / native-wrapper; Button | Native command; current API and scoped evidence in P2 report | fluent-button | VERIFIED: P2 event + Windows input | VERIFIED: scoped light/dark states | P2 paired release comparison; see report | PARTIAL: one named native button; Windows indexed actions; no reader certification |
| FluentIcon | IMPLEMENTED / presenter; builtin presentation / composition | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | icons | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| FluentTextArea | IMPLEMENTED / native-wrapper; TextEdit | PARTIAL: Two-way native editor; IME/scrolling runtime pending | fluent-text-area | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| FluentTextField | IMPLEMENTED / native-wrapper; LineEdit | PARTIAL: Two-way native editor; IME/focus/error layout runtime pending | fluent-text-field | NOT_RUN | NOT_RUN | P0 Release n=3 smoke; not budget acceptance | NOT_RUN |
| IconButton | IMPLEMENTED / custom/pending-migration; builtin presentation / composition | PARTIAL: Icon command; custom input/inline tooltip pending P3/P5A | icon-button | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| MetricCard | IMPLEMENTED / composed; builtin presentation / composition | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | metric-card | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| ModalManager | IMPLEMENTED / reviewed-exception; Button | PARTIAL: Single confirmation; Return/Tab/restoration incomplete; P5B | modal-manager | PARTIAL: P2 request counts | PARTIAL: P2 confirmation buttons | NOT_RUN | PARTIAL: native button actions; no containment/reader claim |
| NavigationBackButton | IMPLEMENTED / composed; Tooltip | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | navigation-foundations | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| NavigationContentSurface | IMPLEMENTED / presenter; builtin presentation / composition | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | navigation-foundations | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| NavigationPaneToggleButton | IMPLEMENTED / composed; Tooltip | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | navigation-foundations | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| NavigationView | IMPLEMENTED / reviewed-exception; LineEdit, ScrollView, Tooltip | PARTIAL: Host-owned navigation; bounded 256/model; behavior/perf pending P5C | navigation-view | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| PageHeader | IMPLEMENTED / composed; Button | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | page-header | PARTIAL: P2 action forwarding | PARTIAL: P2 button sizing | NOT_RUN | PARTIAL: native button action |
| SectionHeader | IMPLEMENTED / composed; builtin presentation / composition | PARTIAL: Current presentation/composition exists; runtime dimensions not certified | section-header | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| SegmentButton | IMPLEMENTED / custom/pending-migration; builtin presentation / composition | PARTIAL: Host-controlled selected; custom input pending P3 | segment-button | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| SettingRow | IMPLEMENTED / composed; builtin presentation / composition | PARTIAL: Content slot; enabled must bind to actual child controls | setting-row | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| SurfaceCard | IMPLEMENTED / reviewed-exception; builtin presentation / composition | PARTIAL: Optional card action/slot; input helpers retained; scoped exception | surfaces | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| ToastHost | IMPLEMENTED / reviewed-exception; builtin presentation / composition | PARTIAL: One dismissal request surface; retained hidden tree; P5A lifecycle pending | toast-host | NOT_RUN | NOT_RUN | P0 Release n=3 smoke; not budget acceptance | NOT_RUN |
| TooltipHost | IMPLEMENTED / presenter; builtin presentation / composition | PARTIAL: Inline content presenter; native tooltip service supplied by host/composition | tooltip-host | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |
| WindowControlButton | IMPLEMENTED / custom/pending-migration; builtin presentation / composition | PARTIAL: Action presenter only; custom input pending P3; OS actions host-owned | window-control-button | NOT_RUN | NOT_RUN | NOT_RUN | NOT_RUN |

Private NavigationItemRow is separately recorded by the native guard (ordinary
input pending P5C); NavigationModel is a pure private global, not a visual export.
Existing historical navigation input observations do not become P1 runtime tests.

## Planned names and independently gated batches

The following names are the P1 plan, not exports or implemented APIs. Use Fluent
for new wrappers; no cosmetic renaming of current components. Property/direction/
base and supported behavior are defined per batch from the pinned public controls.
No empty facade aliases or historical compatibility paths will be introduced.

| Stage | Planned public components | Native basis / limits | Status |
|---|---|---|---|
| P2 | Existing FluentButton migration | Actual Button; passive danger outline, removed preview/accent/show_icon | PASS — scoped migration; see P2 report and native sizing limits |
| P3 | Remaining current components | Propagate validated native route; tokens/slots only in scope | NOT_STARTED |
| P4A | FluentCheckBox, FluentSwitch, FluentRadioGroup, FluentComboBox | CheckBox/Switch/RadioGroup/ComboBox; no assumed three-state or RadioGroup index | BACKLOG |
| P4B | FluentSlider, FluentSpinBox, FluentProgressBar, FluentProgressRing | Slider/int SpinBox/ProgressIndicator/Spinner; no floating NumberBox claim | BACKLOG |
| P4C | FluentScrollView, FluentListView, FluentStandardListView, FluentGroupBox, FluentTabWidget | ScrollView/ListView/StandardListView/GroupBox/TabWidget; resolve @children/compiler virtualization first | BACKLOG |
| P4D | FluentStandardTableView, FluentDatePicker, FluentTimePicker | StandardTableView, DatePickerPopup/TimePickerPopup; actual structs/ranges and finite popup contract | BACKLOG |
| P5A | Tooltip service and existing ToastHost | Builtin Tooltip plus content; finite toast lifecycle | NOT_STARTED |
| P5B | Existing ModalManager | Fixed confirmation, explicit focus protocol; not complete ContentDialog | NOT_STARTED |
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
