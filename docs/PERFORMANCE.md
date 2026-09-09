# Performance protocol — P2 measurement and retained budgets

P0 adds a neutral, generated harness instead of measuring Gallery as a button.
Root Kit retains zero normal/runtime dependency edges. That fact is not a speed
or memory result. P0 smoke results do not certify release performance.

```console
python scripts/run_perf.py --help
python scripts/run_perf.py --generate-only
python scripts/run_perf.py --profile debug --samples 3
python scripts/run_perf.py --profile release --samples 30
python scripts/run_perf.py --native-probe
```

Each invocation creates fresh projects only in `target/perf-harness/<UTC run>/`.
Native and Kit variants use the same template/features, Fluent, embedded resources,
winit-software, system Segoe UI Variable Text, 100%, Light, 820×440. Scenarios are
empty, import-only, one Button, 100 Buttons, text input, hidden Toast. Empty/hidden
native controls are empty-window references (there is no std Toast). The 100
buttons are 100 real declarations in a for, all within the visible window; there
is no Gallery catalog/page/platform adapter in the measurement binary.

The only local Kit path whitelist is inside generated test projects. It never
changes external Tasks source rules. Every project seeds Cargo.lock from the
workspace, explicitly reconciles it with `cargo metadata --offline`, saves the
resolved output/lock, then builds `--locked --offline`. Build profile/toolchain
and actual commands are recorded. Dependencies share the existing target cache;
project/binary paths and raw logs stay separate. This is a new-process warm-cache
measurement, not a cold build/start claim. No working-set trimming/cache flush.

The Rust clock starts at main entry, and separately marks construction and show
return. `first_render_callback_ms` is emitted only for an actual AfterRendering
hook. Unsupported hooks produce no numeric sample, never zero or a substitute
present timestamp. It excludes OS loader time and is not actual display present.
The P2 external Windows sampler reads PrivateUsage (Private Bytes) at 1 and 2
seconds after Popen, plus WorkingSetSize at 2 seconds. Their within-process
difference is recorded; it is evidence about this interval, not a 60-second idle
or general steady-state guarantee. A 1.5s timer marker establishes a working event loop; the
2.5s timer exits. These two harness timers are identical in A/B and must not be
counted as Kit idle work. The short fixed sample is not 60-second idle acceptance.

Order alternates native/Kit then Kit/native. All samples are retained with no
outlier deletion; summary uses nearest-rank p50/p95, n, min/max. Three samples
validate plumbing and reveal gross spread only; at least 30 paired release runs
and controlled environment noise are needed before freezing quantitative gates.
GPU memory, exact present/process-spawn timing, interaction frames, layout/instance
counts, 60-second CPU/ticks, real IME, navigation 16/64/256/257 and 100-cycle
overlay/page lifetimes are NOT_RUN until explicitly measured in later phases.

## P1 decisions from actual P0 evidence

P0 Release data at `target/perf-harness/20260909T030437314388Z/result.json` has
three paired new processes per side, with construction ranges of 0.246–2.278ms.
All 36 rendering hooks were unsupported. Construction is not startup/present, and
a 2s memory observation is not established steady state. Therefore P1 does not
use those samples to freeze startup, memory or interaction thresholds. The exact
missing conditions below are obligations, not deleted budgets. No UI behavior
changed in P1. P2 changes FluentButton and extends the harness, so P0 measurements
remain historical pre-migration evidence, not a directly interchangeable sample set.

**Frozen now:** matched simple-scene binary delta ≤ max(512KiB, 5% of A), with
identical release toolchain/features/geometry/state and explicitly accounted
required assets. This deterministic metric does not need process timing samples.
Also fixed: no new unnecessary hidden animation/timer work, no working-set trimming,
no sacrificing input/accessibility, and the current 256-item-per-model limit.
Their runtime verification is distinct from freezing the engineering constraint.

The hidden-Toast raw binary delta is 2,264,064 bytes versus empty (2.159MiB).
It is above the unadjusted simple-scene allowance and remains **unaccepted** until
code/assets and scope are attributed. A missing std Toast is not permission to
raise the budget or claim equivalent A/B behavior. P5A/P7 must retain this finding.

## Retained SPEC thresholds and waiting conditions

Let A be native and B be Kit; delta is B−A. None is silently relaxed or removed.

| Metric | Threshold (unchanged) | P1 decision / waiting condition |
|---|---|---|
| Startup p50 | ≤ max(10ms, 10% of A) | PROVISIONAL: distinguish loader/main/render/present; acquire supported marker and ≥30 paired release runs before P2 acceptance |
| Startup p95 | ≤ max(20ms, 15% of A) | PROVISIONAL: same marker and ≥30 pairs, retain min/max/outliers, improve sampler if noise exceeds allowance |
| Steady Private Bytes | ≤ max(2MiB, 5% of A) | PROVISIONAL: prove settling interval, ≥30 pairs and count-dependent slope in P2/P7 |
| Interaction frame p95 | ≤ max(1ms, 10% of A), preserve 60Hz when native sustains it | PROVISIONAL: actual frame sampler and matched real input in P2/P7 |
| Idle CPU increment | ≤0.2 percentage points, no unnecessary sustained redraw | PROVISIONAL numeric limit: ≥60s controlled idle/native noise and separate harness/OS activity in P5/P7 |
| Simple binary increment | ≤ max(512KiB, 5% of A) | FROZEN: release, identical features/assets and resolution; unmatched Toast comparison unaccepted |
| Navigation validation at 256 | Target ≤ one 60Hz frame (about 16.67ms) | PROVISIONAL latency: P5C runtime instrument; hard model size remains 256 |
| Hidden work | No continuous decoration animation or unnecessary running Timer after close | FIXED new-code rule: verify lifecycle/rapid reversals and 100 cycles in P5/P6/P7; existing debt remains recorded |

No numeric budget is frozen by a debug measurement. Native-vs-Kit source details
and probe limits are in NATIVE_REUSE.md; measured values and failures belong in
the P0 report, with raw evidence retained under target.

## P2 software frame marker

Harness schema 2 adds `first_software_frame_ms`: elapsed from Rust main entry to
completion of a synchronous public `Window.take_snapshot()` with nontransparent
pixels after show. Native/Kit both execute it. It measures software render-buffer
readiness, including construction/show, and excludes OS loader time and actual
screen presentation. `first_render_callback_ms` remains absent if AfterRendering
is unsupported; the snapshot value is never substituted into that field.

P2 compares 1 and 100 real buttons, 30 paired Release processes per side, preserving
all raw samples. `private_bytes_1s` and `memory_sample_delta_bytes` supplement the
2s memory sample. Apply retained startup thresholds to this explicitly named frame
milestone only; actual presentation and long-run idle remain unverified. Deterministic
binary and source-declaration comparisons are separate. Results and remaining budget
limits are in implementation/kit-fluent-v1/P2.md. No existing threshold is raised.

P2 measured result: 30 paired processes per side for 1/100 buttons passed the
binary and named software-frame delta allowances. Binary increments were
42/43.5 KiB; 2s Private Bytes p50 increments were 52/260 KiB. Software-frame p50
deltas were -0.214/+0.481ms. Single-button memory was unchanged from 1s to 2s in
all samples; 100-button observations included small changes, so full steady-state
and idle acceptance remain unclaimed. See [P2 report](implementation/kit-fluent-v1/P2.md)
for exact raw data, environment and all unrun metrics.


## P3 paired foundation scenes

`python scripts/run_perf.py --profile release --scenes icons-100 segments-100 --samples 30`
compares 100 actual icon commands and 100 host-bound selected/unselected commands.
Both icon variants use the same embedded asset, 20px icon, 44×32 geometry and tooltip
text; the native reference uses a minimal Tooltip Text while Kit uses TooltipHost.
Selected native Button disables automatic toggling just like SegmentButton. The
window/features/font/DPI and sample rules above remain unchanged. No budget is
raised. P3 report separates successful collection from budget/steady-state claims.

## P6 bounded motion comparison

Release sources before/after P6 use byte-identical measurement Slint/Rust, the same
resolved external graph, Fluent/winit/software, 1200x500 and requested Segoe UI
Variable Text at 100%. Each count has one before and one after process: 200 raw
software-buffer samples (100 toggles pairs) and at least 60 seconds settled idle.
All samples are retained; this is not the 30-pair full P7 matrix.

| Toast count | Before p50 / p95 ms | After p50 / p95 ms | Idle one-core CPU before / after |
|---|---|---|---|
| 1 | 4.3748 / 6.9911 | 4.1576 / 6.3147 | 0% / 0% |
| 20 | 5.3667 / 7.0746 | 5.7165 / 6.4411 | 0% / 0% |

Both p95 increments meet the unchanged max(1 ms, 10% of A) allowance and idle
increments meet 0.2 percentage points. The 20-instance median rises slightly;
lower p95 in this small comparison is not a universal speedup claim. Rendering
notification is unsupported here, so actual presentation/redraw counts remain
NOT_RUN rather than zero. CPU uses two real GetProcessTimes readings over each
60-second interval with no active frame-driver timer during idle. No working-set
trimming or accessibility removal. The old P0 unmatched hidden-Toast binary
attribution remains separate; P6 does not close full P7 native/memory budgets.

Raw: target/motion-bench/20260909T185333555763Z/result.json and
target/motion-bench/20260909T190613605793Z/result.json; exact identities and allowance
calculation in target/p6-comparison.json and the staged P6 report.

Font scope of the P6 harness: Theme.ui_font_family is assigned, while Window uses
its unchanged default font resolution in both runs. No per-glyph font-family
verification or system-font redistribution is claimed.
