# P5B — finite native-action confirmation

Base: `69bbab67f70bec3684f3b4f70b73cac6bb30bf31` (P5A); both package commands
passed after that commit. One confirmation overlay only; no arbitrary content,
new modal runtime, navigation rewrite, push, tag or external consumer change.

## Reproducer and current contract

`target/button-checks/20260909T173805230357Z/result.json`: FAIL. Screenshot shows Cancel has the native focus ring after Tab;
Return emits accepted=1, dismissed=0 because the old parent capture handler
unconditionally accepted. That obsolete Return handler is removed entirely.

ModalManager now conditionally instantiates its fixed native actions/focus scope.
Initial focus is Cancel with show_secondary, otherwise primary. Native buttons
own Return/Space. The only custom keyboard rules are plain Tab/ShiftTab cycling
among the one/two actions and Escape dismissal. A passive scrim blocks background
pointer input. At most one accepted/dismissed request is admitted per shown cycle;
host owns closing. Ignoring a request retains focus in the actions without issuing
another command. No Timer or Kit animation was added. Hosts keep show_secondary
stable while shown and coordinate programmatic focus/page changes themselves.

New callback restore_focus_requested() fires once on logical close after observed
open, including programmatic close, with no initial-hidden callback. Host restores
its known opener or fallback; Kit cannot store arbitrary external focus elements.
Gallery feedback/modal pages carry an epoch to the actual three opener buttons,
which remember which button opened the confirmation. This is host demonstration
state, not a required Kit adapter, global service or business model.

Breaking behavior: Return no longer always accepts. Callers choosing Confirm
navigate/focus that native action; the existing button/composition suite now Tabs
from initial Cancel to primary before its preserved acceptance assertion. The
first run of the old test against the new initial-focus protocol also failed
because its Tab now correctly focused Confirm; it remains in target evidence.

The reviewed API changes exactly one member on ModalManager (restore callback),
with identical names, other members and defaults across all 53 exports. Current
snapshot/probe/docs and Gallery use sites are synchronized; no compatibility path.
Native manifest review covers only fixed focus coordination, scrim and single
request/restore lifetime. No ordinary activation duplication or hidden proxy.

## Evidence

`target/button-checks/20260909T174214125052Z/result.json`: PASS, 20 WindowEvent assertions. Initial Cancel, Return/Space, repeated
Tab and ShiftTab, single-action focus, Escape, ignored duplicate/alternate requests,
shown scrim, hidden pointer/keyboard, eight initial logical closes plus ten rapid
open/close pairs all pass. Restore callbacks total exactly 18. Host opener focus is
observed after close; no direct Kit accepted/dismissed invocation.

`target/button-checks/20260909T174304816689Z/result.json`: PASS, 20 preserved button/composition assertions, with the current
explicit focus navigation before Confirm. Runtime Light/Dark screenshots show
initial Cancel, primary, single action and restored opener. Manually inspected
initial Cancel/single action/closed final. Reader evidence is not inferred from rings.

`target/verify-p5b/result.json`: six commands PASS: fmt, boundary/API/assets,
native reuse, 75 Python tests, Gallery build and 40 modal-page Matrix captures.
Stable source `2a53a82fb96b5d0dcca253c2645d10b0ae03858ed40af60f2f411e2cece3e65d`. Full Clippy/20 Rust tests
were last run at P4D, NOT_RUN again in this scoped Slint subbatch; full P5 core
reconciliation follows P5E. Packaging runs after the authorized commit.

Windows 26200, Slint 1.17.1 Fluent/winit/software, requested Segoe UI Variable Text.
Runtime is 700x520/100%; Gallery closed-page examples cover Light/Dark, four sizes,
simulated 100/125/150/200/225%. Gallery host routing restoration is compiled and
wired; automated runtime assertions exercise the equivalent explicit opener
callback in the isolated host, not OS-level automation of every Gallery opener.
Native OS reader containment, IME, all monitor/DPI and general focus restoration
are NOT_RUN. Release CPU/frame performance is NOT_RUN in this unit; no invented
numbers or claim of a complete ContentDialog. P6 measures bounded transient motion.

Delivery: finite input/host-focus contract PASS. Overall capability remains PARTIAL
for native accessibility/full dialog semantics. This scoped unit is complete;
commit its report before the authorized P5C continuation.
