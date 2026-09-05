# Validation and boundary contracts

The architecture authority is the extraction SPEC v2. Phase 2 retains the original Tasks guard's import, dependency, cycle and frozen-API checks, with a new small Python standard-library scanner for Kit. Tasks still uses its unchanged embedded Kit and guard until the Phase 4 cutover.

## Guard scope

`python scripts/check_ui_boundaries.py` reads the full Kit/Gallery Slint trees, the facade's reachable exports, the reviewed API baseline, asset manifest, source headers, Cargo manifests and host-filtered `cargo metadata --locked`. It checks:

- foundation → no higher layer; primitives → foundation; patterns/overlays → their own layer, primitives and foundation. Same-layer imports must be acyclic; patterns and overlays cannot import each other. Implementation cannot import the facade.
- Gallery may import its own files, std-widgets and the exact named `@quadrant-kit` facade. Raw Kit paths, external imports and canonical path escapes fail.
- Multiline declarations/imports, re-export aliases, comment/string delimiters, escaped strings, public property direction/type, callback arguments/returns, pure public functions, enum order/values, struct fields and explicit base types. Unknown public syntax fails explicitly.
- All 28 reachable public names, extra/missing exports and duplicate definitions. Actual Product names/tokens are rejected; generic FocusScope, focus-ring, task text and copyright references remain legal.
- Static image references share the same scanner with the distribution checker. Referenced files must stay in the package; all icon assets have a current hash/MIT record, and handwritten source retains GPL/copyright headers. MIT SVGs are never given GPL source headers.
- Kit helper has no normal/runtime dependency edges. Gallery's same-repository Kit path build dependency is allowed. Product packages and source patches/replacements fail; resolved Slint stays at 1.17.1.

This is deliberately not a complete Slint compiler. Expression typing, builtin inherited properties, event behavior and native accessibility require the pinned Slint compiler, Gallery probe and runtime evidence. String interpolation and new declaration syntax outside the supported subset currently fail and require an explicit scanner extension with fixtures. Literal Unicode escapes follow the installed Slint 1.17.1 literal implementation. Formatting/comments and underscore/hyphen spelling normalize; string contents retain semantic whitespace.

The Cargo module also has fixtures for future Tasks Git+full-SHA, alias, workspace-inherited, target/build/dev, local-source and resolved-revision rules. These fixtures are not a claim that the final Tasks guard has been installed: config/build-source mapping, Agent/GUI target graphs and Product Rust/window API checks remain Phase 4 responsibilities.

## Reviewed baselines

`scripts/kit_api_v1.json` schema 1 records **28 names, 217 explicitly declared properties, 13 callbacks and four enums**. Each export has separate `signature` and `defaults` sections. Declaration order and physical implementation paths are not signature keys. Inherited custom component contracts are protected by their own exported baseline plus the recorded base name; builtin inherited properties are covered by the fixed compiler version. Default expressions are token-normalized, not evaluated: an expression change is reported for review even when it may evaluate identically. Callback/function argument order and enum order are preserved. Function bodies and other interaction behavior require review/tests beyond this declaration baseline.

Initial migration differences are explicitly authorized by SPEC v2:

| Area | Removed from the old embedded Kit |
|---|---|
| Public names | Branding, TaskRowShell, InboxItem, InboxPane |
| Theme | q1_accent, q2_accent, q3_accent, q4_accent |
| Typography | timer |
| UiConstants | focus_wide_breakpoint |
| FluentIcons | quadrants/today/focus/review/completed regular and filled aliases, restore_task |

All other declared generic defaults remain unchanged. The existing 32-entry asset baseline was reviewed against the live static closure and byte hashes; SVG bytes and licensing were not refreshed or rewritten. Extraction hashes are historical provenance, while this API baseline and asset manifest are live guards.

To propose an intentional API update:

```console
python scripts/check_ui_boundaries.py --write-baseline target/kit_api_candidate.json
git diff --no-index scripts/kit_api_v1.json target/kit_api_candidate.json
```

The write command only emits a candidate; it does not report validation success. Review both signature and default changes, update public documentation/probes as needed, then deliberately adopt and commit the baseline. CI invokes only the read/check command and never refreshes a baseline.

## Reproducible checks

Use the commands in the root README with Python 3.11+. `cargo package --locked -p quadrant-kit` must run from a clean committed checkout. Distribution verification checks Git-tracked closure against package list and compares required Slint/resource/license bytes in the actual `.crate` archive without extracting it.

```console
python scripts/verify_incremental.py
cargo +1.92.0 build --locked -p quadrant-kit -p quadrant-kit-gallery --target-dir target/msrv-1.92
```

The incremental command requires exclusive checkout/build access. It builds Gallery, changes a deep Theme accent token, rebuilds, restores exact original bytes and rebuilds again; then repeats with a referenced SVG. It requires the Gallery build script to rerun and the binary hash to change. It saves verbose build logs and a JSON report under the selected target's `incremental-verification/`. This is build invalidation evidence, not an assertion of pixel equality.

`.github/workflows/ci.yml` runs on push, pull_request and workflow_dispatch with contents:read. Linux runs quality, boundary/tests, Gallery/probe, package/archive and incremental checks; Windows builds/checks and captures a smoke scene; macOS checks workspace all-targets and the guard; a separate Windows job actually builds helper and Gallery with Rust 1.92.0. CI execution and retained remote consumption are Phase 3 gates, not inferred from this workflow's presence.

## Phase 2 local evidence — 2026-09-06

Local evidence is kept under ignored `target/phase2/` and `target/incremental-verification/`; screenshots carry their own source/environment identity. Platform results are recorded at the phase handoff after the final commands. Development Rust remains 1.94.1; dependency versions and Slint renderer defaults were not upgraded.

Native keyboard/IME, real system-theme transitions, real monitor DPI changes and complete accessibility acceptance remain unverified. In particular ModalManager does not yet promise a complete focus trap/restoration contract; text wrappers retain std-widgets behavior but custom control screen-reader/focus coverage remains a P1 follow-up. Phase 1 screenshot evidence is documented separately in GALLERY.md.
