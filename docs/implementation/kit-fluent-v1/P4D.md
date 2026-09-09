# P4D — native table and date/time pickers

Base: `6bca23dd0117c23fd322261272d50671cc96501a` (P4C). Both P4C source-package
checks passed after commit. Part of the explicitly ordered P4/P5/P6 request.
Original root SPEC changes remain excluded; no push, tag, consumer or toolchain change.

## Current contract and review

Adds FluentStandardTableView, FluentDatePicker, FluentTimePicker and native Date/Time
struct exports (53 public names / 37 visuals). Reviewed exact additions with no
change to earlier signatures. Table uses builtin TableColumn/StandardListViewItem,
native rows/columns/selection and sort callbacks; host sorting is deliberately
absent. Its native enabled only controls scrollbars, so the public name is
scrollbars-enabled. Rows/header actions stay native; no DataGrid editing promise.

Pickers compose a visible public Button and DatePickerPopup/TimePickerPopup in the
overlays layer. Host input is the only committed value; accepted(value) requests
host adoption. Cancel discards the popup draft; native confirmed/canceled handlers
close first, then Kit restores opener focus and forwards once. Imperative close()
emits no user request. Setting the picker's own visible=false or enabled=false closes it; open() checks both.
Hosts must close popups before hiding a retained ancestor, or unload that content.
No shown shadow, calendar/parser/timezone/clock or private widget code. Hosts supply
valid initial date/time; native text editing validates user input. Default labels
are simple numeric text and can be overridden for localization. Native year-list,
locale, keyboard and large-popup limits are not expanded by Kit.

Facade/current snapshot/probe, Gallery native-pickers page/catalog, native manifest
and documentation/status are synchronized. Public docs describe real inherited
and explicit state; native structs are exported without duplicate Kit definitions.

## Checks and runtime

`target/verify-p4d/result.json`: all nine commands PASS: fmt, Clippy all workspace
all targets/features with warnings denied, 20 Rust tests, boundaries/native,
75 Python tests, Gallery build, 40 Matrix captures and 30 paired release samples.
Stable source content `a29b4b04d80b0ad7a15022c1b4ba2e9a91840e99052ee0189cbdfe6ada3c08a5`. This is the full
P4 core reconciliation after all four batches. Packaging follows authorized commit.

`target/button-checks/20260909T171104854347Z/result.json`: PASS, 18 public WindowEvent assertions. Covers host table assignment,
ascending/descending requests without row mutation, pointer/key selection and empty
rows; leap-day date/time cancel/reopen/accept, exact request counts, host ownership,
focus restoration, disabled opener and ten rapid popup open/close cycles.
Native text input rejects 02/30/2024 and accepts 02/28/2024; accepted does not mutate
the host's 02/29/2024 until the host decides to adopt the request. All steps use
public input/focus or documented host setters/functions, not direct action callbacks.
Initial harness compilation failed because row is a Slint builtin property;
renaming the test property selected_row fixed it. Failed target evidence is retained.

Runtime Light/Dark images include open native date/time, bottom-right reposition,
invalid text with disabled OK, disabled popup cleanup and empty table. Manual review
confirmed the invalid-input control and fully removed disabled popup. Gallery Matrix
`target/verify-p4d/visual/` covers the closed-control page at four sizes and simulated
100/125/150/200/225%. This does not certify open popups on every real monitor/DPI,
OS chrome capture, locale combinations, native reader or all keyboard behavior.
Native calendar can require substantial height; no full WinUI picker equivalence.

## Paired measurement

`target/perf-harness/20260909T171824993322Z/result.json`: 30 alternating native/Kit pairs, Windows
26200, pinned Slint 1.17.1 Fluent/winit/software, release, Light/100%, 820x440,
requested Segoe UI Variable Text. Same 780x400 table, 100 two-cell rows and columns.
External graph: `c4a8357c7edc1abf6220b729c74fdd387f32d2fd861fb89a0ff1ecc970b89534`. All 60 raw samples retained.

| Metric | Native A | Kit B | B-A |
|---|---:|---:|---:|
| Software buffer readiness ms p50 | 9.9760 | 10.0115 | 0.0355 |
| Software buffer readiness ms p95 | 14.3812 | 11.1091 | -3.2721 |
| Private Bytes at 2s p50 | 5582848.0000 | 5627904.0000 | 45056.0000 |
| Binary bytes p50 | 14936576.0000 | 14949376.0000 | 12800.0000 |

Unchanged binary and provisional software-buffer/2s-memory allowances PASS.
Actual present, interaction-frame p95, 60s idle CPU and platform accessibility
remain NOT_RUN; no budgets, outliers or working sets were changed to pass.

Gate: PASS for the scoped typed native wrapper batch. See P4.md for all four units.
