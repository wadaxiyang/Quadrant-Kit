# Validation and boundary contracts

## Current P4 selection checks

`python scripts/run_button_checks.py --suite selection` builds and runs the isolated
P4A native event host, including programmatic state, disabled input and model changes.
The `numeric` suite covers native Slider/SpinBox bounds, editing/read-only/disabled,
and ProgressBar/ProgressRing running/hidden/stable-render state. The paired
`progress-100` scene creates 50 native/Kit bars and 50 rings with identical geometry.
RadioGroup static re-export is deliberately verified by the scanner and generated
Rust compilation; build-script-only generation cannot prove compiler-special child
lowering works. `run_perf.py --scenes selection-100 --samples 30` compares paired
native/Kit CheckBoxes under the existing release measurement protocol.

## Fluent evolution P2 — current checks

P2 migrates FluentButton to a visible native Button. The explicitly reviewed current
snapshot removes show_icon/accent/preview inputs and adds accessible_name plus
read-only native state outputs. Other component declarations are unchanged. Run
`python scripts/run_button_checks.py` for the isolated current event/geometry host;
it saves source, build/runtime logs and light/dark state images. Windows indexed
and physical input is separately recorded in the P2 report. Performance schema 2
separates software frame-buffer readiness from unsupported AfterRendering/present.

## P1 foundation checks retained

Current API checks compare the facade, reviewed current snapshot, PUBLIC_API
declarations/defaults and actual probe imports/uses. Export/member/type counts
derive from those current artifacts; historical counts below are measurements,
not permanent API restrictions. Keep valid behavioral assertions when changing
current call sites; do not maintain frozen old/new consumer fixtures.

`python scripts/check_native_reuse.py` is implemented and covered by positive/
negative fixtures in normal Python test discovery. `check_ui_boundaries.py` also
calls it, so the existing Linux/Windows/macOS CI boundary steps inherit enforcement.
It checks component-record coverage/public ownership, real native references,
aliases/composition, duplicate input, literal hidden proxies and reviewed current
custom-input debt. It prints pending migrations and runtime_verified=false.
See NATIVE_REUSE.md for limits; native runtime/a11y is never inferred from its PASS.

The scanner additionally retains balanced implementation bodies for that checker
and resolves only verified Slint 1.17.1 Date/Time public type exports. Builtin model
references are accepted as declarations; the locked compiler checks their semantics.
Unsupported public syntax/unknown std re-exports still fail. The native compilation
probe exercises aliases/re-exports and actual type bindings.

The existing Kit API probe includes host-controlled selection/state, explicit
slot enabled bindings, child content, focus entry methods and a narrow editor.
This proves compilation only. The status/manifest/catalog set and README's editable
SPEC Mermaid source also have tests. No public API/default or baseline changes
are adopted by P1. Phase-specific results belong in
[P1.md](implementation/kit-fluent-v1/P1.md); historical sections below keep their
original facts. Package checks still require clean committed source; no automatic
commit or baseline acceptance is authorized by a test failure.

Current native-window integration checks and platform limits are recorded in
[GALLERY_NATIVE_CHROME_VALIDATION.md](GALLERY_NATIVE_CHROME_VALIDATION.md).

The current evolution authority is [Fluent SPEC v1.1](specs/QUADRANT_KIT_FLUENT_EVOLUTION_SPEC.md)
and its [stage ledger](implementation/kit-fluent-v1/STATE.md). Extraction SPEC v2
and the NavigationView/Gallery rebuild gates retain their historical scope.
Kit's Python standard-library
scanner checks its own import, dependency, cycle, API and asset contracts. Tasks
has completed its separate cutover and owns its Product/runtime guards. Kit can
be built, checked and learned without opening Tasks. Historical phase results
below retain their original scope; publication is summarized at the end.

## Guard scope

`python scripts/check_ui_boundaries.py` reads the full Kit/Gallery Slint trees, the facade's reachable exports, the reviewed API baseline, asset manifest, source headers, Cargo manifests and host-filtered `cargo metadata --locked`. It checks:

- foundation → no higher layer; primitives → foundation; patterns/overlays → their own layer, primitives and foundation. Same-layer imports must be acyclic; patterns and overlays cannot import each other. Implementation cannot import the facade.
- Gallery may import its own files, std-widgets and the exact named `@quadrant-kit` facade. Raw Kit paths, external imports and canonical path escapes fail.
- Multiline declarations/imports, re-export aliases, comment/string delimiters, escaped strings, public property direction/type, callback arguments/returns, pure public functions, enum order/values, struct fields and explicit base types. Unknown public syntax fails explicitly.
- All 35 currently reachable public names, extra/missing exports and duplicate definitions. Actual Product names/tokens are rejected; generic FocusScope, focus-ring, task text and copyright references remain legal.
- Static image references share the same scanner with the distribution checker. Referenced files must stay in the package; all icon assets have a current hash/MIT record, and handwritten source retains GPL/copyright headers. MIT SVGs are never given GPL source headers.
- Kit helper has no normal/runtime dependency edges. Gallery's same-repository Kit path build dependency is allowed. Product packages and source patches/replacements fail; resolved Slint stays at 1.17.1.

This is deliberately not a complete Slint compiler. Expression typing, builtin inherited properties, event behavior and native accessibility require the pinned Slint compiler, Gallery probe and runtime evidence. String interpolation and new declaration syntax outside the supported subset currently fail and require an explicit scanner extension with fixtures. Literal Unicode escapes follow the installed Slint 1.17.1 literal implementation. Formatting/comments and underscore/hyphen spelling normalize; string contents retain semantic whitespace.

The Cargo module also has fixtures for future Tasks Git+full-SHA, alias, workspace-inherited, target/build/dev, local-source and resolved-revision rules. These fixtures are not a claim that the final Tasks guard has been installed: config/build-source mapping, Agent/GUI target graphs and Product Rust/window API checks remain Phase 4 responsibilities.

## Reviewed baselines

The subsequent user-requested Windows shell integration is recorded in
[GALLERY_TITLE_BAR_VALIDATION.md](GALLERY_TITLE_BAR_VALIDATION.md). It supersedes
the earlier main-Gallery native-decoration deferral without rewriting historical
phase evidence. Its checks apply to the changed host and explicit Slint feature.

Navigation Phase 6 catalog/route behavior is recorded in
[NAVIGATION_REBUILD_PHASE6.md](NAVIGATION_REBUILD_PHASE6.md). Current keyboard/focus
polish, the six-command common gate, native input observations and 184 simulated-DPI
render scenes are recorded in [NAVIGATION_REBUILD_PHASE7.md](NAVIGATION_REBUILD_PHASE7.md).
Real monitor transitions and unavailable platform/accessibility coverage remain
separately NOT_RUN. The final local construction gate and identified package/MSRV/
incremental evidence are tracked in [NAVIGATION_REBUILD_PHASE8.md](NAVIGATION_REBUILD_PHASE8.md).
The unchanged public API was reviewed in [Phase 3](NAVIGATION_REBUILD_PHASE3.md).

Current repository tests compare every PUBLIC_API.md Slint declaration, including
defaults, with the facade; reconcile the 35 probe imports and their uses; check
240 properties, 20 callbacks, 21 components, six globals, seven enums and the
ten-field NavigationEntry; and reject live legacy navigation/shell identifiers.
The catalog test separately requires all 21 visual exports to have real typed
destinations. Probe compilation complements these declaration/token checks.

`scripts/kit_api_v1.json` schema 1 records the exact current facade. The historical navigation baseline had 35 names; Fluent additions/removals are reviewed in the staged reports. Phase 1 added seven names, seven Theme aliases and four UiConstants properties; Phase 2 added NavigationView with 15 properties and six callbacks. Phase 3 removes SidebarItem and three legacy sidebar tokens, rebinding two navigation defaults to their identical resolved values; all other surviving contracts are preserved. Each export has separate `signature` and `defaults` sections. Declaration order and physical implementation paths are not signature keys. Inherited custom component contracts are protected by their own exported baseline plus the recorded base name; builtin inherited properties are covered by the fixed compiler version. Default expressions are token-normalized, not evaluated: an expression change is reported for review even when it may evaluate identically. Callback/function argument order and enum order are preserved. Function bodies and other interaction behavior require review/tests beyond this declaration baseline. The extraction's historical 28-name results below retain their original scope.

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

Implementation checked: `960373bd30d350699ed29fec667cb69ab3ad77fd`; this evidence is recorded by a subsequent documentation-only commit. Local logs are kept under ignored `target/phase2/` and `target/incremental-verification/`; screenshots carry their own source/environment identity. Development Rust remains 1.94.1; dependency versions and Slint renderer defaults were not upgraded.

| Check | Result | Evidence |
|---|---|---|
| Windows fmt, clippy all-targets/all-features with warnings denied, Rust tests | PASS; 5 Rust tests | `fmt.log`, `clippy.log`, `tests.log` |
| Windows boundary/API/assets/resolved Cargo and Python fixtures | PASS; 28 exports, 32 SVGs, 31 Python tests | `boundaries-final.log`, `python-tests-final.log` |
| Windows native Gallery + compiled API probe | PASS | `build-final.log` |
| Rust 1.92.0 Windows MSVC helper and Gallery actual build | PASS; separate MSRV target | `msrv.log` |
| Linux Ubuntu 24.04 x86_64 under WSL2: fmt, clippy all-targets/all-features, Rust tests, Gallery/probe build | PASS; 5 Rust tests; own Linux target | `linux-fmt.log`, `linux-clippy.log`, `linux-tests.log`, `linux-build.log` |
| Linux boundary/API/assets/resolved Cargo and Python fixtures | PASS; 31 Python tests | `linux-boundaries-final.log`, `linux-python-tests-final.log` |
| Independent local Git clone, own target, no Tasks checkout | PASS: boundary guard and Gallery build | `isolated-boundaries.log`, `isolated-build.log` |
| Source package and actual archive closure | PASS: 72 files packaged; 59 required files byte-checked, including 32 SVGs | `package.log`, `distribution.log` |
| Deep token and SVG incremental invalidation | PASS: both trigger Gallery build-script rerun and binary change; exact source bytes restored | `incremental-verification/result.json`; verbose logs name the changed token/SVG |
| Windows and Linux WSLg Controls rendering smoke | PASS: page 4, preview 1, Light, 1040×800, 100%, winit-software | `capture-windows.log`, `capture-linux.log`; source-keyed PNG/JSON pairs under `target/visual-baselines/` |
| macOS workspace all-targets | NOT_RUN locally: no macOS host/SDK; native CI job prepared | Must run on the published candidate in Phase 3 |
| Remote CI, retained commit and Git+SHA consumer | NOT_RUN: candidate has not been pushed | Phase 3 |

The Linux first build failed on missing fontconfig development files; the first WSLg render failed on missing libxkbcommon-x11. Both were resolved with Ubuntu packages extracted into a user-owned validation sysroot, with pkg-config and runtime library paths scoped to validation commands. No system package replacement or repository-specific linker override was committed. Initial stale apt indexes also produced 404s, resolved with a private refreshed package index. Linux/Windows screenshots share the same clean source SHA and content hash; fonts differ by platform, so pixel equivalence is not claimed.

**Gate 2 local requirements are satisfied:** Kit can be independently reviewed, built, tested and packaged, the public contract excludes Product APIs and licensing material is present. This is not completion of all platform/publication gates or of the full extraction. macOS and real remote consumption remain mandatory before final migration acceptance.

Native keyboard/IME, real system-theme transitions, real monitor DPI changes and complete accessibility acceptance remain unverified. In particular ModalManager does not yet promise a complete focus trap/restoration contract; text wrappers retain std-widgets behavior but custom control screen-reader/focus coverage remains a P1 follow-up. Phase 1 screenshot evidence is documented separately in GALLERY.md.

## Phase 3 publication verification

The distribution command now supports anonymous retained-reference verification and a generated Git+SHA consumer with fresh cache/target/source directories. Its additional fixtures cover request validation, inherited source/credential isolation, manifest generation and resolved source/path mismatches. See CONSUMER_GUIDE.md for parameters. Publication evidence belongs to the actual remote run/report and the Tasks migration ledger, not to a self-referential SHA inside this candidate. Historical NOT_RUN entries above describe Phase 2.

### Published source and acceptance evidence

The adopted source `838ecfbead2d0a1966907ddd742cb6f34516d3f6` passed all four jobs
in [candidate CI](https://github.com/wadaxiyang/Quadrant-Kit/actions/runs/34003620362),
[retained-tag CI](https://github.com/wadaxiyang/Quadrant-Kit/actions/runs/34004051391)
and [main CI](https://github.com/wadaxiyang/Quadrant-Kit/actions/runs/34004053852).
These are actual completed runs at that exact source, covering Linux quality,
package/archive and incremental checks, Windows native Gallery/screenshot,
macOS all-targets/guards and actual Rust 1.92 helper/Gallery builds. The suite
contains 35 Python fixtures and five Rust tests at this source.

The protected retained tag and peeled source are listed in CONSUMER_GUIDE.md.
Independent anonymous fetch, a neutral Git+SHA consumer with fresh Cargo/target
directories, and Light/Dark consumer rendering passed. Private reports are under
`target/phase3/`; the externally recorded integration evidence is in the
[Tasks ledger](https://github.com/wadaxiyang/Quadrant-Tasks/blob/main/docs/migrations/kit-extraction-v2.md).
Tasks does not need to be present to run any Kit check. Documentation added after
the adopted source is validated at its own commit; it does not retarget consumers.

Phase 7 also executed the reversible Badge exercise documented in GALLERY.md.
Remaining P1 work includes complete native keyboard/IME/screen-reader coverage,
modal Tab containment/restoration and the full state/size/backend matrix. These
are explicit coverage limits. Product's user-accepted tray/reminder/DPI scenarios
do not certify every Kit component or operating-system theme transition.


## Fluent evolution P3 current gate

Run the common checks above and `python scripts/run_button_checks.py --suite foundation`.
The current API snapshot intentionally changes seven declarations; the component
set stays at 35 names/21 visuals, now 234 properties and 20 callbacks. The P3 report
records reviewed removals/additions, true input evidence, all-page rendering and
retained intermediate failures. Historical API counts above remain historical.
No pending native-command allowance remains after P5C. Full reader/IME,
WinUI runtime reference and full native accessibility remain separate; current finite modal/navigation evidence is recorded in P5B/P5C.

### P4C native containers

`python scripts/run_button_checks.py --suite containers` builds an actual Rust
consumer and dispatches public WindowEvent input. It compares 10,000-item Kit/native
ListView delegate counts and scroll extents, then exercises model replacement,
StandardListView selection, wheel input, explicit group-child enable coordination
and static native tabs. `python scripts/run_perf.py --scenes lists-10000 --samples 30`
uses matching native/Kit virtual-list geometry and records all paired samples.

### P4D native table and picker checks

`python scripts/run_button_checks.py --suite pickers` verifies public WindowEvent
table input, native date/time acceptance/cancel, invalid calendar text, host value
ownership, disabled/rapid popup lifetime and opener focus. Native popup screenshots
include a bottom-right placement. This is separate from OS-level reader validation.
`python scripts/run_perf.py --scenes table-100 --samples 30` pairs 100-row native/Kit
tables with identical columns, dimensions, font, backend and renderer.

### P5A transient input/lifetime

`python scripts/run_button_checks.py --suite toast --timeout-seconds 45` waits
through real four-second timeouts and dispatches native pointer/key input. It checks
one request per cycle, host ownership, hover pause, hidden input, rapid reversals
and native tooltip focus behavior. The runner accepts a bounded 1..300 second
allowance for longer lifecycle/idle suites; default remains 30 seconds.

### P5B finite confirmation

`python scripts/run_button_checks.py --suite modal` checks actual native button
Return/Space, initial Cancel/primary focus, Tab/ShiftTab cycling, Escape, scrim,
programmatic/rapid close, one request per cycle and host restore callback counts.
`python scripts/run_button_checks.py --suite button` retains the current upstream
button/composition regression, now navigating from initial Cancel to Confirm.
These are finite WindowEvent contracts, separate from actual OS reader containment.

### P5C bounded navigation

`python scripts/run_button_checks.py --suite navigation --profile release` runs
16/64/256/257 model replacement/row-change/selected-only checks, real public
WindowEvent keyboard/pointer sequences, and 30 cold validation timings per size.
Raw BENCH lines retain every sample. `--profile` defaults to debug for other suites.
This is bounded ScrollView composition, not a virtualized infinite tree.

### P5D popups and native menus

`python scripts/run_button_checks.py --suite popup --profile release` exercises
actual native popup state/dismissal, focus restore, separate command regions and
scroll content. On Windows, native Menu uses an OS modal loop; the test sends
Down/Return only after verifying that its own process owns the foreground window.
This is a real Windows input subset, not reader verification. Raw logs retain the
foreground check and all native/current failures.

### P5E inline lifetime

`python scripts/run_button_checks.py --suite inline --profile release` verifies
controlled requests, required host-conditional Expander slots, child mount/timer
teardown, explicit header focus, native disabled behavior and InfoBar close cycles.
The popup suite additionally forces disabled Split focus before Space to guard
the native edge found by the P5E reproducer. Native reader/live-region semantics
remain a separate verification category.
