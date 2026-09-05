# Quadrant Kit

This independent repository contains reusable Slint source components and their Gallery. The root Rust crate only supplies `SLINT_LIBRARY_NAME` and `slint_library_path()` for build scripts; it has no UI runtime or application responsibilities.

Consumers and Gallery import `@quadrant-kit`, mapped to the `ui/kit.slint` file. Implementation imports follow patterns/overlays → primitives → foundation, with no cycles or imports back through the facade. Gallery may use a same-repository path build dependency on the root package. Tasks may consume Kit only through a verified public Git URL and a full commit SHA after publication and external-consumer verification. Never introduce sibling path overrides, patches, replacements, copied Kit implementations, or build-time downloads into Tasks.

Product branding, Inbox, task rows, quadrant colors, product-specific tokens and icon aliases, domain/storage/IPC/Agent dependencies are outside Kit. Keep static resources within the package, preserve attribution and per-asset licensing, and do not bundle system fonts.

Keep extraction separate from visual redesign, toolchain upgrades, and behavioral rewrites. Prefer existing std-widgets for text editing and scrolling. Globals are per component instance: each host must initialize its own theme, system state, and font. ModalManager is a single confirmation overlay; do not claim complete focus containment, restoration, or accessibility without evidence.

Available checks:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
python scripts/check_ui_boundaries.py
python -m unittest discover -s scripts/tests -p "test_*.py"
cargo build --locked -p quadrant-kit-gallery
python scripts/verify_distribution.py --package
cargo package --locked -p quadrant-kit --list
```

Read the actual commands and limitations in docs/VALIDATION.md and docs/CONSUMER_GUIDE.md. The versioned boundary/API guard uses a small fail-closed declaration scanner; Slint compilation and the Gallery probe complement it. Run scripts/verify_incremental.py only with exclusive access to the checkout and build directory because it temporarily changes and restores a token and SVG. Do not auto-refresh baselines to hide errors.

Before editing or Git operations, inspect this repository's status. Keep one writer per checkout; never overwrite unknown changes, change published history, or publish private machine files. Publishing a Kit candidate belongs to the explicitly authorized publication phase. Report unrun checks as NOT_RUN; no fake revisions or invented successful tests.
