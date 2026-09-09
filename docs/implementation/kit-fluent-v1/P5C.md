# P5C — bounded native navigation and reactive models

Base: `c73c1e4ce98e44ef350d1675bc3cddcc8dbf263f` (P5B); both package commands
passed. No routing state, required Rust adapter, virtual tree claim or backend upgrade.

## Reproducer and change

Old baseline `target/button-checks/20260909T175120548145Z/result.json` FAIL:
row-change duplicate IDs still admitted commands for 16/64/256 entries. Current
validity is a reactive binding, including both regions and row notifications;
new read-only model_valid exposes the actual result. Invalid combined models
suppress both regions. Pure validation scans only preceding same-region rows for
duplicates, preserving the complete validation rules with fewer model reads.
Native input is now two visible Button surfaces for label/optional arrow, with
passive left-aligned eliding text/icon/chevron. No duplicate TouchArea, Return,
Space or accessible default action remains. The final pending-command allowance
is removed. Native checked background replaces old custom row painting.

Up/Down focus enabled visible rows, crossing the primary/footer ends. Home/End
stay in the current region. Left/Right collapse/expand or recover an ancestor.
Native Tab/ShiftTab and Return/Space remain native. Focus scrolls into view through
public ScrollView viewport properties. Selection and expansion stay host-owned;
no callback is emitted solely on focus movement or host selection assignment.
Focus recovery settles through Slint's normal event loop. Models remain bounded
to 256 combined entries; no ListView virtualization was introduced.

## Evidence

Final `target/button-checks/20260909T181225220960Z/result.json`: PASS, Release,
152 assertions (32 model/input assertions plus 120 validation-sample assertions).
Whole replacement, row notifications, selected-only updates, nested collapse,
disabled recovery, compact separate regions, footer traversal and scrolled last
row pass. Selected-only changes read zero model rows at all four sizes. At 256,
settled replacement reads fall from baseline 101758 to 68862. This is observed
model access, not a claim of zero allocations or universal rendering speedup.

| Entries | Samples | Validation p50 ms | p95 ms | Max ms |
|---|---|---|---|---|
| 16 | 30 | 0.0970 | 0.1276 | 0.1461 |
| 64 | 30 | 0.4685 | 0.7141 | 0.8386 |
| 256 | 30 | 7.8046 | 8.1342 | 8.3517 |
| 257 | 30 | 0.0002 | 0.0023 | 1.1882 |

The 256-entry p95 is below the unchanged 16.67 ms target. All samples are retained
in runtime.log, including rejected 257-entry samples. Before baseline snapshot
cost included rendering; no fabricated before pure-validation distribution.
Earlier reactive/custom-row and native-row runs are retained. The compact label
initially failed because native default centering overlapped the arrow region;
explicit x/y fixes the geometry without shrinking native functionality. The failed
run `20260909T180602825047Z` remains available; final screenshots were inspected
for compact selected/arrow focus and expanded long-text elision.

`target/verify-p5c/result.json`: all eight checks PASS: fmt, full Clippy, 20 Rust
tests, boundary/current API/assets, native reuse, 75 Python tests, Gallery build,
40 navigation Matrix captures. Stable source `ccb5f4628bd42060743faa86a5cb84432d67d5ac41b5e0d7a42203dc336dcbd7`.
Native manifest/probe/docs/current API are synchronized; exactly model-valid was
added to NavigationView across the existing 53 exports. Report/status edits occur
after this source identity; packaging follows the authorized commit.

Windows 26200, Slint 1.17.1 Fluent/winit/software; runtime 760x520 at 100%, Gallery
Light/Dark four sizes and simulated 100/125/150/200/225%. Public WindowEvent input
is separate from native OS accessibility. Reader, IME, real monitor transitions,
long-idle CPU, actual presentation frame timing and full application performance
remain NOT_RUN in this scoped batch. Scoped P5C PASS; continue with P5D.
