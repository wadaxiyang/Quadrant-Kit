# Kit UI implementation

## Layer ownership and static entry

ui/kit.slint is the only supported consumer entry. It statically re-exports current
components and types; it does not instantiate controls, dispatch business events,
adapt old APIs or register components at runtime.

Foundation owns semantic values, typography, motion/elevation recipes, layout
constants and resources. It does not instantiate interactive controls; it may
read public Palette/StyleMetrics outputs, never private Slint palettes.
Primitives own small native wrappers and necessary passive presenters. Patterns
compose page, settings and navigation structures. Overlays own temporary content
and its bounded lifecycle. Patterns and overlays do not import one another.
Use acyclic direct implementation imports downward, never through the facade.

Implement new components in the proper layer and add explicit facade exports.
New features in existing components update that component's current API without
a second registration step. Keep private helpers private. Do not add a duplicate
public-name manifest, universal component factory or unnecessary interface shells.

Compiler-special native controls may use a verified public static re-export when
an extra subclass breaks native child lowering. This is the actual current native
implementation, not a placeholder or old-name alias. Guard the exact supported
native identity and compile the generated runtime, not just a build script.

Tooltip content may be a passive primitive presenter; public Slint Tooltip owns
service behavior. Its API may be simplified as needed, with current callers
updated. Do not create a primitives-to-overlays dependency for ordinary tooltips.

## Current API and behavior

Keep useful public concepts consistent but do not freeze historical signatures.
Properties, directions/types, callbacks, defaults, names and public bases may
change for a clear purpose. Update facade, current docs, probe and affected callers;
remove replaced code and obsolete parameters. Do not maintain deprecated aliases,
legacy modes, old implementations or backward-compatibility adapters.

Use a visible public native control as the actual input owner. Do not keep a
second TouchArea/FocusScope activation implementation around native behavior.
Decoration must not intercept input or create duplicate accessibility nodes.
Do not use opacity=0 to hide a native proxy behind a custom replacement.

Delete screenshot-only preview parameters when migrating a component; Gallery
uses real/test-driven input or its own clearly labeled static references.
When old parameters do not map to native capabilities, redesign the current API
rather than adding no-op fields or compatibility renderers. Required functionality
and correct interaction still matter; breaking APIs is not permission to fake them.

Document one state-ownership protocol. Do not keep conflicting old/new state or
silently change controlled inputs. User events fire according to the current
contract, not again on animation completion or programmatic assignment.
Read-only Palette colors are not writable per-control style overrides.

Custom Badge/Card/InfoBar geometry or missing compound behavior is allowed only
within recorded scope; composites still reuse native editing, scrolling and actions.
Slots have explicit layout, focus and enabled rules. Opacity is not disabling,
and a modal scrim does not prove focus containment or restoration.

## Cost, lifetime and checks

Reuse tokens and native behavior first. Add shared visual helpers only for real
benefit without unnecessary per-instance layers. Default ordinary surfaces to no
shadow; avoid broad clips, blur effects and whole-page geometry animations.
Use native ListView structures needed for virtualization and verify behavior.
Keep NavigationView model limits and host-owned routing boundaries explicit.

Preserve native-owned motion. Kit transitions are cancelable and respect their
effective policy. Hidden components must not keep unnecessary input, animation or
timer work; exit subtrees live only until bounded cleanup. Test rapid reversals.

Update current API/probe, native reuse, status and behavior/visual evidence.
Snapshots describe the current version and may change with documented API changes;
never auto-refresh them to hide unrelated failures. Do not create frozen legacy
consumer tests. Extend the existing parser with tests for new syntax.
