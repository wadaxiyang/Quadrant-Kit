# P4B — native numeric and progress controls

Base: `666b9e48ee49f789c1960cc8413d04fcf0486142` (P4A). P4A's two source-package
checks passed after its authorized commit. This unit is part of the explicitly
ordered P4/P5/P6 request. Original root SPEC changes remain excluded; no push,
tag, consumer change or runtime/toolchain upgrade.

## Implementation and contract review

FluentSlider and FluentSpinBox inherit public native Slider/SpinBox, retaining one
native input owner, native storage, float/int types and their real callbacks.
There is no manual drag, key activation, numeric parsing or shadow value.
Slider has no native read-only property; SpinBox has read-only. Hosts normalize
programmatic values/ranges: assignments are not automatically clamped and do not
emit edited/changed. Native boundary attempts may report an unchanged value.

FluentProgressBar/FluentProgressRing conditionally contain public
ProgressIndicator/Spinner. progress presentation clamps to 0..1; running=false
stops native indeterminate motion, and visible=false removes the native child.
animating reports requested indeterminate state, not measured CPU. Hosts retaining
hidden ancestor trees must bind running to host activity or unload their content.
No Kit Timer, extra native animation or input handler. Default bar height is 3px;
ring is 32x32; callers can supply explicit dimensions.

The reviewed API adds exactly four names (43 public names / 29 visuals), with
eight explicitly declared progress properties. Earlier contracts remain identical.
Facade, current probe, manifest/status and conditional native-numeric Gallery page
are synchronized. The snapshot was adopted only after checking the exact four
additions, unchanged earlier contracts and the reviewed progress property set.

## Checks

`target/verify-p4b/result.json` retains exact commands, exit codes and logs for
source content `ecd3372f88591656e27d3b0ec19cbcbe11133cc749de823713fcf0787a079793`; source was stable throughout.
All seven commands PASS / exit 0: fmt; boundaries/API/assets/resolved graph; native
reuse; 73 Python tests; Gallery/probe build; 40 Matrix captures; 30 paired release
samples. Full Rust tests/Clippy were last run at P4A and are NOT_RUN again in this
subbatch; full P4 reconciliation follows the remaining subbatches. No Rust runtime
application logic changed. Packaging runs after this unit's authorized commit.

`python scripts/run_button_checks.py --suite numeric`: PASS, 17 public WindowEvent
assertions, `target/button-checks/20260909T163336715026Z/result.json`. Covers programmatic updates, Home/End/steps, bounded
native drag/release, disabled pointer/key input, out-of-range host assignment,
SpinBox read-only/buttons/editing, stopping/hiding/reopening progress and identical
software frames 500ms apart after stopping. No direct invocation of Kit actions.

Earlier FAIL runs remain under `target/button-checks/`: the first used the native
decrement arrow when expecting increment. Screenshot review also found a real
absolute-layout bar-height defect; explicit sizing fixed it. The intermediate
fixed-size plus min-size declaration was rejected by Slint and removed. The final
runtime/Gallery build uses the valid current geometry; failures were not relabeled.

## Visual and performance evidence

Gallery captures: `target/verify-p4b/visual/`, Light/Dark, four sizes from 760x520
to 1440x900, simulated 100/125/150/200/225%. Windows 26200, Slint 1.17.1 Fluent,
winit/software, requested Segoe UI Variable Text. Manual inspection includes the
isolated Dark stopped frame; native bar/ring render correctly at fixed dimensions.
Caption composition limitations from P3 remain; capture is not native OS chrome,
WinUI runtime equivalence, real monitor transition, IME or reader certification.

Raw performance: `target/perf-harness/20260909T163828762838Z/result.json`. 30 alternating native/Kit
pairs; 50 bars plus 50 rings each, release, 820x440, Light, 100%, identical geometry,
font and content; external graph `c4a8357c7edc1abf6220b729c74fdd387f32d2fd861fb89a0ff1ecc970b89534`.

| Metric | Native A | Kit B | B-A |
|---|---:|---:|---:|
| Software-buffer readiness ms p50 | 7.4835 | 7.5811 | 0.0976 |
| Software-buffer readiness ms p95 | 8.5976 | 10.1139 | 1.5163 |
| Private Bytes at 2s p50 | 5595136.0000 | 5681152.0000 | 86016.0000 |
| Binary bytes p50 | 11755520.0000 | 11797504.0000 | 41984.0000 |

The unchanged binary and provisional software-buffer timing allowances pass; the
2s memory-point delta is below the unchanged allowance. Memory changes between
1s/2s are preserved in raw samples; this is not a 60s steady-idle result. Actual
present, interaction-frame p95, long-idle CPU and platform reader/IME are NOT_RUN.
No budget was relaxed and no outliers or working sets were trimmed.

Gate: PASS for scoped native numeric/progress behavior and paired sampling.
P4C starts after this report and commit; P4 is not yet complete.
