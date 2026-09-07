# Navigation rebuild Phase 7 — keyboard, accessibility and visual polish

Date: 2026-09-07. Starting commit: `6c54791685fef7f266cff3535f74fe0b41f0c5de`.

## Implementation

Left/Right request collapse/expansion without invoking a destination. Left on
a leaf or collapsed row focuses an enabled visible ancestor, falling back to
the pane focus scope. Tab/Shift+Tab remain the item traversal mechanism;
Up/Down/Home/End item traversal is explicitly not implemented. The host still
owns selection, expansion and pane mode; no internal adaptive state machine
or router was added.

Rows now recover focus on any transition to inactive, including dynamic disable.
IconButton clears focus and suppresses its ring when disabled, and explicitly
guards keyboard activation. Compact search transfers focus to the newly created
standard editor after the host accepts expansion. Back and compact search use
popup tooltips, like pane toggle and rows, to escape pane clipping.

Four non-interactive reference pages identify their specimens as Reference.
TextField/TextArea keyboard notes distinguish submission from newlines; an unused
TextArea counter is removed. Native window decorations and minimum 760x520 size
remain unchanged. Gallery continues to own its explicit pane mode.

An opt-in NavigationValidationWindow uses the public facade and existing Gallery
hierarchy fixtures, with counters, long branch labels and focus-state switches.
It is separate from the 25 catalog destinations and initializes its own globals.
The capture script records simulated DPI, source and binary identity, suite/schema,
actual PNG dimensions/hash and process success. It does not refresh baselines.

## Verification

Phase 7 local acceptance: **PASS** within the Windows coverage below.

### Common gate

All six commands exited 0, using the existing locked toolchain and dependencies:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS: 17 tests (1 Kit, 16 Gallery) |
| `python scripts/check_ui_boundaries.py` | PASS: 35 exports, 65 static files, 32 SVG assets |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS: 44 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS |

The first Clippy attempt found missing Markdown backticks in a Rust doc comment.
That comment was corrected, then the full common gate passed. Logs and command
results are in ignored `target/navigation-phase7/gate/`. Distribution package
closure is not implied by the boundary command's static-file count.

### Native input observations

Windows 11, Slint 1.17.1, winit-software, 100% scale. These are manual native
keyboard/pointer observations with UI Automation state and screenshots, not a
screen-reader certification or automated interaction suite.

| Flow | Observed result |
| --- | --- |
| Back, pane toggle, labels and independent chevrons | Pointer, Enter and Space produce the expected request counters; Tab/Shift+Tab reach separate controls. Disabled Back cannot invoke. |
| Focused Back disabled and re-enabled | Focus is cleared, the ring disappears, and re-enabling does not revive a stale ring. |
| Deep selected row disabled or hidden by ancestor collapse | Focus recovers to an enabled visible ancestor; selection remains host-owned. Re-enabling does not leave a second ring. |
| Left/Right and group/leaf semantics | Right expands; Left collapses or focuses an ancestor without invoking a destination. Group activation expands; destination activation invokes. |
| Compact search | Enter and Space expand through the host callback and transfer focus to the new standard editor. Editing and Enter submission remain observable. |
| Footer and optional-region removal | Removing a focused footer or search clears inaccessible focus; subsequent Tab reaches a visible control. |
| Minimum window and long list | At actual 760x520, Tab skips disabled rows and scrolls the offscreen iconless long-label row into view. The footer remains separate. |
| Compact tooltips and focus rings | Full ancestor text, Back and search tooltips escape the narrow pane; Light/Dark pane and content focus states were inspected. |
| Actual Gallery source panel | Pointer/Enter/Space open or close the panel; Tab enters the read-only editor; collapse returns focus to the toggle; Tab skips the hidden editor. |

Initial input work reproduced the stale disabled-focus/ring problem and the
missing search focus after compact expansion. Both were fixed and rechecked in
the final native run. Evidence is under ignored `target/navigation-phase7/native/`
and `native-final/`; final observations 01-22 cover focus/keyboard/tooltips,
27-34 cover the actual minimum-size validation window, and 35-46 cover the actual
minimum-size Gallery and source panel. Observation 23 was an unsuccessful resize
attempt at the default size and is not minimum-size evidence.

Native decorations were retained. The system menu and maximize were observed;
after the restore action the automation API reported a minimized window, and
reactivation returned to the normal size with state preserved. Alt+F4 closed both
hosts. This is not a complete custom-chrome or native-window-action test matrix.

Final native executable SHA-256:
`131e054135b74cdc813c77b3e328a6584bb64f3b2b710f78ff69bd014b02b36e`.
Its recorded source content SHA-256 was
`1171c42028a4f4644302d07c8a106254a9a210af092fa3cacdd36bb3c7defe59`.
The subsequent Rust doc-comment fix changed build identity but no executable
logic or Slint source; the rebuilt binary below passed the render matrix.

### Render matrix

`python scripts/capture_navigation_polish.py` exited 0: **184 / 184 PASS**.
Each scene used a fresh process and PNG, and its dimensions and hash were checked.
The complete manifest scene set and all image hashes were rechecked after the run.

- 20 real Gallery Home scenes: 1040x800 and 760x520, Light/Dark, five scales.
- 40 standard full-viewport navigation scenes: both sizes/themes/pane modes,
  simulated 100/125/150/200/225%.
- 112 optional-region, surface-mode and narrow-pane scenes: variants 1-7,
  both sizes/themes/pane modes, simulated 100/225%.
- 12 additional one-level, two-level and empty-model scenes at 1040x800, 100%,
  both themes and pane modes. The standard fixture supplies three levels,
  deep selection, disabled rows and iconless/long localized-like labels.

Representative Light/Dark, minimum/default, expanded/compact, 180/304 px pane,
hidden-region, empty-model and Fluent/Flat/Transparent images were visually
reviewed. Long labels elide before chevrons. Pane scrollbars remain distinct from
the content frame. Fluent has only a top-left content corner; Flat has square
corners, and Transparent leaves the host background visible. This is not a claim
that every pixel of all 184 images was manually reviewed. Cursor hover/tooltips
can appear in native captures; hashes identify artifacts, not golden baselines.

Render source content SHA-256:
`c42260cddad5e7008ab9351cc2ee0a53536296418affa2e7f44a5e14ce828645`.
Render binary SHA-256:
`df0c9ca7a60b874621fecdb2d5535084fbf6efadd362a579d3c8197be4652303`.
The source remained unchanged throughout all 184 captures. The recorded Git HEAD
is the starting Phase 6 commit with `dirty: true`, not a fabricated Phase 7 SHA.
Only this report and the validation index were finalized after capture.

Artifacts: ignored `target/navigation-phase7/render.log` and
`target/navigation-phase7/render/dirty-c42260cddad5e7008ab9351cc2ee0a53536296418affa2e7f44a5e14ce828645/`
(184 PNG/scene-manifest pairs plus `results.json`). They can be regenerated with
the committed script; no screenshot baseline or machine-private file is published.

## Scope and limits

No public API baseline, Cargo manifest/lockfile, toolchain, root Rust locator,
resources, attribution or licenses changed. No publication or Tasks changes.
NOT_RUN: real monitor DPI transitions, Linux/macOS native runtime, full backend
matrix, screen-reader action dispatch, IME and live OS theme transitions. Native
chrome is retained; custom chrome is deferred. Package/archive, MSRV and exclusive
incremental mutation checks belong to Phase 8 and are NOT_RUN in this phase.
Modal focus containment/restoration remains unclaimed. Phase 8 has not started.
