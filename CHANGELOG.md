# Changelog

## 0.1.0 — unreleased extraction candidate

- Extract generic Slint source and Gallery into an independent workspace. Root Rust helper exposes only the facade library name and build-time file location.
- Public facade deliberately changes from 32 embedded names to 28: Branding, TaskRowShell, InboxItem, and InboxPane remain with Tasks.
- Remove Q1–Q4 colors, Typography.timer, UiConstants.focus_wide_breakpoint, and 11 product icon aliases from Kit. Preserve other component defaults, behavior, and visual recipes.
- Retain 32 required generic SVGs unchanged with their MIT license and source mapping. Embed static Slint assets as files at build time.
- Gallery now has eight neutral pages (0–7). Remove Inbox routing, task specimens, product branding and workflow text; keep controls, previews, and feedback specimens.
- Reject invalid snapshot configuration before native window creation; explicitly initialize theme/font and follow backend system theme through a separate never-shown host instance.
- Add a compiled probe covering all 28 public names. Replace snapshot filename-only reuse with source, environment, and scenario manifests.

This is not a compatibility-preserving release of the old embedded facade. No stable release tag or verified remote consumption exists at this stage. Required validation and outstanding limitations are tracked in docs/GALLERY.md.
