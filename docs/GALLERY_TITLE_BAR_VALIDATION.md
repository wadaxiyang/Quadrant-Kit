# Gallery unified Windows title bar

Historical report for the hand-drawn caption implementation. The current native
window adapters supersede it; see [native chrome validation](GALLERY_NATIVE_CHROME_VALIDATION.md).

Authority: the user's post-Phase-8 request to make the navigation operation row,
Gallery toolbar and Windows title bar one visual/interactive row.
Starting commit: `5155f7ea62a8b90719cbcbac4113fecc3c2530e5`.

The main Windows Gallery now owns a single 48 px title bar. Back and pane toggle
reuse public Kit components; the decorative menu image and internal navigation
operation row are removed. Actual window controls reuse WindowControlButton.
Native caption hit testing uses the pinned winit window accessor and Win32;
window state and edge resize use Slint's existing window APIs. No public Kit API change or
toolchain/dependency version upgrade is introduced.

Gallery alone adds Slint's `unstable-winit-030` feature and a Windows-only direct
dependency on the already locked windows-sys 0.61.2. Cargo.lock adds two dependency
edges, with no package additions or version changes. Other platforms and the
separate validation host retain native decorations. This change does not claim
native Windows caption-button rendering or hover Snap Layout support.

After awaiting the actual winit window, Gallery installs a scoped Win32 subclass.
It returns HTCAPTION only for the Slint title text/empty-space bounds, excluding
buttons, content and the normal window's resize edge. Windows then owns caption
dragging and double-click. Caption right-button release explicitly opens winit's
standard system menu because the separate native frame is absent. Signed screen coordinates
are converted to client coordinates and divided by the live Slint scale factor.
The FFI is isolated in the Windows module with safety comments. A guard owns
stable callback data, unregisters before freeing it, and tracks WM_NCDESTROY;
other host code retains `deny(unsafe_code)`. Initialization failure exits with an
error. No Cargo registry source or dependency is patched. Unit tests cover
negative monitor positions and hit-test exclusions.
The menu bridge holds a weak native-window reference so Slint can hide/destroy
the window normally; it upgrades that reference only while opening the menu.

Native contracts: [WM_NCHITTEST](https://learn.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest)
and [SetWindowSubclass](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass).

## Verification

Verified locally on Windows on 2026-09-08. Logs and captures are under ignored
`target/titlebar-correction/`; earlier Phase 8 results remain historical.

| Check | Result | Evidence |
|---|---|---|
| `cargo fmt --all --check` | PASS | `fmt.log` |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | PASS | `clippy-clean.log` |
| `cargo test --workspace --locked` | PASS; 1 helper + 18 Gallery tests | `tests-clean.log` |
| `python scripts/check_ui_boundaries.py` | PASS; 35 exports, 65 static files, 32 SVGs, resolved Cargo | `boundaries.log` |
| `python -m unittest discover -s scripts/tests -p "test_*.py"` | PASS; 46 tests | `python-tests.log` |
| `cargo build --locked -p quadrant-kit-gallery` | PASS; Gallery and API probe | `build-clean.log` |
| `cargo +1.92.0 build --locked -p quadrant-kit -p quadrant-kit-gallery --target-dir target/msrv-1.92` | PASS | `msrv-clean.log` |
| Home `--mode All` capture | PASS; 41 scenes, checked physical dimensions and PNG hashes | `render-final.log`, `render-audit.json` |

The final build/render input has content SHA-256
`c8098387f264d5967a77eb1ebd69c267c322a60892457b700287da04fe8feb3d`;
the native Gallery executable has SHA-256
`76426f7cbd9b128a9dd6dc34e70234c4a674bf1c7ab6353ca8b0d33d47d02571`.
The checkout was dirty relative to the starting commit while testing; subsequent
changes only complete this evidence report. Each scene/native session records
its actual source and binary identity rather than attributing it to a fake revision.

Native input observations during this correction:

- One 48 px top row; Back/pane toggle align with the title and utilities. No
  decorative hamburger, extra native caption or internal navigation operation row.
- Caption drag moves the real window; final replay moved `(156,156)` to
  `(246,232)` for a `(90,76)` drag. Native double-click maximizes to the monitor
  work area (1920x1032 here) and restores the prior size/position.
- Actual maximize/restore, minimize/recover and close buttons work. Corner resize
  reaches 760x520; all title-bar controls remain visible in Light and Dark.
- Right-click exposes the standard Windows system menu. Shift+Tab reaches Close
  and Maximize; Enter maximizes, Space restores, and focus remains on that control.
- Pane toggle changes the shell, compact search expands and focuses its editor,
  All components enables Back, and Back returns Home and clears disabled focus.
- The final weak-reference adjustment was replayed through native drag, right-click
  and system-menu Close: exit code 0 with an empty runtime log.

These observations are cumulative: `native-caption/` covers native double-click,
pointer window buttons, minimum-size resize and Dark; `native-delivery/` covers
menu, keyboard and navigation; `native-clean/` verifies the final lifetime change.
Their source manifests distinguish intermediate binaries from the final one.

Final render coverage is 1040x800 Light/100%, plus 760x520, 900x600, 1100x720 and
1440x900 in both themes at simulated 100/125/150/200/225%. Representative default
and minimum/high-scale captures were visually inspected. These are renderer
scale-factor fixtures, not physical monitor/DPI-transition tests.

## Scope limits

Non-Windows runtime/CI, physical multi-monitor DPI transitions, other renderers,
full screen-reader/IME coverage, package/archive and incremental rebuild checks
are NOT_RUN for this host correction. Kit sources, API/asset baselines and Tasks
are unchanged. Native DWM caption-button rendering and hover Snap Layout UI are
not implemented by this Gallery composition. No publication or push is performed.
