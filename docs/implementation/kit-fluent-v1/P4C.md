# P4C — native containers and virtual lists

Base: `7f46e63f9bbdf4ccad52ff20f2553e496dfa8b74` (P4B). Both P4B source-package
commands passed after commit. Ordered P4/P5/P6 continuation is explicitly authorized.
Original root SPEC changes remain excluded; no push or consumer update.

## Current contract

Adds exactly five exports (48 public names / 34 visual components). ScrollView and
GroupBox inherit native layout/slots. ListView and TabWidget statically re-export
the actual public native implementation to preserve compiler-special child grammar.
ListView accepts one direct for child; TabWidget accepts fixed Tab children. Native
inactive tab content remains instantiated; hosts coordinate costly child lifetimes.

StandardListView composes one native view, shares its current-item storage and
forwards native callbacks. Its limited native enabled property is deliberately
named scrollbars-enabled: it does not disable row input. GroupBox/ScrollView do
not recursively disable arbitrary children either. Host programmatic selection
and model replacement are explicit; no custom scrolling, row activation or tabs.

Facade, current API/docs/probe, native manifest, status and conditional Gallery
native-containers page are synchronized. Snapshot adoption checked the exact
five additions, reviewed five StandardListView members and unchanged earlier APIs.
Native re-export positive/wrong-owner/missing-record tests cover all three identities;
unknown native exports still fail closed.

## Evidence

`target/button-checks/20260909T165734941579Z/result.json`: PASS, 16 public WindowEvent assertions. A 10,000-row Kit/native pair
creates 9 delegates initially, 18 cumulatively after scrolling to row 9999; both
report 240,000px extent. Empty/short/replaced models, programmatic versus user
selection, row-change notification, native wheel input, host-coordinated child
switch disable/reenable and programmatic/pointer tab selection pass. Five runtime
PNGs include the changed row, Dark theme and both tab bodies. Initial failures
(non-exhaustive Rust struct construction; wrong second-tab pointer coordinate)
remain in earlier target runs. Public Default-based item construction and actual
measured tab coordinates fix those harness defects.

`target/verify-p4c/result.json`: all seven commands PASS: fmt, boundaries/native,
74 Python tests, Gallery build, 40 Matrix captures and 30 paired release samples.
Stable source content: `8595f02ec8d423567ed2c502a53900189b136ebaafbcd8580a49fd6c5760710d`. Full Clippy/Rust tests
were last run at P4A; NOT_RUN again in this subbatch, reconciled at P4D. Source
packaging follows this unit's authorized commit.

Visuals: `target/verify-p4c/visual/`, Light/Dark, four sizes, simulated
100/125/150/200/225%. Manually inspected Light 1440x900/100 and isolated Dark
updated runtime. Lower specimens remain scrollable. Known P3 OS chrome capture
limitations remain; screenshots are not reader/IME/real monitor-transition evidence.

Raw paired performance: `target/perf-harness/20260909T165959743557Z/result.json`. Windows 26200,
Slint 1.17.1 Fluent/winit/software, release, 820x440, Light/100%, requested Segoe UI
Variable Text. Each side has a 780x400 native virtual view of 10,000 24px text rows.
External graph: `c4a8357c7edc1abf6220b729c74fdd387f32d2fd861fb89a0ff1ecc970b89534`. All 60 samples/outliers retained.

| Metric | Native A | Kit B | B-A |
|---|---:|---:|---:|
| Software buffer readiness ms p50 | 8.8757 | 8.8498 | -0.0259 |
| Software buffer readiness ms p95 | 9.7034 | 10.2537 | 0.5503 |
| Private Bytes at 2s p50 | 5263360.0000 | 5275648.0000 | 12288.0000 |
| Binary bytes p50 | 13922816.0000 | 13925888.0000 | 3072.0000 |

Unchanged binary and provisional software-buffer/2s-memory allowances PASS.
Actual presentation, interaction-frame p95, 60s idle CPU and platform accessibility
are NOT_RUN; this is not a full steady-state performance certification.

Gate: PASS for the scoped container contract. P4D follows after this report/commit;
P4 is not yet complete.
