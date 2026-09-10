# Compact navigation and shared Back

Based on P8 HEAD `5e3bff0bd20528816f085cf9e4930ef8867f579c` plus retained uncommitted
follow-ups. This supersedes the compact layout and Gallery-local Back described
in NAVIGATION_HEADER_BACK.md. No commit, publication or Tasks change is requested.

All fixes belong to Kit: NavigationView hides the compact search row and keeps
one centered pane toggle; NavigationItemRow removes compact chevrons and their
reserved width, centers all depths, and gives the full row to its primary action.
Expanded indentation and separate chevrons remain. Group icons still request
expansion; destination-group icons invoke, with Left/Right expansion available.
The host retains search text while the native editor is conditionally absent.
The obsolete search-expansion request/focus timer is removed.

NavigationBackButton now owns the borderless navigation action, including keyboard,
pointer, disabled/cancel and accessible default behavior. Its public declarations
are unchanged; the intentional behavior change is documented in PUBLIC_API.
GalleryToolbar imports it from @quadrant-kit and only supplies OS-measured dimensions
and its host callback. The Gallery-local CaptionBack implementation is removed.
This is a narrowly reviewed exception because public Button lacks flat styling,
recorded in the guarded Kit manifest (45 records, eight exceptions); no hidden
native proxy or new runtime dependency is introduced. Native Tooltip remains.

## Verification

- PASS: current public-facade Navigation consumer, 169 assertions including painted
  icon centering, compact full-row hit regions, keyboard expansion, one header and
  retained search state: `target/button-checks/20260910T084930326353Z/`.
- PASS: public foundation consumer, 30 assertions including Back focus/activation:
  `target/button-checks/20260910T084752246533Z/`.
- PASS: actual Gallery toolbar/public Back and Settings consumer, 18 assertions:
  `target/button-checks/20260910T084552839021Z/`.
- PASS: fmt, UI/native boundaries and 88 Python tests:
  `target/verify-navigation-compact/result.json`, logs 0 through 3.
- Initial Gallery build FAIL: the user's running old Gallery locked the output
  executable (os error 5). Its exact-path process was closed gracefully. Build
  retry PASS: `target/verify-navigation-compact/build-retry.log` (31.07 seconds).
  Verification then ran a copied executable, avoiding the build output lock.
- PASS: actual Windows UIA/caption geometry, Back, Settings, compact absence of
  search/chevrons, centered primary/footer action bounds and restored expanded
  chevrons: `target/navigation-compact-window/result.json`. Restored/maximized and
  compact PNGs are whole-window PrintWindow captures; compact manually inspected.
- Binary SHA256: `f9b829bcf51d4f625b7b5f2d7064993b806514b9666e145e9b6d70e96160bff5`.
- One intermediate consumer compile FAIL (element name shadowing the button enum)
  is retained in `target/button-checks/20260910T084529483253Z/`; explicitly naming
  AccessibleRole.button resolved it before the passing runs.
- Clippy/Rust unit suite NOT_RUN again for this Slint-only change; their preceding
  full run is in NAVIGATION_HEADER_BACK.md. Gallery generated Rust was rebuilt.

Independent generated consumers import only the public facade, so the compact/Back
evidence is not a Gallery-only patch.
Native screen-reader, cross-platform and new performance acceptance are NOT_RUN.
Historical P0-P8 limitations and clean-source package prerequisites remain.
