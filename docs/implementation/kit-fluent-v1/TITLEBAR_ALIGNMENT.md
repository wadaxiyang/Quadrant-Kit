# Title-bar alignment and navigation name

Subsequent layout refinement: [NAVIGATION_HEADER_BACK.md](NAVIGATION_HEADER_BACK.md).
The measurements below retain this earlier follow-up's exact scope.

Follow-up to the uncommitted navigation/settings work, still based on P8 HEAD
`5e3bff0bd20528816f085cf9e4930ef8867f579c`. No new dependency or public API declaration,
native caption replacement, commit, publication or Tasks change is included.

The mismatch came from a 48px Gallery toolbar containing 40px actions beside
standard-height DWM caption buttons. Microsoft's [title-bar design guidance](https://learn.microsoft.com/en-us/windows/apps/design/basics/titlebar-design)
defines a 32px standard row. The Gallery now uses that height with 32px native
high Back/pane actions centered in their slots. Their width remains the native
44px icon-control minimum; forcing it to 32px offsets/clips the menu icon.
System caption controls remain OS-owned.
The window background, DWM extension and caption hit test consume the actual
toolbar height, including scaled pixels; the old 48px drag band is removed.

"Kit Gallery" uses the existing NavigationView pane_title input, rendered above
search in a 44px left-aligned band with 16px inset. Compact mode hides the heading.
The native window title remains available to the taskbar and accessibility.
NavigationView's reviewed digest changes only for this passive title presentation.
No navigation action or model contract changes in this follow-up.

PASS for the requested layout and navigation-name change.

- Full core checks PASS in `target/verify-caption-alignment/`: fmt, Clippy with
  warnings denied, 21 Rust tests, current API/native boundaries, 88 Python tests
  and Gallery build. The later change only widens the toolbar controls to 44px;
  the final Gallery build passes in `target/caption-final-build-retry.log`.
- Nine settings/toolbar runtime assertions PASS at
  `target/button-checks/20260910T074228036523Z/`, including host state, system theme,
  page recreation and programmatic updates after input.
- `target/caption-final/result.json` records actual Windows geometry, hit testing
  and UIA checks for the final executable. Restored caption-button center deltas
  are 0 physical pixels; maximized deltas are 1.5 physical pixels. All three native
  caption centers return the expected Windows hit codes. The blank top row is
  HTCAPTION, while the left actions and the area below 32px are HTCLIENT.
- Native maximize/restore was exercised through WM_SYSCOMMAND. Foreground-guarded
  OS Space input on the native pane button hides/restores the navigation name.
  UIA verifies one name below the toolbar, 44x32 action bounds, Settings selection,
  named native ComboBoxes, Home/Back and absence of the former theme/width buttons.
- Actual own-window PrintWindow captures were inspected: `restored.png`,
  `maximized.png`, `settings.png` and `compact.png` in `target/caption-final/`.
  These include native caption rendering, unlike the software-only snapshots.

Final executable SHA-256:
`0ab6609ada151fc7b06e295833300b7bba4d1c3c79e4db6d82ada5f36aab078c`.
Windows 11 build 26200, winit/software, pinned Slint 1.17.1, scale 100%.
The tested binary is copied before launching so later builds do not replace an
in-use executable. One earlier build failed during replacement while its test
window was still open; that failure is retained in `target/caption-final-build.log`.
The initial UIA discovery miss and probe-pattern failures remain in the earlier
`target/caption-geometry-*` reports. Main-window discovery now checks both process
and title, rather than only the process containing the separate system-theme host.

The pinned native pane button exposes ExpandCollapsePattern, but its direct UIA
Expand/Collapse call did not actuate the pane in the preflight run. Keyboard Space
is independently verified; this report does not certify that UIA operation or a
screen-reader session. macOS runtime, physical monitor/DPI transitions and new
source packaging are NOT_RUN for this follow-up. No commit is made to bypass the
existing clean-source package guard.
