# Native reuse

Use public `std-widgets.slint` and builtins from pinned Slint 1.17.1. A visible native
control owns its input; never hide a native proxy beneath a custom replacement,
copy Slint internals, duplicate activation or treat a mere import as reuse.

## Guard and ownership

The [manifest](../scripts/native_reuse_manifest.json) is the exact current inventory:
45 visual records, 42 public components, three private helpers, eight reviewed
exceptions, no pending migration. It is development metadata, not a runtime registry.
The facade remains the sole public export entry.

`python scripts/check_native_reuse.py` shares the fail-closed Slint parser with the
boundary checker. It reconciles coverage, public/private ownership, native reference
closure, direct input owners and literal hidden proxies. Exceptions require a
reviewed declaration/body digest; it is never auto-refreshed. Unsupported syntax or
unknown native exports fail. Tests include aliases, indirect composition, cycles,
missing records, fake reuse and custom-input violations.

| Status | Meaning |
|---|---|
| native-wrapper | One real public native owner, no duplicate input |
| presenter | Passive presentation |
| composed | Children own behavior; no unreviewed direct input |
| reviewed-exception | Missing public capability with a bounded recorded scope |
| custom/pending-migration | Unmigrated ordinary input; none currently allowed |

The scanner is lexical, not a full compiler/dataflow analysis. It cannot prove
dynamic visibility, input behavior, focus, accessibility, performance or whether
an exception is justified. PASS always retains `runtime_verified=false`.

## Current exceptions

| Component / helper | Allowed custom scope |
|---|---|
| SurfaceCard | Optional arbitrary-child action surface; no public Button content slot. Interactive cards use passive children |
| NavigationView | Bounded hierarchy validation, host requests and focus recovery; native search/scroll/toggle retained |
| NavigationItemRow | Row projection, selection/chevron decoration and focus coordination |
| NavigationRowTarget (private) | Flat navigation-only pointer/key/default action, cancellation and keyboard cue |
| NavigationBackButton | Shared borderless navigation action; one focus/pointer target, disabled/cancel and keyboard cue |
| ToastHost | At-most-once dismissal, auto-dismiss/hover lifecycle and bounded presentation |
| ModalManager | Finite confirmation focus/request/restore protocol; not arbitrary ContentDialog |
| TransientLifetime (private) | Bounded opacity/cleanup without input or business state |

Public Button has no flat style in 1.17.1. The explicitly requested navigation
exceptions do not permit a general Button replacement. Gallery imports shared
NavigationBackButton and NavigationView; only geometry and routing stay host-side.
Native Tooltip remains the service for navigation help.

RadioGroup, ListView and TabWidget are verified public native re-exports: wrapper
subclasses break compiler-special child lowering. Their actual generated Rust must
compile. RadioGroup dynamic RadioButton repeaters are unsupported in this version.
Date/Time public structs and builtin model types are separately checked by the
locked compiler. See [API](PUBLIC_API.md) for current contracts and limits.

## Pinned source reference

Inspect the resolved `i-slint-compiler-1.17.1/widgets/fluent/` sources for Button,
style-base, radio group, scrolling, table and picker capabilities. The implementation
uses only public imports; reading upstream code is not permission to vendor it.
[`native_control_probe.slint`](../gallery/ui/native_control_probe.slint) and generated
verification consumers check the actual supported APIs. [Validation](VALIDATION.md)
documents how to run them; [STATUS](STATUS.md) separates runtime/reader evidence.
