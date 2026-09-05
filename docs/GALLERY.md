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

The old page 8 and Task patterns filter are removed. Inbox samples remain in the unchanged Tasks source during staging; their eventual Product fixture preservation belongs to cutover. Kit has no Inbox models, product brand, task navigation aliases, or quadrant colors.

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

The distribution script validates this package's current static imports, resource ownership and hashes. Full Slint lexical/API/architecture guards, their fixtures, incremental token/SVG checks, Linux/macOS CI, Rust 1.92, remote consumers, and complete native interaction matrices remain later gates.

## Learning order

Start with theme.slint and constants.slint, then FluentIcon / SurfaceCard / Badge. Run Gallery while reading one component's small property/callback surface. Next study buttons and focus/disabled handling, then std-widgets text wrappers, page/navigation/settings composition, and Toast/Modal. Make a small change, build and observe the affected specimen, then restore the experiment or commit an intentional change. Tasks does not need to be open for this workflow.
