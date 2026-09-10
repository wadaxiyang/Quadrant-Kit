# Navigation header and caption Back follow-up

Superseded in compact layout and shared Back ownership by [NAVIGATION_COMPACT.md](NAVIGATION_COMPACT.md).

Based on P8 HEAD `5e3bff0bd20528816f085cf9e4930ef8867f579c`, retaining the two earlier
uncommitted follow-ups. This report supersedes their toolbar layout only.
No dependency, public declaration, native system action, Tasks or Git publication change.

The pane toggle and name now share NavigationView's 44px header above search.
Compact mode hides the name; the toggle remains reachable. Header actions remain
native. The toolbar contains only caption Back. Windows supplies its width, height
and top offset from WM_GETTITLEBARINFOEX's actual minimize rectangle, converted
from screen physical coordinates to Slint logical client coordinates.

The explicit borderless-caption request requires a narrow Gallery-only exception:
public Button has no flat style. CaptionBack uses public FocusScope/TouchArea,
without a hidden native proxy, Kit private imports, timers or extra accessibility
nodes. Pointer activation, Space/Return release (repeat suppression), cancellation,
disabled state and the accessible default action request existing host navigation.
Idle/pointer appearance has no frame; keyboard focus remains visible. The Kit
NavigationBackButton implementation is unchanged. The host exception is noted in
native_reuse_manifest separately from its 45 guarded Kit records.

## Validation

- PASS: fmt, Clippy (all targets/features, warnings denied), 21 Rust tests,
  88 Python tests, UI/native guards and Gallery build. Logs/source fingerprint:
  `target/verify-navigation-header/result.json` and `0.log` through `6.log`.
- PASS: 18 actual toolbar/Settings runtime assertions (nine preserved Settings
  checks and nine caption Back checks). `target/button-checks/20260910T080611572169Z/`.
- PASS: Windows UIA/default action, native maximize/restore, caption/client hit
  regions and foreground-guarded pane folding, plus actual window images:
  `target/navigation-header-window/result.json`. The permanent probe now contains
  the geometry/capture checks previously assembled under target/.
  Its final rerun also passed: `target/navigation-header-final/result.json`.
- At OS DPI 100% / Slint scale 1, Back is 47x30 physical px restored and 47x29
  maximized. It exactly matches the native minimize rectangle; vertical center
  delta from all three system buttons is 0 in both states. Windows' native widths
  themselves vary: restored min/max/close 47/46/47; maximized 47/46/48. We preserve
  these OS rectangles rather than claim that Windows makes all three identical.
- Gallery binary SHA256:
  `16ede890b55447a6204d9ec1ec70c66da05668abcb7d20d0de06939dfc2e2eb6`.
- Visual inspection PASS: restored, maximized, Settings and compact window layout;
  caption Back is borderless at rest and the pane toggle stays in the nav header.

Full screen-reader behavior, native macOS, real monitor DPI
changes and new performance measurements are NOT_RUN for this layout follow-up.
Packaging retains the existing uncommitted-source prerequisite; no commit is made
to bypass it.
