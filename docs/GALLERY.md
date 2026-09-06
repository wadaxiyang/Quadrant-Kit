# Gallery

The Gallery is a development, verification, and learning application with no Product dependencies. Build it with `cargo build --locked -p quadrant-kit-gallery`; run with `cargo run --locked -p quadrant-kit-gallery`. It opens DesignGalleryWindow. The compiled KitApiProbe is not shown during normal startup.

## Pages and controls

| Page | Content |
|---:|---|
| 0 | Overview: 28-name catalog and public import example |
| 1 | Tokens: generic theme, spacing, elevation, motion |
| 2 | Typography: generic text roles and content boundaries |
| 3 | Icons: 32 generic SVGs and action-button states |
| 4 | Controls: buttons, segments, input wrappers, settings, badges |
| 5 | Surfaces: decorative/interactive cards and variants |
| 6 | Feedback: Toast and single confirmation Modal |
| 7 | Navigation: generic sidebar, page framing, metrics, empty state, window controls |

The old page 8 and Task patterns filter are removed. Inbox components and
Product window probes belong to Tasks after cutover. Kit has no Inbox models,
product brand, task navigation aliases, or quadrant colors.

Use Light / Dark / System, Compact / Medium / Wide, and the per-page live settings. Existing preview properties remain specimen inputs; they do not assert that real pointer, focus, IME, or assistive-technology paths have been tested. The modal specimen no longer claims a complete Tab trap.

## Theme host

Rust applies the known font, selected theme, and initial system preference before showing Gallery, even when its theme equals the default. On Windows it retains Segoe UI Variable Text; elsewhere Slint's system font fallback remains. Font files are not bundled.

GallerySystemTheme is a separate, never-shown Slint host instance whose standard Palette system binding is never overridden. It observes the backend's system color preference and reports changes to DesignGalleryWindow. DesignGalleryWindow explicitly applies the same effective Light/Dark scheme to Kit Theme and its own standard Palette. Unknown falls back to Light consistently. Only the Gallery owns these host instances; the root helper does not depend on Slint or detect system settings. OS notification support varies by backend and requires native verification; manually toggling Light/Dark does not prove OS theme transitions.

## Snapshot interface

Existing names remain supported:

| Variable | Accepted values |
|---|---|
| QUADRANT_GALLERY_WIDTH / HEIGHT | Both supplied together, finite positive numbers; otherwise use window defaults |
| QUADRANT_GALLERY_THEME | light, dark, system (case insensitive), default light |
| QUADRANT_GALLERY_PAGE | Integer 0–7, default 0; 8 is an error |
| QUADRANT_GALLERY_PREVIEW | Integer 0–2, default 1 |
| QUADRANT_GALLERY_SNAPSHOT | Nonempty output path; absent means interactive run |

Invalid configuration fails before constructing the window. Snapshot mode preserves bounded retries for transparent frames, PNG output, and nonzero errors for output failures. The screenshot tool adds a 30-second subprocess timeout. It only stops its own child on timeout.

```console
python scripts/capture_gallery_baseline.py --mode Smoke --page 4 --preview 1
python scripts/capture_gallery_baseline.py --mode Smoke --page 4 --preview 1 --reuse-existing
```

Windows PowerShell entry:

```powershell
pwsh -NoProfile -File scripts/capture_gallery_baseline.ps1 -Mode Smoke -Page 4 -Preview 1
```

The wrapper invokes the Python standard-library script. PowerShell 7 is the local verification target; Windows PowerShell 5.1 has not been tested. Ordinary Gallery keeps its existing default backend selection. The snapshot tool explicitly defaults to winit-software for reproducibility; its `--backend` option permits winit-skia or winit-femtovg if supported by the build. This does not change Cargo features or normal runtime defaults.

Smoke captures Light at 1040×800 / 100%. Matrix captures four sizes, Light/Dark, and 100/125/150/200/225% simulated scales for the selected page/preview. All combines them. Simulated scales are render checks, not actual monitor DPI transitions.

Outputs default to `target/visual-baselines/<full-sha-or-dirty-content-id>/<os-backend-renderer>/`. Each image has its own JSON manifest; page, preview, theme, size and scale appear in filenames. Manifests record full Git SHA when available, dirty/content identity, OS, architecture, renderer, font policy, Slint version, binary hash, logical and actual pixel dimensions, PNG hash, and UTC time. An unborn repository explicitly records no SHA and uses a dirty content identifier. Custom output directories should be outside the repository or under ignored target output.

Reuse requires complete scenario equality and a matching actual PNG hash/dimensions. Other pages and environments are never merged into a current-scene manifest. A fresh capture uses a new temporary PNG so stale files cannot impersonate successful output. Source changes during build/capture cause an explicit failure. Cross-OS/font/renderer images are not asserted pixel-equal. Font policy is recorded, but installed font binary/version parity still needs control for strict cross-machine comparisons.

## Validation and limits

Phase 1 builds the independent Gallery and API probe on Windows. Local command logs and smoke output are recorded under `target/phase1`; exact successful checks are listed in the Tasks migration ledger at the phase handoff. Do not infer passing remote CI, other platforms, MSRV 1.92, package runtime, or accessibility from source presence.

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/verify_distribution.py
python -m unittest discover -s scripts/tests -p "test_*.py"
```

Phase 2 adds the lexical API/layer guard, declaration/default baseline, fixtures, archive verification, incremental token/SVG checks and CI jobs. Current evidence and remaining platform/native limits are in [VALIDATION.md](VALIDATION.md). The Phase 1 table below is historical; it does not substitute for later checks.

### Phase 1 local result — 2026-09-06

The final implementation checked is `acad992704b75959f1ae2f51304864e919b5a87b`; this result is recorded by a subsequent documentation-only commit. Environment: native Windows x86_64 MSVC, Rust/Cargo 1.94.1, Python 3.13.15, Slint 1.17.1.

| Check | Result | Local evidence under target/phase1 |
|---|---|---|
| fmt / clippy with all targets, all features and warnings denied | PASS | `fmt.log`, `clippy-final.log` |
| Rust workspace tests | PASS: 1 helper + 4 configuration tests | `tests-final.log` |
| Python snapshot identity/reuse tests | PASS: 2 tests | `python-tests.log` |
| Isolated local Git clone, own target directory, Gallery build | PASS; no Tasks checkout or Product packages required | `isolated-build-final.log` |
| Static closure / package list | PASS: 32 SVGs with matching hashes; required source and license files included | `distribution-final.log` |
| Source package verification | PASS: 70 files; root helper compiled from the package | `package-final.log` |
| Eight pages, Light/Dark/System, preview 0/1/2, representative 200%/225% simulated scales | PASS for 12 captured rendering scenes | `native/results.json`; final header fix rechecked in `native-final/result.json` and final Controls smoke |
| Invalid input and output path handling | PASS: 8 invalid configurations rejected with exit 1; output-as-directory rejected with exit 2 | `native/results.json`, `native/output-error-recheck.log` |
| Public PowerShell capture entry and matching-scene reuse | PASS | `capture-final.log`, `capture-reuse.log` |

The full eight-page run preceded a final neutral header icon tint correction; final Light Controls and Dark Navigation at 225% were recaptured afterward. The error-case harness initially could not decode a localized Windows error; its output-error case was repeated with explicit UTF-8 and preserved correctly. The Gallery process correctly returned exit 2 on both runs.

This is Gate 1 evidence, not complete migration acceptance. No remote push/CI/consumer build, real OS theme-toggle test, real monitor DPI transition, or complete native keyboard/IME/a11y matrix is claimed. Raw logs/screenshots and the isolated checkout are private local artifacts; the source/manifests/documentation are versioned.

## Learning order

Start with theme.slint and constants.slint, then FluentIcon / SurfaceCard / Badge. Run Gallery while reading one component's small property/callback surface. Next study buttons and focus/disabled handling, then std-widgets text wrappers, page/navigation/settings composition, and Toast/Modal. Make a small change, build and observe the affected specimen, then restore the experiment or commit an intentional change. Tasks does not need to be open for this workflow.

## First exercise: token to Badge to Gallery

Open only the Kit Git root. Start with a clean `ui/primitives/badge.slint` and
preserve any existing work before experimenting. No Tasks process or source is
required for any of the following steps.

1. Read `UiConstants.space_4: 4px` in `ui/foundation/constants.slint` and the
   `Theme`/`Typography` bindings in `ui/foundation/theme.slint`. These are Slint
   globals; each host component instance owns its initialization.
2. Run `cargo run --locked -p quadrant-kit-gallery`. Overview (page 0) shows
   neutral/accent badges; Controls (page 4) also shows five semantic kinds.
   Inspect Badge's two public input properties and Text child, then close Gallery
   before rebuilding the executable on Windows.
3. Capture the starting state:

   ```console
   python scripts/capture_gallery_baseline.py --mode Smoke --page 0 --preview 1 --output-directory target/learning/before
   ```

4. In `ui/primitives/badge.slint`, temporarily replace only
   `border-radius: 12px;` with `border-radius: UiConstants.space_4;`. The component
   already imports UiConstants. Leave the global token and public baseline alone.

   ```console
   python scripts/check_ui_boundaries.py
   python scripts/capture_gallery_baseline.py --mode Smoke --page 0 --preview 1 --output-directory target/learning/modified
   ```

   The capture command builds and runs Gallery. Compare the two PNGs: pill-shaped
   badges become rounded rectangles. The guard passes because no declared public
   signature/default changed; that does not approve the visual change for users.
5. Restore exactly that line to `12px` (or restore your saved original bytes).
   Run the guard and capture with `--output-directory target/learning/restored`.
   Verify `git diff -- ui/primitives/badge.slint` is empty relative to your starting
   state. If keeping a deliberate improvement instead, review it, add appropriate
   coverage/changelog notes and commit Kit separately before proposing adoption.

On 2026-09-06 this exercise was executed from Kit at
`838ecfbead2d0a1966907ddd742cb6f34516d3f6`: native Windows, Rust 1.94.1,
Slint 1.17.1, winit-software, Light, 1040×800, 100%, page 0/preview 1.
All three guards and build/capture commands exited 0. The modified image was
visually checked and had a different hash. Restored source bytes and the restored
PNG matched their starting hashes. Logs, source/scenario manifests and PNGs are
retained in ignored `target/phase7/learning/`; the experiment was not committed.
This demonstrates the learning loop, not an additional IME/DPI/a11y test.
