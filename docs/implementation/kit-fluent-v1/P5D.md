# P5D — native flyout and command/menu composition

Base: `701fc1f309bab7bad0a5ccc56f88482037e402da` (P5C), with both source package
commands PASS. P5D adds three static exports in overlays, no menu runtime service,
no cross-layer dependency and no consumer cutover.

## Capability first, current contract

`target/button-checks/20260909T182508937732Z/result.json`: pure native PopupWindow/Button/Menu baseline PASS. The first
menu run (`20260909T182347598953Z`) timed out because Windows native Menu runs an
OS modal loop, outside Slint WindowEvent dispatch. The test now sends Down/Return
from a bounded helper thread only after confirming its own process is foreground;
OS_MENU_INPUT=OWN_PROCESS is recorded. Native keyboard menu action then passes.
No Kit implementation of menu input is introduced. Outside Windows that OS subset
is not claimed; native readers and complete menu accessibility remain NOT_RUN.

FluentFlyout inherits actual public PopupWindow. Its surface/vertical content slot
adds only local tokens; inherited show()/close()/is-open, Escape, outside-click
policy and focus restoration remain native. Hosts focus a known opener before
show and use Tab/native child actions. Content handles its own enabled state,
scrolling and close actions. There is no synthetic host-controlled shown mirror.

FluentDropDownButton opens an anchored flyout; FluentSplitButton has separate
native primary and secondary owners. Primary alone emits clicked. Secondary
opens the popup. Both expose actual is_open, open()/close() and native focus.
Own-root disable/hide closes the popup; retained ancestor hiding/page replacement
requires explicit host close or unloading. Menu and ContextMenuArea stay native
Gallery examples, including disabled items and submenu declarations.

Native text-symbol arrows were absent in the software snapshot; existing local
chevron SVGs with native colorize-icon replace them. DropDown uses the native icon
slot before text; Split has an icon-only secondary owner. No invisible proxy.
Gallery compilation caught redundant min-width plus explicit width on Split;
the redundant root minimum was removed. Runtime originally overrode width and did
not expose that default-layout error: both default probe and Gallery now compile.
The padding property collided with an inherited builtin; it is content_padding.
Failed capability builds remain in target (`182546...`, `182611...`).

## Evidence and scope

`target/button-checks/20260909T183033855133Z/result.json`: PASS, 23 runtime assertions, Release, winit/software.
Includes actual popup state, native Return/Space/Tab/Escape, inside/outside policy,
focus restoration, disabled controls, split regions, ten rapid popup cycles,
edge placement and native ScrollView inside flyout. Native Windows menu command
passes its own foreground-guarded input check. Current test source uses only the
current facade; native baseline copies live in ignored target, not frozen fixtures.

`target/button-checks/20260909T183033855133Z/scales.json`: all four additional runs PASS at 125/150/200/225%,
with every runtime assertion repeated. Combined 100% plus four scaled runs: 115
assertions. These are simulated scale factors, not physical-monitor transitions.
Inspected Light edge placement and Dark scrolled content: native placement keeps
the bounded popup inside the 700x500 parent, and scrolling reaches rows 27–29.
A software snapshot does not capture the Windows OS menu surface itself.

`target/verify-p5d/result.json`: six checks PASS: fmt, boundaries/current API,
native guard, 76 Python tests, Gallery build and 40 native-popups Matrix captures.
Stable source `39714ed6a98283594a0a0219a7a689c7ec7d15cbc0e45d4dcdd61b0f90f9c8bb`. Exactly three new names
were reviewed/adopted; previous 53 signatures/defaults unchanged. There are now
56 exports / 40 visual components; catalog/probe/status/native manifest align.
A scanner fixture checks PopupWindow child composition and rejects custom input;
ContextMenuArea/Menu children remain fully scanned. Full Clippy/20 Rust tests
last passed P5C and are scheduled again for P5E. Package checks follow this commit.

No real-present frame timing, paired memory/binary budget, native OS reader,
all menu shortcuts/submenu input or cross-platform runtime is claimed. P5D scoped
contract PASS; these capability limits remain explicit. Next authorized unit P5E.
