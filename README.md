# Quadrant Kit

Reusable Fluent-oriented Slint source components, with a Gallery for development and learning. This is an independent Cargo workspace; Quadrant Tasks is not needed to build it.

Version **0.1.0 is an unpublished extraction candidate**. No release tag or remotely consumable Kit commit has been verified yet. The source was extracted from Quadrant at `5a2262cd480d639673fa4f5dd406a9c7196361b5`; see [provenance](docs/PROVENANCE.md).

## Run

With the existing pinned Rust 1.94.1 toolchain and native desktop build prerequisites:

```console
cargo build --locked -p quadrant-kit-gallery
cargo run --locked -p quadrant-kit-gallery
```

The declared minimum Rust version is 1.92; its build verification is tracked separately from the pinned development toolchain. Slint and slint-build stay at 1.17.1. Default Slint features, Fluent style, and runtime renderer selection are retained.

The root `quadrant-kit` package is only a build-time source locator. It owns no event loop, windows, persistence, platform integration, or Slint runtime dependency. Consumers compile the Slint source into their own application through the single `@quadrant-kit` facade.

## Explore

- [Architecture and component conventions](docs/ARCHITECTURE.md)
- [Public API and coverage](docs/PUBLIC_API.md)
- [Consumer setup](docs/CONSUMER_GUIDE.md)
- [Gallery, snapshots, and learning sequence](docs/GALLERY.md)
- [Source ownership and licenses](docs/PROVENANCE.md)
- [Candidate changes](CHANGELOG.md)

The public facade exports 28 names. Branding, task models, Inbox, task row composition, quadrant colors, product-specific timer/layout tokens, and product navigation aliases belong to Tasks. Generic component behavior and defaults are preserved during extraction.

Code is GPL-3.0-only; the Microsoft SVG assets retain their MIT license. See [LICENSE](LICENSE), [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md), and [assets/icons/LICENSE-MIT](assets/icons/LICENSE-MIT).
