# Consumer guide

Quadrant Kit is a Slint source library compiled by each consumer. The root helper exports `SLINT_LIBRARY_NAME` (`quadrant-kit`) and `slint_library_path() -> PathBuf`. It has no Slint runtime types: generated windows, enums, globals, and DTOs belong to the consumer's `slint::include_modules!()` module.

## Candidate status

The target remote is `https://github.com/wadaxiyang/Quadrant-Kit.git`; no published revision has been verified in Phase 1. Do not switch Tasks yet. The following integration describes the eventual contract, not an executable placeholder dependency.

After publication, add a build dependency named `quadrant-kit` with that Git URL and the verified full 40-character commit SHA to the consumer UI crate. Do not use a sibling checkout, path patch, source replacement, cache copy, or a build script that downloads the library. Keep the adopted commit reachable through a retained reference; review the lockfile and use `--locked` afterward. Gallery's `path = ".."` is legal because it is inside this repository.

Pin both `slint` and `slint-build` to `=1.17.1`. Use edition 2024 and the declared Rust 1.92 baseline; development currently pins 1.94.1. Verification of the minimum version is tracked independently. Do not silently upgrade dependencies or change backends during integration.

In the consumer build script:

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

The mapping targets the facade **file**, not its parent directory. Consumer Slint code uses:

```slint
import { FluentButton, Theme } from "@quadrant-kit";
```

Internal `foundation/`, `primitives/`, and other file paths are not supported public imports. The application still owns generated code, event loop, window lifecycles, system integration, font strategy, and per-window Theme/Palette synchronization. Explicit embedding covers static Slint resources; validate the complete runtime package and any native assets separately.

## Editor and validation

For `slint.libraryPaths`, map `quadrant-kit` to the resolved package's `ui/kit.slint`, using `cargo metadata --locked --format-version 1` to locate the selected Git package. Do not use `--no-deps` to locate an external package, or redirect Tasks' editor to a sibling checkout. Keep absolute editor paths local and ignored.

Before adopting a release, require a retained remote commit, CI for that SHA, a fresh Git+SHA smoke consumer, and a no-sibling consumer build. Check package imports/assets, licenses, generated API, every window instance, and native runtime resources. Phase 1 local Gallery success is not evidence that any remote consumer already passes.

Code remains GPL-3.0-only and the SVGs retain MIT. Preserve attribution and assess distribution obligations for the actual combined application; being a build dependency is not a license exemption.
