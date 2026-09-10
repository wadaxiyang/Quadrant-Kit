# Flat navigation and Gallery Settings follow-up

The subsequent [title-bar alignment](TITLEBAR_ALIGNMENT.md) supersedes this
report's toolbar height and name placement; its earlier measurements stay scoped
to the sources recorded here.

User-requested follow-up to P8, based on `5e3bff0bd20528816f085cf9e4930ef8867f579c`.
The initial checkout was clean. No dependency/backend/window-shell upgrade,
public API declaration change, Tasks edit, commit or publication is included.

## Reference and implementation

The old rows composed normal public Slint Buttons for the command and chevron,
including their persistent border, checked accent fill and focus rectangle.
The public pinned Button has no flat/background/border styling input.
Reference review used Microsoft's [NavigationView guidance](https://learn.microsoft.com/en-us/windows/apps/design/controls/navigationview),
[Gallery MainWindow](https://raw.githubusercontent.com/microsoft/WinUI-Gallery/main/WinUIGallery/MainWindow.xaml)
and [Gallery Settings page](https://raw.githubusercontent.com/microsoft/WinUI-Gallery/main/WinUIGallery/Pages/SettingsPage.xaml).
The design adopts flat rows, a small left selection indicator and an Appearance
theme selector inside Settings; this is not a pixel-equivalence certification.

NavigationItemRow now composes a private NavigationRowTarget for each independent
command/chevron region. The narrowly documented exception uses public TouchArea
capture and FocusScope keyboard/accessibility behavior. It has no hidden native
proxy, imported private Slint implementation or general-purpose Button export.
Pointer selection has no persistent frame. Keyboard focus remains visible.
Release-based Enter/Space suppresses held-key repeats and cancels on Escape,
focus loss or disable. Direction keys, model validation, controlled selection,
tooltip service and native scrolling remain intact. Row visuals have no animation.
The native guard remains fail-closed; 45 records cover 42 public visual components
and three private helpers, with seven scoped reviewed exceptions.

Gallery has 33 destinations / 40 catalog entries. Settings is a fixed footer
destination and participates in search and Back history. App theme and Preview
width edit window-owned state through native ComboBoxes. Page recreation retains
values; programmatic updates remain reactive after user input. The toolbar now
contains Back, pane toggle and title, plus the existing OS caption exclusion areas.

## Validation

PASS — requested navigation/settings implementation and scoped verification.

| Category | Result and evidence |
|---|---|
| Input | PASS: 160 navigation WindowEvent assertions, including all 152 earlier assertions and eight new gesture/focus checks; `target/button-checks/20260910T065702826489Z/`. Settings has nine assertions against the actual page/toolbar, `target/button-checks/20260910T065744783031Z/`. |
| Core | PASS: format, Gallery build, 21 Rust tests, boundaries/current API/native guard and 88 Python tests; `target/verify-nav-settings-final/`. Clippy all targets/features with warnings denied passed in `target/verify-nav-settings/1.log`; subsequent Rust edits only corrected the new search test's catalog-order assumption. |
| Windows UIA | PASS: `target/gallery-settings-uia-current/result.json`; unique focusable Settings target, published host selection, navigation activation, named native ComboBoxes, Home/Back history and absence of the old toolbar actions. Checked navigation buttons expose TogglePattern; native Back exposes InvokePattern. This is not reader or WinUI UIA parity certification. |
| Rendering | PASS: 80 fresh Home/Settings captures, four sizes (760x520, 900x600, 1100x720, 1440x900), Light/Dark and 100/125/150/200/225% simulated scale; `target/nav-settings-visual/result.json`. Visual inspection covered narrow Light/Dark, narrow 225%, wide Dark, and the navigation gesture snapshots. |
| Actual window composition | PASS: own-process PrintWindow Home/Settings captures in `target/gallery-settings-uia-pass/`, including the actual Windows caption area. Software snapshots have the previously documented black/washed-out transparent caption artifact and are used only for client content review. |
| Bounded model cost | PASS within the prior provisional 16.67 ms validation target: 30 samples per 16/64/256/257; current 256-entry validation p95 8.5337 ms, min 7.148 ms, max 8.795 ms. Raw BENCH lines remain in the navigation runtime log. No whole-Gallery memory/presentation performance claim is added. |
| Distribution | Static closure PASS: 87 files / 32 SVGs. Package-list FAIL on uncommitted source; clean-source package acceptance BLOCKED until an authorized commit. No dirty override or automatic commit was used. |

The Gallery build/UIA/capture executable SHA-256 is
`a091e2a72592ac5b0c5a6405b2bacdf904221bad54e270aa57eefc22969e32ab`.
The capture source content digest is
`2e40b5e0201554d5fd9df9229b102c3f323ccc4193f82e631c8f6f8793fed1d4`;
later changes only finish verification scripts and documentation. Individual runtime
and core reports retain their own exact dirty-source digests. Environment:
Windows 11 build 26200, pinned Slint 1.17.1, winit/software, Segoe UI Variable Text.

Initial navigation runtime retained one failing compact hit-region assertion;
explicit left anchoring fixes its default centering. Initial settings runtime
retained two failing post-input programmatic-binding assertions; explicit theme
synchronization and two-way preview state fix them. Existing assertions were retained.
The new controller test initially assumed a unique "settings" search result, but
existing component keywords also match; it now checks discoverability and uses the
unique "appearance" keyword for Enter/history. UIA development runs retain an
Invoke-only unsupported-pattern result, one startup tree discovery miss and a
PowerShell variable-name error; the final checked-pattern probe passes. Raw failures
remain under `target/button-checks/`, `target/verify-nav-settings/` and the earlier
`target/gallery-settings-uia*` directories.

Actual screen-reader support and pixel-identical WinUI runtime comparison are
NOT_RUN. Earlier P7/P8 platform, actual presentation and long-term memory gaps
remain unchanged. This follow-up does not mark the entire SPEC accepted.
