# Changelog

## 0.1.0 — unreleased extraction candidate

- Extract generic Slint source and Gallery into an independent workspace. Root Rust helper exposes only the facade library name and build-time file location.
- Public facade deliberately changes from 32 embedded names to 28: Branding, TaskRowShell, InboxItem, and InboxPane remain with Tasks.
- Remove Q1–Q4 colors, Typography.timer, UiConstants.focus_wide_breakpoint, and 11 product icon aliases from Kit. Preserve other component defaults, behavior, and visual recipes.
- Retain 32 required generic SVGs unchanged with their MIT license and source mapping. Embed static Slint assets as files at build time.
- Gallery now has eight neutral pages (0–7). Remove Inbox routing, task specimens, product branding and workflow text; keep controls, previews, and feedback specimens.
- Reject invalid snapshot configuration before native window creation; explicitly initialize theme/font and follow backend system theme through a separate never-shown host instance.
- Add a compiled probe covering all 28 public names. Replace snapshot filename-only reuse with source, environment, and scenario manifests.
- Add the fail-closed lexical boundary/API guard, explicit signature/default baseline, Cargo policy fixtures, shared static-resource scanner and package archive verification.
- Add incremental token/SVG rebuild verification and push/PR/manual CI for Linux, Windows, macOS and the Rust 1.92 build baseline. Actual execution evidence is tracked separately from workflow presence.

This is not a compatibility-preserving release of the old embedded facade.
The extraction candidate has now been published, retained and remotely consumed;
no stable release tag is claimed. Source/CI references and outstanding limitations
are tracked in docs/VALIDATION.md and docs/CONSUMER_GUIDE.md.

## Documentation handoff — 2026-09-06

- Align candidate publication, consumer setup and Gallery ownership with the
  implemented repositories. Add an executable dependency example using the
  qualified retained source, without changing Tasks' adoption.
- Add the completed token → Badge → Gallery exercise and relocation cache
  recovery. The trial visual change was fully restored; no public API/default,
  asset, runtime behavior, Slint version or MSRV changed in this handoff.
- Keep native accessibility/focus/IME limitations explicit. Documentation
  handoff is not a new component release or a waiver of those follow-ups.

For future changes, document compatible fixes and reviewed visual impact in a
patch; record compatible additions with facade/docs/probe coverage. Breaking
names/types/callbacks or important behavior require the next minor (for example
0.1.x → 0.2.0) and consumer migration instructions. Review toolchain/Slint/backend
changes separately. Git SHA pinning does not make all pre-1.0 versions compatible.
