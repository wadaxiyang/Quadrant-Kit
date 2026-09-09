# P5E — controlled Expander and inline InfoBar

Base: `ddbffbc54ed8d4c7501f12a981293824cad30575` (P5D), both package commands PASS.
Two visual components and InfoBarKind are statically exported from patterns/page.
Existing 56 export declarations/defaults remain unchanged. No runtime adapter,
new focus service, notification service or patterns/overlays dependency.

## Capability and contracts

Native composition probe `target/button-checks/20260909T183714920490Z/result.json`
FAIL: forcibly focusing a disabled public Button can still reach its native key
handler. The current Expander guards its request callback with header_enabled and
own visibility. The same edge is fixed in P5D SplitButton's primary callback and
covered by a new preserved popup regression. No keyboard activation is reimplemented.

`20260909T183822510092Z` compile FAIL: Slint 1.17.1 rejects @children inside a
conditional element. Expander therefore requires the host to provide an explicit
`if expanded: Content { ... }` slot using the same controlled expanded value.
Hosts include page activity when retaining hidden pages. The component controls
only header presentation, content visibility/height and expansion requests; it
cannot unload arbitrary unconditional caller content. This limitation is explicit
in PUBLIC_API, CONSUMER_GUIDE, Gallery and the current probe. No unsupported child
factory, copied implementation or hidden timer is used to pretend otherwise.

Native header activation focuses the header before expanded_requested(bool).
Programmatic collapse with content focus requires host focus_header() or a known
fallback when the header is disabled. header_enabled names its exact scope;
children own their enabled state. Host acceptance alone changes expanded.

InfoBar is an inline status with info/success/warning/error, wrapping title/message,
optional native close and no automatic dismissal timer. Host owns shown. One
dismissed request is admitted per shown cycle; ignored requests disable the close
command until a new cycle. Hidden body/input is conditionally unmounted. It is
not an OS notification or a verified reader live region.

## Evidence

`target/button-checks/20260909T183916344927Z/result.json`: PASS, Release, 19 runtime assertions: initial/collapsed mounting,
controlled requests, native pointer/Return/Space, disabled header, explicit focus,
child command/timer teardown and remount, hidden input, nonclosable InfoBar and
once-per-shown-cycle dismissal. Expanded/nonclosable/closed screenshots inspected.
`target/button-checks/20260909T184055971833Z/result.json`: PASS, 23 popup assertions including forced disabled Split focus
plus Space, native Windows menu input subset and scrolled popup close restoration.

`target/verify-p5e/result.json`: eight commands PASS: fmt, full Clippy, 20 Rust
tests, boundaries/API/assets, native guard, 76 Python tests, Gallery build and
40 native-inline Matrix captures. Stable source `80a7f13fe787b7b4389f69ad9373d7899b710098eb488f028b5109c3851e33e8`.
Current facade has 59 names / 42 visuals, with reachable catalog coverage. Current
docs remove stale opening counts, preserving historical reports and retained
consumer revisions. Gallery's initial placeholder spelling error was corrected;
`target/p5e-gallery-first-failure.log` retains that observed build failure.

Windows 26200, Slint 1.17.1 Fluent/winit/software; runtime 700x500/100%; Gallery
Light/Dark four sizes, simulated 100/125/150/200/225%. Reader/live-region, physical
DPI transitions, full cross-platform input and new 60-second CPU/frame measurements
are NOT_RUN here. P6 records before/after transient motion separately. Packaging
follows this authorized commit. Scoped P5E PASS under the documented host-conditional
slot protocol; P5 implementation units complete, full-dialog/reader limits remain.
