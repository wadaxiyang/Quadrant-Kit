# Architecture

Kit is a source-only library. The Rust helper returns `ui/kit.slint` using its
compile-time manifest directory; it has no runtime dependencies. That path belongs
to a consumer build, never runtime settings. The [README diagram](../README.md#架构与组件接入)
is the single editable dependency diagram.

| Layer | Owns | May depend on |
|---|---|---|
| Facade (`ui/kit.slint`) | Current static exports only | All implementation layers |
| Foundation | Tokens, typography, motion policy, local assets, pure recipes | Acyclic foundation helpers; public Slint design outputs |
| Primitives | Native wrappers and passive presenters | Foundation, small acyclic primitive helpers, public Slint |
| Patterns | Navigation, pages, settings and other compositions | Primitives/foundation, acyclic patterns, public Slint |
| Overlays | Temporary content and bounded presentation | Primitives/foundation, acyclic overlays, public Slint |
| Gallery / consumer | Windows, theme detection, fonts, routing, business state | Public facade and host modules |

Patterns and overlays never import one another; implementation never imports back
through the facade or depends on Gallery. Gallery imports Kit only through
`@quadrant-kit`. Same-layer imports remain acyclic. No registry, factory, runtime
theme parser, page cache or host service is required to use a component.

## Component changes

1. Implement in the owning layer using public native controls where available.
2. Add a static facade export only for a new public component/type.
3. Define current inputs/defaults, state ownership, events, focus, disabled and slot rules.
4. Update [API](PUBLIC_API.md), reviewed snapshot/probe, [native manifest](../scripts/native_reuse_manifest.json),
   [status](COMPONENT_STATUS.md), Gallery specimen and relevant tests together.
5. Remove replaced implementations and old aliases. Review intentional API changes;
   CI never adopts a new baseline automatically. See [validation](VALIDATION.md#api-review).

Separate interface files or another wrapper layer are useful only when they remove
real duplication. RadioGroup, ListView and TabWidget retain verified public native
re-exports because an extra subclass breaks compiler-special child lowering.

## Host ownership

Each top-level window initializes Theme, Palette, font and Motion independently.
Kit globals are not a cross-window service. Hosts own OS actions and detection,
navigation history, committed picker values and lifecycle of arbitrary slots.
NavigationView emits requests; Gallery's router is only a consumer of that API.
NavigationBackButton owns shared appearance/input; Gallery supplies caption geometry.

Toast/Modal share a private TransientLifetime presenter under primitives. It owns
bounded opacity/cleanup, never commands or business state. Expander content requires
an explicit host conditional. Details: [API](PUBLIC_API.md), [Motion](MOTION.md).

Static resources are embedded during the consumer build; assets and attribution
remain local. See [consumer setup](CONSUMER_GUIDE.md) and [provenance](PROVENANCE.md).

After relocating a built checkout, stale helper artifacts may contain the old path.
From the new root run `cargo clean -p quadrant-kit -p quadrant-kit-gallery`, then
the locked Gallery build. Do not persist helper paths or change consumer source rules.
