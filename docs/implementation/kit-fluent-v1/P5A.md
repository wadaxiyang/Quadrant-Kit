# P5A — native tooltip service and finite Toast lifecycle

Base: `f93cdac96cbeedfc738bf2ca711cb0b9a9868e42` (completed P4). Both package checks
passed after P4D commit. This unit precedes Modal/navigation work; no P5B changes.
Original root SPEC changes remain excluded. No push, tag or consumer change.

## Reproducer and fix

Old implementation: `target/button-checks/20260909T172409921989Z/result.json` is FAIL against the current intended lifecycle.
Keeping shown=true produced two dismissed callbacks in nine seconds. The old
hidden pointer/Tab and focus tests passed; those were not falsely relabeled as
reproduced bugs. Later old-run count assertions cascade from the duplicate event.

ToastHost now admits one dismissed request per observed false-to-true shown cycle,
shared by automatic timeout and the native close button. It sets the guard before
emitting; host retains shown ownership. Ignoring the request stops the timer and
disables close until a new cycle. Message changes do not reset the cycle. Hiding
removes native input/Tooltip and the Timer subtree. Passive hover pauses timeout;
leaving restarts a full four seconds. Retained hidden ancestors need host shown=false
or subtree unloading. Coalesced false/true writes within one event are not a new
observed cycle. No focus acquisition or custom close activation.

The no-animation behavior baseline removes Toast height/opacity animations, fixes
height at 56px and aligns the close command to the right. Long text retains the
bounded two-line/elision contract. Native button animation is unchanged. P6 later
adds only bounded Kit motion. No API signature/default change; snapshot remains
unchanged at 53 public names / 37 visuals.

TooltipHost remains passive content; native Tooltip owns hover delay, placement
and dismissal. Gallery now demonstrates that service separately from a passive
presenter and actually handles Toast dismissal/reopen. Tooltip preferred geometry
includes its padding. Native tooltip placement can exceed a containing window;
the embedded software snapshot crops edge content. Pinned i-slint-core window.rs
explicitly retains that placement scope. A centered popup renders its whole text.
No keyboard/reader or work-area clamping promise is inferred from screenshots.

## Checks and evidence

`target/button-checks/20260909T173033799990Z/result.json`: PASS, 11 public WindowEvent assertions over real timeouts. Nine seconds
now yields exactly one request. Host state, passive hover, native manual close,
fresh cycles, ten rapid show/hide pairs, pointer/Tab behavior when hidden and
non-stealing native tooltip hover all pass. Runtime images retain visible/hidden,
Dark requested-close, edge/inside tooltip and pointer-leave states. Manual review
confirms right-aligned close and full inside tooltip; edge crop remains recorded.
Intermediate visual adjustment runs also passed input; no failed history deleted.

`target/verify-p5a/result.json`: seven commands PASS: fmt, boundaries/native,
75 Python tests, Gallery build, 40 Toast Matrix captures and one Tooltip Smoke
capture. Stable source `465e6009c03ebca89ebd570811312276e309056fd5572b0945532185410f4910`. Full Clippy and
20 Rust tests were last run in P4D; NOT_RUN again in this scoped Slint behavior
subbatch, with full P5 reconciliation due after the remaining units. Source package
checks follow this authorized commit. Current API/probe/manifest/Gallery agree.

Final image review corrected one specimen caption from content-driven height to
fixed-height elision; this text-only followup was checked by the current static
suite and does not change control geometry. The captured source identity above
precedes that caption correction.

Environment: Windows 26200, Slint 1.17.1 Fluent, winit/software, requested Segoe UI
Variable Text. Gallery Matrix covers Light/Dark, four sizes and simulated
100/125/150/200/225%; runtime is 760x480/100%. Native OS input, reader/IME and real
monitor transitions are NOT_RUN. No new release performance/60s idle measurement
in this unit; P0's unmatched hidden-toast binary result is not erased or claimed
resolved. Timer eligibility/subtree removal and actual event counts are evidence
of the finite protocol, not a fabricated CPU/frame-time result; P6/P7 measure it.

Gate: PASS for the scoped native tooltip/Toast lifecycle. Commit before P5B.
