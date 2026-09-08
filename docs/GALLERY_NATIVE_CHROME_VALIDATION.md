# Gallery native window chrome

Follow-up to the user's request to share application toolbar controls while
retaining platform-native window controls. Starting source:
`7a4e447e002ec12c66d2215ca1e89b1df4499faa`.

## Ownership

`GalleryToolbar` contains Back, pane toggle, title, theme and preview controls.
It accepts native leading/trailing exclusion insets and exposes a caption input
callback. It draws no window-control glyphs and owns no minimize/maximize/close
state. Gallery disables NavigationView's internal operation row to avoid duplication.
Kit's public facade, dependency boundary and WindowControlButton specimen are unchanged.

`gallery/src/window_chrome.rs` configures the pinned winit backend before window
creation. Platform code lives in `window_chrome/windows.rs` and `macos.rs`.
Native decorations remain enabled. Each adapter is scoped to the main Gallery
window, holds weak winit references, and keeps its native measurements alive
only for the window lifetime. The navigation validation window remains independent.

Windows keeps WS_CAPTION and the system caption buttons. WM_NCCALCSIZE extends
the client directly, without first calling the default caption calculation;
maximized client geometry uses the monitor work area. DWM receives non-client
messages first, including NCMOUSELEAVE. Remaining hit tests use DPI-scaled resize
borders and the actual toolbar title bounds. The Slint surface leaves the native
button area transparent; opaque Slint clears otherwise cover DWM rendering.
The adapter requests DWM frame composition, synchronizes native caption colors,
and converts measured button bounds from window to client to Slint coordinates.
Only the application title region starts caption dragging. If DWM declines a
maximized-button hit, the host queries WM_GETTITLEBARINFOEX and returns the native
hit code for the OS-reported rectangle. No button widths or glyphs are emulated.
Caption right-click explicitly invokes winit's native system menu because the
extended client suppresses the default caption context-menu path.
System button rendering/actions are preserved; complete accessibility coverage
is not claimed.

macOS requests a transparent titlebar and full-size content view, retaining the
real AppKit traffic lights. It measures visible native button rectangles in the
content view and converts AppKit points through the native and Slint scale factors.
AppKit appearance follows the Gallery theme. Caption input uses AppKit dragging and zoom/minimize actions. Known double-click
preferences Minimize/None are honored; other values use native zoom. New OS-specific
preference values and full-screen toolbar transitions require native verification.
Linux continues to use its normal decorated window and one application toolbar.

No dependency versions are upgraded. Gallery adds direct macOS edges to the already
locked objc2 0.6.4, objc2-app-kit 0.3.2 and objc2-foundation 0.3.2, and Windows API
features on the already locked windows-sys 0.61.2. No Kit runtime dependency is added.

## Verification

Local verification on Windows 11 build 26200, 2026-09-08:

| Check | Result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS |
| `cargo test --workspace --locked` | PASS: 20 unit tests, no failures |
| `python scripts/check_ui_boundaries.py` | PASS, resolved Cargo checked; public API unchanged |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS: 46 tests |
| `cargo build --locked -p quadrant-kit-gallery` | PASS |
| Apple Silicon `cargo check --locked -p quadrant-kit-gallery --target aarch64-apple-darwin --target-dir target/macos-native-check` | PASS, type checking only |
| Apple Silicon `cargo clippy --locked -p quadrant-kit-gallery --all-targets --target aarch64-apple-darwin --target-dir target/macos-native-check -- -D warnings` | PASS |
| `python scripts/capture_gallery_baseline.py --mode All --output-directory target/native-chrome/scenes` | PASS: 41 software-rendered scenes; all physical dimensions match requested logical size and scale |

The scene matrix covers light/dark, 760x520, 900x600, 1100x720, 1440x900 and
100/125/150/200/225% simulated scale, plus the 1040x800 smoke scene. These are
capture/dimension checks, not proof of native DPI transitions or a pixel-perfect
review of every scene. Desktop checks confirmed light/dark composition with both
the default renderer selection and explicit `winit-software`.

Native input checks passed: title dragging and double-click maximize/restore;
native maximize/restore buttons (including maximized hit testing), minimize and
reactivation, close, caption right-click system menu, and bottom-right resize to
the exact 760x520 minimum. Initial size is 1040x800; maximized content matches the
1920x1032 monitor work area. Pane toggle, component navigation and Back to Home
were exercised with the integrated toolbar. Runtime stderr was empty.

Local logs and desktop evidence are under ignored `target/native-chrome/`, including
`native-gallery-light.jpg`, `native-gallery-dark.jpg`, `native-system-menu.jpg`,
`native-maximized.jpg` and `native-minimum.jpg`. Build SHA-256:
`e58b3b36e3c306ab4069d2326e6a4ba64a8a637c10c356b80b3f811b7666461c`.
The scene manifests identify the pre-report working source by content SHA-256
`f283a1eeb2e3825c90f2aa6467922c5bd299a37cf1cd9042176ef77691d2f0d8`;
only documentation changes follow that capture. Earlier
[hand-drawn Windows caption evidence](GALLERY_TITLE_BAR_VALIDATION.md) is historical.

Windows multi-monitor/real DPI transitions, Windows 10, Snap hover, complete native
accessibility, and other renderer-specific native behavior: **NOT_RUN**.
This correction does not rerun MSRV, distribution/package or destructive incremental
checks: **NOT_RUN**; earlier phase reports retain their own scope.

macOS native build/link/run, actual Retina/multi-monitor transitions, full-screen
Spaces and macOS accessibility: **NOT_RUN**, no Mac host available locally.
Publication and remote CI: **NOT_RUN**, not authorized by this correction.

## Native contracts

- [DWM custom frame](https://learn.microsoft.com/en-us/windows/win32/dwm/customframe)
- [DWM non-client hit testing](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmdefwindowproc)
- [AppKit transparent titlebar](https://developer.apple.com/documentation/appkit/nswindow/titlebarappearstransparent)
- [winit macOS window attributes](https://docs.rs/winit/0.30.13/winit/platform/macos/trait.WindowAttributesExtMacOS.html)
