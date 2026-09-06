# Architecture

The root Rust package returns its own `ui/kit.slint` location using `env!("CARGO_MANIFEST_DIR")`. The returned path exists during consumer compilation; it is not a runtime resource URL and must not be persisted into application settings. Gallery is the only member application in this workspace.

Dependency arrows mean consumer → dependency:

```text
Gallery → @quadrant-kit facade → patterns / overlays → primitives → foundation
```

The facade only re-exports symbols. Same-layer helpers must remain acyclic; implementation files import lower layers directly rather than the facade. Gallery imports its own page/shared files relatively and Kit only by its public named entry. There is no dependency on any Tasks crate or source directory.

Foundation owns general semantic colors, typography, motion, elevation, layout constants, and generic image properties. Primitives provide small controls; patterns compose navigation/page/settings/window presentation; overlays provide transient feedback and one confirmation layer. Product business composition and platform window actions are consumer responsibilities.

## Component conventions

Use Theme, Typography, and UiConstants for existing styles and logical pixel sizes. Preserve property directions, callback signatures, defaults, keyboard handling, and existing preview inputs during extraction. Prefer composition and std-widgets wrappers for input, selection, scrolling, and IME behavior. Do not introduce new production properties solely to force screenshot states.

Theme.mode, Theme.system_dark, and Theme.ui_font_family are host inputs. dark_mode is derived. Different Slint component instances do not automatically share these globals. Consumers initialize every independent window before showing it, and also coordinate std-widgets Palette. Kit does not detect OS theme, select fonts, start a process, or access settings. Gallery's host owns its system-theme observation, with a light fallback for Unknown.

Static SVG references resolve within the package and are embedded by the consumer build. Images supplied dynamically by consumers are not Kit-owned assets. Keep Microsoft MIT assets separate from GPL source attribution; see PROVENANCE.md.

ModalManager currently has a single shown/title/message/action state and accepted/dismissed callbacks. Escape and Return handling exists. Complete Tab containment, focus restoration, nested modal stacks, and screen-reader behavior are not established. Preserve and document these limits rather than expanding the modal framework during extraction.

## Read a token and a component

`UiConstants.space_4` in `ui/foundation/constants.slint` is an `out` length
property whose value is 4 logical pixels. `Badge` imports that global directly
inside the implementation layer; consumers import Badge through the facade.
Badge's `in` text/kind properties let a caller choose content and semantic color,
while its Text child binds to them. The component has no business model or
callback. See the [Gallery exercise](GALLERY.md#first-exercise-token-to-badge-to-gallery)
for a reversible change to its internal corner radius.

The public baseline protects explicit properties/defaults, callbacks, enums and
bases. An internal rectangle style can change while that baseline still passes;
visual/behavioral review is required as well. The compiled probe catches type
integration, but is not an exhaustive runtime state or accessibility test.

If the entire built checkout is relocated, old helper rlibs can retain the
previous manifest path. From the new Kit root run
`cargo clean -p quadrant-kit -p quadrant-kit-gallery`, then the locked Gallery
build. This rebuilds local package artifacts without replacing source or removing
historical QA directories. Consumer builds resolve the published package in Cargo
storage and must never persist the helper path as a runtime resource setting.
