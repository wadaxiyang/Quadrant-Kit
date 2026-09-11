# Consumer guide

Kit is compiled into the consumer's Slint application. The root helper exports
`SLINT_LIBRARY_NAME` and `slint_library_path()` only; generated Rust, windows,
event loops and business state belong to the application.

## Select a source

Use the public Git URL `https://github.com/wadaxiyang/Quadrant-Kit.git` and a verified
full 40-character SHA resolved from the retained `v0.1.0` tag. The current Fluent
API differs from the historically published extraction candidate. [Status](STATUS.md)
records the current release gaps; [History](HISTORY.md#published-extraction) records
the old adopted SHA. Do not copy that old revision while expecting the current API.

Add `quadrant-kit` as a build dependency only after selecting that verified source.
Pin consumer `slint` and `slint-build` to `=1.17.1`; use edition 2024 and Rust >=1.92.
Do not substitute sibling paths, source patches, cached copies or build-time downloads
in Tasks. Gallery's same-repository path and generated verification consumers under
target are explicit development exceptions. Use `--locked` after reviewing the lockfile.

## Build and import

Configure the consumer build script:

```rust
let libraries = std::collections::HashMap::from([(
    quadrant_kit::SLINT_LIBRARY_NAME.to_owned(),
    quadrant_kit::slint_library_path(),
)]);
let config = slint_build::CompilerConfiguration::new()
    .with_style("fluent".into())
    .with_library_paths(libraries)
    .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
// Compile the consumer's actual .slint entry with this configuration.
```

The named mapping targets the facade **file**, not its directory:

```slint
import { FluentButton, Theme } from "@quadrant-kit";
```

Compile the consumer's real entry with this configuration. Internal Kit paths are
not public imports. For editor `slint.libraryPaths`, use the same mapping, locating
the selected package with `cargo metadata --locked --format-version 1` (not
`--no-deps`). Keep absolute editor settings local and ignored.

## Initialize each window

- Set Theme.mode, system_dark and ui_font_family from host policy, then coordinate
  that window's standard Palette.color-scheme in the same direction.
- Initialize Motion.animations_enabled / reduced_motion per window. They affect
  Kit-owned transitions, not every native animation.
- Bind host-controlled state and handle requests according to [PUBLIC_API](PUBLIC_API.md).
  Programmatic assignments and user callbacks have distinct meanings.
- Bind arbitrary slot children's enabled state explicitly. Expander requires
  `if expanded: Content { ... }`; include page activity when retaining hidden pages.
- Close/unload popups and overlays before hiding retained ancestors. Bind modal
  focus restoration to an actual opener or valid fallback.

Static resources use EmbedFiles. Validate other native runtime assets separately;
never redistribute system font files. GPL code and MIT SVG obligations still apply
when Kit is a build dependency.

## Adoption checks

Before adoption, review same-SHA CI, the retained remote reference and actual runtime
evidence. [VALIDATION](VALIDATION.md#distribution) documents archive and optional
anonymous remote checks. The independent fresh Git+SHA consumer was explicitly
NOT_RUN for `v0.1.0`; users needing that assurance should run it before adoption.
Build-only results mean runtime NOT_RUN. Never move an adopted tag or update Tasks'
revision as part of a Kit-only change.

When upgrading, reconcile the current [API](PUBLIC_API.md) and [changelog](../CHANGELOG.md).
There are no old-name adapters, frozen consumer fixtures or cross-version signature guarantees.
