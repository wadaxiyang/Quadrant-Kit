# Architecture

Compiler-special native controls may be statically re-exported through a primitive
module when subclassing loses their native child grammar (currently RadioGroup, ListView and TabWidget).
The facade still only re-exports current contracts; this does not create a runtime
registry. The pinned runtime probe and exact native-owner guard cover this path.

The root Rust package returns its own `ui/kit.slint` location using `env!("CARGO_MANIFEST_DIR")`. The returned path exists during consumer compilation; it is not a runtime resource URL and must not be persisted into application settings. Gallery is the only member application in this workspace.

Dependency arrows mean consumer → dependency:

```text
Gallery → @quadrant-kit facade → patterns / overlays → primitives → foundation
```

The facade only re-exports symbols. Same-layer helpers must remain acyclic; implementation files import lower layers directly rather than the facade. Gallery imports its own page/shared files relatively and Kit only by its public named entry. There is no dependency on any Tasks crate or source directory.

Foundation owns general semantic colors, typography, motion, elevation, layout constants, and generic image properties. Primitives provide small controls; patterns compose navigation/page/settings/window presentation; overlays provide transient feedback and one confirmation layer. Product business composition and platform window actions are consumer responsibilities.

## Static entry and ownership (P1)

| Layer | Owns | May depend on | Must not own |
|---|---|---|---|
| Facade | Explicit current exports in ui/kit.slint | All implementation layers | Instances, state, event dispatch, old API adapters |
| Foundation | Globals, constants, resources, pure recipes | Acyclic foundation helpers; public Palette/StyleMetrics outputs when needed | Visible/input instances, timers, host detection |
| Primitives | Native wrappers and passive presenters | Foundation; small acyclic same-layer helpers; public Slint | Routing, platform/window code, generic input framework |
| Patterns | Navigation, page/settings composition, slot protocols | Primitives/foundation; acyclic patterns; public Slint | Overlays imports, domain models, page caches/history |
| Overlays | Temporary content, bounded visibility/focus requests | Primitives/foundation; acyclic overlays; public Slint | Patterns imports, notification services/modal stacks |
| Gallery/host | Data and event handling, pages, system theme/font, window lifecycle | Public facade and std widgets; host-specific modules | Private Kit paths or copies of Kit components |

The README Mermaid is the editable allowed-dependency diagram, not an instance or
event graph. Foundation currently has no std import; its public design-output edge
is a permitted future capability. Same-layer imports must be acyclic, and patterns
and overlays never depend on each other. Component API declarations may live with
their implementation: separate interface files and another wrapper layer are not
required merely for modularity.

Implement a component in its owning layer, explicitly export its public names at
the facade, then instantiate it through @quadrant-kit. Only callers that need it
change. Adding a member to an existing component does not register it again. The
Gallery catalog indexes examples, the native manifest records development review,
and the API snapshot checks current consistency; none is a runtime registry or a
second source of public export truth. Public type re-exports are restricted to
verified upstream contracts by the scanner; native capability probes complement it.

P1 changes no public API or visual implementation. The old extraction freeze is
superseded by the current-version policy. Planned controls and exact substage
ownership are in COMPONENT_STATUS.md; current custom-input exceptions and their
review scope are in NATIVE_REUSE.md.

## Component conventions

Use Theme, Typography, and UiConstants for semantic styles and logical sizes. APIs may change for a concrete benefit with synchronized current callers, docs, probes and a reviewed snapshot. Each version contains only its current implementation, without old-name aliases, adapters or duplicate behavior branches. Reuse public std-widgets/builtins for native behavior; migrate existing command debt in its assigned phase and remove preview-only inputs at that time. Do not introduce production properties solely to force screenshot states.

Theme.mode, Theme.system_dark, and Theme.ui_font_family are host inputs. dark_mode is derived. Different Slint component instances do not automatically share these globals. Consumers initialize every independent window before showing it, and also coordinate std-widgets Palette. Kit does not detect OS theme, select fonts, start a process, or access settings. Gallery's host owns its system-theme observation, with a light fallback for Unknown.

Static SVG references resolve within the package and are embedded by the consumer build. Images supplied dynamically by consumers are not Kit-owned assets. Keep Microsoft MIT assets separate from GPL source attribution; see PROVENANCE.md.

ModalManager currently has a single shown/title/message/action state and accepted/dismissed callbacks. Escape and Return handling exists. Complete Tab containment, focus restoration, nested modal stacks, and screen-reader behavior are not established. Its finite confirmation contract is addressed in P5B; P1 does not claim those missing capabilities.

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
