# Navigation rebuild Phase 8 — final local validation

Date: 2026-09-07. Predecessor: `ec7685b98044328ecda03ce3e99f05a6484de0d1`.
Authority: the root NavigationView/Gallery rebuild SPEC, Phase 8.

## Finalization

README, PUBLIC_API, GALLERY, CONSUMER_GUIDE, VALIDATION and CHANGELOG now describe
the current catalog, keyboard behavior and unpublished navigation API. The
published extraction candidate and its historical platform/consumer evidence
remain distinct from the current local implementation.

The public baseline is unchanged from the explicit Phase 3 review: 35 names,
240 properties, 20 callbacks, 21 components, six globals, seven enums and one
ten-field struct. No candidate baseline was generated or adopted in this phase.
Two repository tests now compare documented declarations/defaults and probe
coverage with the facade, reconcile counts, and prevent live legacy navigation
identifiers from returning. The existing catalog test covers all 21 visual
exports and their typed routes; there are 25 destinations and eight numeric aliases.

Legacy-name searches found no live SidebarItem implementation, import, instance,
public declaration or runnable example, and no old Catalog filter shell. Remaining
references in the SPEC, changelog, historical phase reports, removal descriptions
and extraction manifest are intentional historical provenance.

No Slint, Gallery runtime, public API, Cargo manifest/lockfile, toolchain, root
Rust locator, resource bytes, asset baseline or licensing changes are included.
The Phase 7 native input observations and 184 render scenes therefore cover the
same UI/runtime source; Phase 8 does not claim a new independent native run.

## Verification

In progress. The reviewed documentation/guard changes are committed before the
full clean-source gate; final command results and source identity follow here.

## Scope

This is local construction, not a release or a Tasks adoption. Remote CI, retained
references, anonymous external-consumer verification and publication are NOT_RUN
for this rebuild. Linux/macOS native runtime, real monitor DPI transitions, full
backend coverage, IME and screen-reader action dispatch remain NOT_RUN for the
current UI. Up/Down/Home/End item traversal is not implemented; modal focus
containment/restoration is unclaimed. Native decorations are retained; custom
chrome remains deferred. Historical extraction evidence is not relabeled.
