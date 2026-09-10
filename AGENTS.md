# Quadrant Kit

## Mission and boundaries

Quadrant Kit is a reusable, source-only Slint component library with a development
Gallery. It aims for lightweight desktop controls visually close to WinUI 3, using
public controls of the pinned Slint version as their behavior foundation.
It is not a replacement UI runtime and does not embed Microsoft.UI.Xaml.

The root Rust package only exposes SLINT_LIBRARY_NAME and slint_library_path()
for consumer build scripts. Keep its normal/runtime dependency graph empty.
Do not add a required runtime adapter, event loop, window, background task,
business model, storage, IPC, Agent service, route history or page cache to Kit.

## Public entry, implementation and version policy

Consumers import @quadrant-kit, mapped to ui/kit.slint. This facade is the unified
public entry: it only re-exports current components and types. Component
implementation and consumer/business code remain separate.

Registering a component means a static export in ui/kit.slint, not a runtime
registry, factory, JSON dispatch table or per-host registration callback.
Implement a new component in its proper layer, export it from the facade, then
use it through the named entry. Update current docs, probe and Gallery coverage.
Adding a feature to an existing component does not re-register that component.
The Gallery catalog is a demo index, not the library's registration mechanism.

Keep public APIs small, consistent and meaningful; avoid gratuitous churn.
Modularity does not promise backward compatibility. Components, properties,
names, types, directions, callbacks, defaults and base types may change when
there is a concrete design benefit. Update current callers, docs and tests in the
same scoped change, and document removals/renames/behavior changes clearly.

Each version contains only its current implementation. Remove replaced code;
do not keep legacy branches, duplicate implementations, deprecated aliases,
compatibility adapters or version-dispatch paths. Developers needing old APIs
choose an old version themselves. Do not rewrite Git history or retained tags.

Do not create frozen legacy-consumer fixtures or require old source to compile
against both old and new Kit versions. API snapshots and probes check consistency
of the current version, not permanent preservation of historical signatures.
Review intended additions, removals and changes, synchronize their use sites,
then deliberately adopt the current snapshot. CI never auto-refreshes baselines.
Do not erase valid behavioral assertions merely to hide a regression.

Internal imports are acyclic: patterns/overlays -> primitives -> foundation.
Patterns and overlays do not import each other. Internal code never imports
back through the facade or depends on Gallery. Do not require duplicate interface
and implementation files or an extra wrapper for every component without benefit.
See ui/AGENTS.md for ownership and docs/ARCHITECTURE.md for the dependency map.
Keep the Mermaid architecture diagram and static-export explanation in README
aligned with the code. Do not replace the diagram source with a screenshot.

## Consumers, assets and native controls

Gallery may use the same-repository Kit path as a build dependency. Explicitly
scoped generated verification consumers may use a test-only path under target/.
These exceptions do not authorize Product path overrides. Tasks consumes only a
verified public Git URL and full retained commit SHA. Do not introduce sibling
paths, patches, copied Kit code or build-time downloads into Tasks.
Do not change Tasks or its dependency revision as part of a Kit-only task.

Keep Product branding, Inbox, task rows, quadrant colors and product icon aliases
outside Kit. Keep assets local, preserve source attribution and per-asset licenses,
and never redistribute system font files.

Reuse public std-widgets.slint controls and public builtins whenever the behavior
exists. Wrap, compose and decorate through supported APIs. Do not reimplement
ordinary button activation, text editing, selection, dragging or scrolling.
A hidden native control behind a custom replacement is not reuse.

Custom behavior is permitted only for a genuinely missing component or a missing
composition-specific behavior. Record its narrow scope in native_reuse_manifest
and NATIVE_REUSE.md. Composites must still reuse their native children. Do not
import, copy, fork or vendor Slint internal widgets to bypass public limitations.

Maintain one actual input owner per native control. Do not duplicate TouchArea,
keyboard activation or accessible default actions. Keep focus, disabled, IME and
accessibility behavior correct and test it. Remove screenshot-only preview APIs
when migrating their components; do not preserve them in compatibility branches.

## State, motion and performance

Define state ownership for the current API. Do not silently convert host-controlled
state into component-owned state or keep conflicting old/new writable fields.
Programmatic updates and user events retain their documented current meanings.
Animations must not cause duplicate commands or seize ownership of input state.

Slint globals are shared within one top-level component instance, not automatically
across independent windows. Each host initializes theme, system state, font and
public Palette coordination. OS detection and native window actions stay host-side.

Use shared semantic tokens and small justified compositions. Default to flat
controls and optional limited elevation. No runtime theme parser, reflection,
asset scan or registration service is needed for ordinary components.
Keep native-owned animations native-owned. Kit motion is restrained, cancelable
and inactive when hidden; document the real scope of reduced-motion control.
Use bounded exit lifecycles and stop unnecessary timers afterward.

No runtime dependencies does not mean zero UI overhead. Measure release builds
against equivalent native Slint scenes with matching backend, renderer, features,
fonts, DPI and content. Record raw data and environment. Do not relax budgets,
trim process working sets or remove accessibility to manufacture a win.

## Staged execution and truthfulness

For the active task read docs/specs/QUADRANT_KIT_FLUENT_EVOLUTION_SPEC.md and
 docs/STATUS.md. The v1.1 modularity/version policy supersedes the earlier
requirement to preserve old call signatures. Specification scope, roadmap and
acceptance are split under docs/specs/. Historical reports are recoverable from
the Git revision documented in docs/HISTORY.md; their results retain original scope.

A generic implementation request means P0 only; explicit later instructions take
precedence. Run only the requested phase. For P4/P5, run the first unfinished
subphase unless a narrower one is named. Stop after its report; no full rewrite.
Do not mix in Slint/MSRV/backend upgrades or another Gallery window-shell rewrite.

Before editing or Git operations inspect status and preserve unknown changes.
Keep one writer per checkout. Do not change published history or publish private
machine files. Commit/push/tag/release or Product cutover only when authorized.

Report PASS, FAIL, NOT_RUN and BLOCKED accurately. Compilation, screenshots,
input, accessibility and performance are separate evidence categories.
Never invent revisions, successful tests, screenshots or performance numbers.

## Validation and documentation

Use actual commands and prerequisites in docs/VALIDATION.md. Core checks:

    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    cargo test --workspace --locked
    python scripts/check_ui_boundaries.py
    python scripts/check_native_reuse.py
    python -m unittest discover -s scripts/tests -p "test_*.py"
    cargo build --locked -p quadrant-kit-gallery
    python scripts/verify_distribution.py --package
    cargo package --locked -p quadrant-kit --list

Run current API/consumer, native-reuse and performance checks as implemented and
documented. Extend the existing fail-closed scanner with fixtures; do not replace
it with silent parsing failures. A deliberate API change updates current tests
and snapshots; an accidental undocumented change still fails.

Package checks may need a clean committed checkout; do not commit unknown files
merely to satisfy that condition. Run verify_incremental.py only with exclusive
checkout/build access because it changes and restores a token and SVG.

For component changes synchronize facade exports, current API docs/snapshot/probe,
current use sites, Gallery/catalog, component status, native reuse and relevant
motion/accessibility/performance evidence. Remove superseded implementation and
API remnants. Permanent rules belong here; current gaps belong in docs/STATUS.md and concise
source/evidence pointers in docs/HISTORY.md. Keep raw logs under ignored target/;
avoid a new permanent document for each retry or short UI follow-up.

ModalManager remains a single confirmation overlay; complete focus containment,
restoration and screen-reader support require actual evidence, not a scrim.
Read docs/CONSUMER_GUIDE.md before source distribution or consumer work.
