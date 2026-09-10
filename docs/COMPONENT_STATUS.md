# Component status

IMPLEMENTED means current code exists, not that every acceptance category passed.
The table reconciles the public facade, native manifest and Gallery catalog; it is
not a runtime registry. Signatures and defaults live in [PUBLIC_API](PUBLIC_API.md).
Verification gaps live in [STATUS](STATUS.md), and scoped historical input, visual,
performance and accessibility results live in [HISTORY](HISTORY.md).

## Current visual components

| Component | Implementation / native basis | Current contract and gap | Gallery scene |
|---|---|---|---|
| Badge | IMPLEMENTED / presenter; builtin presentation / composition | Retained minimal Rectangle/Text; semantic colors unchanged | badge |
| EmptyState | IMPLEMENTED / composed; builtin presentation / composition | Removed unused milestone; title/message wrap; passive card helpers conditional | empty-state |
| FluentButton | IMPLEMENTED / native-wrapper; Button | Visible native Button owns activation, focus and disabled behavior | fluent-button |
| FluentIcon | IMPLEMENTED / presenter; builtin presentation / composition | Retained Image and optical-offset container; no resource scan | icons |
| FluentTextArea | IMPLEMENTED / native-wrapper; TextEdit | Native TextEdit retained; actual multiline Unicode/disabled edits checked | fluent-text-area |
| FluentTextField | IMPLEMENTED / native-wrapper; LineEdit | Removed preview; top-aligned native editor; error caption reserves wrapping height | fluent-text-field |
| IconButton | IMPLEMENTED / native-wrapper; Button, Tooltip | Native Button and Tooltip; native icon sizing; named action; passive danger outline | icon-button |
| MetricCard | IMPLEMENTED / composed; builtin presentation / composition | Label/value/hint wrap; passive SurfaceCard helper reduction | metric-card |
| ModalManager | IMPLEMENTED / reviewed-exception; Button | Fixed native actions; host restore; bounded opacity with immediate input release | modal-manager |
| NavigationBackButton | IMPLEMENTED / reviewed-exception; FocusScope, TouchArea, Tooltip | Shared borderless action; disabled, key/pointer cancellation; host callback | navigation-foundations |
| NavigationContentSurface | IMPLEMENTED / presenter; builtin presentation / composition | Retained clipped host viewport; no routing, implicit padding or new effects | navigation-foundations |
| NavigationPaneToggleButton | IMPLEMENTED / composed; Button, Tooltip | Visible native Button owns icon/action/expanded semantics; pane_mode stays controlled | navigation-foundations |
| NavigationView | IMPLEMENTED / reviewed-exception; Button, LineEdit, ScrollView, Tooltip | Private flat rows, centered compact icons, expanded-only search/chevrons; native toggle/search/scroll/tooltip; bounded focus and reactive combined model validation | navigation-view |
| PageHeader | IMPLEMENTED / composed; Button | Native action stacks below 420px; title/subtitle wrap | page-header |
| SectionHeader | IMPLEMENTED / composed; builtin presentation / composition | Title/description wrap; retained passive badge and trailing slot | section-header |
| SegmentButton | IMPLEMENTED / native-wrapper; Button | Native non-toggling Button; selected remains host input; native checked fill | segment-button |
| SettingRow | IMPLEMENTED / composed; builtin presentation / composition | Own text uses disabled token; host binds slot enabled; long title/description wrap | setting-row |
| SurfaceCard | IMPLEMENTED / reviewed-exception; builtin presentation / composition | No preview or whole-subtree dimming; conditional pointer/a11y; retained optional slot-action exception | surfaces |
| ToastHost | IMPLEMENTED / reviewed-exception; Button, Tooltip | Host state; one request; bounded opacity and immediate close input disable | toast-host |
| TooltipHost | IMPLEMENTED / presenter; builtin presentation / composition | Passive content; native Tooltip service; edge snapshot crop documented | tooltip-host |
| WindowControlButton | IMPLEMENTED / composed; Button, Tooltip | Composes IconButton; removed symbol; enabled and explicit 46x40 default; host owns OS action | window-control-button |
| FluentCheckBox | IMPLEMENTED / native-wrapper; CheckBox | Native inherited selection; P4A contract | native-selection |
| FluentSwitch | IMPLEMENTED / native-wrapper; Switch | Native inherited selection; P4A contract | native-selection |
| FluentRadioGroup | IMPLEMENTED / native-wrapper; RadioGroup | Native inherited selection; P4A contract | native-selection |
| FluentComboBox | IMPLEMENTED / native-wrapper; ComboBox | Native inherited selection; P4A contract | native-selection |
| FluentSlider | IMPLEMENTED / native-wrapper; Slider | P4B native numeric/progress contract | native-numeric |
| FluentSpinBox | IMPLEMENTED / native-wrapper; SpinBox | P4B native numeric/progress contract | native-numeric |
| FluentProgressBar | IMPLEMENTED / native-wrapper; ProgressIndicator | P4B native numeric/progress contract | native-numeric |
| FluentProgressRing | IMPLEMENTED / native-wrapper; Spinner | P4B native numeric/progress contract | native-numeric |
| FluentScrollView | IMPLEMENTED / native-wrapper; ScrollView | P4C native container contract and limitations | native-containers |
| FluentListView | IMPLEMENTED / native-wrapper; ListView | P4C native container contract and limitations | native-containers |
| FluentStandardListView | IMPLEMENTED / native-wrapper; StandardListView | P4C native container contract and limitations | native-containers |
| FluentGroupBox | IMPLEMENTED / native-wrapper; GroupBox | P4C native container contract and limitations | native-containers |
| FluentTabWidget | IMPLEMENTED / native-wrapper; TabWidget | P4C native container contract and limitations | native-containers |
| FluentStandardTableView | IMPLEMENTED / native composition; StandardTableView | P4D typed values and request contract | native-pickers |
| FluentDatePicker | IMPLEMENTED / native composition; DatePickerPopup | P4D typed values and request contract | native-pickers |
| FluentTimePicker | IMPLEMENTED / native composition; TimePickerPopup | P4D typed values and request contract | native-pickers |
| FluentFlyout | IMPLEMENTED / composed; native PopupWindow | Native popup ownership; host content actions | native-popups |
| FluentDropDownButton | IMPLEMENTED / composed; native PopupWindow and Button | Native popup ownership; host content actions | native-popups |
| FluentSplitButton | IMPLEMENTED / composed; native PopupWindow and Button | Native popup ownership; host content actions | native-popups |
| FluentExpander | IMPLEMENTED / composed; Button | Controlled header; mandatory host conditional slot lifetime | native-inline |
| FluentInfoBar | IMPLEMENTED / composed; Button, Tooltip | Inline status; once-per-cycle close request; no auto timer | native-inline |

## Boundaries

Not delivered: general TreeView, arbitrary ContentDialog, rich-text editor, complete
DataGrid, CalendarView, draggable multiwindow TabView, WebView, media/maps services,
automatic search, system notification center, Mica/Acrylic or Lottie/particle engines.
These are distinct future scopes, not missing aliases for the existing controls.

Per-component evidence is historical at its recorded source. See [STATUS](STATUS.md)
for remaining cross-component/platform gates and [HISTORY](HISTORY.md) for exact
source/evidence pointers. A static declaration or screenshot does not certify input,
performance or screen-reader behavior.
