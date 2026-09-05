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
