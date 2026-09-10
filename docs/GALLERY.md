# Gallery

Build/run with `cargo build --locked -p quadrant-kit-gallery` / `cargo run --locked
-p quadrant-kit-gallery`. Gallery demonstrates current public Kit contracts; it
does not register components or provide a required application runtime.

## Pages and interaction

The [catalog](../gallery/catalog.tsv) is the source for 40 entries / 33 destinations
and coverage of all 42 public visual components. Home and All components use the
same catalog. Search matches titles, public names and keywords. Settings is fixed
in the navigation footer and contains theme and preview-width controls.

Every page uses GalleryPage with one content ScrollView. Source/details starts
collapsed and uses a selectable read-only native editor. Snippets illustrate
integration, including any required host state; they are not generated live code.
Changing pages recreates local state, while host-bound state persists. No page cache.

The shell consumes public NavigationView: expanded mode shows title, search,
hierarchy indentation and separate chevrons. Compact mode has one centered toggle,
centered icons and no search/chevron placeholder. Up/Down/Home/End traverse items;
Left/Right request collapse/expansion; Tab and Enter/Space reach actions. Selection,
expansion, search text and Back history belong to the host. Invalid models reject
both navigation regions. See [API](PUBLIC_API.md#navigationview).

The NavigationView specimen includes controlled primary/footer hierarchies,
invalid models, optional regions and request counters. The foundations specimen
uses the same public Back, toggle and content surface. Other command specimens
also use counters; WindowControlButton examples do not close the real Gallery.

## Native window chrome and shared toolbar

The 32px toolbar uses public NavigationBackButton on the left. Windows supplies
its actual minimize-button dimensions; the OS retains all three caption actions.
The pane toggle/name belong to NavigationView below the toolbar.

Windows extends the client into DWM chrome; native bounds drive exclusions, dragging,
resize edges and maximized work area. Only the DWM caption area is transparent.
macOS uses AppKit traffic lights and full-size content; native Mac verification
is pending. Other platforms keep the normal native titlebar and application toolbar.
All adapters remain Gallery-only. Slint software captures do not contain DWM chrome;
use whole-window PrintWindow captures for Windows caption review.

Gallery initializes Theme/Palette/font before showing the window. A separate
never-shown host observes the system theme without overriding its Palette binding;
Unknown falls back to Light. System notifications and font fallback need platform
evidence. A manual Light/Dark switch is not an OS-theme-transition test.

## Snapshot interface

Existing names remain supported:

| Variable | Accepted values |
|---|---|
| QUADRANT_GALLERY_WIDTH / HEIGHT | Both supplied together, finite positive numbers; otherwise use window defaults |
| QUADRANT_GALLERY_THEME | light, dark, system (case insensitive), default light |
| QUADRANT_GALLERY_PAGE | Optional legacy alias 0–7; 8 is an error |
| QUADRANT_GALLERY_DESTINATION | Stable catalog destination; default Home when both route variables are absent |
| QUADRANT_GALLERY_PREVIEW | Integer 0–2, default 1 |
| QUADRANT_GALLERY_SNAPSHOT | Nonempty output path; absent means interactive run |

Numeric aliases remain explicit and are never renumbered:

| Alias | Destination |
| --- | --- |
| 0 | home |
| 1 | tokens |
| 2 | typography |
| 3 | icons |
| 4 | controls |
| 5 | surfaces |
| 6 | feedback |
| 7 | navigation-view |

The runtime rejects PAGE and DESTINATION supplied together, even when they name
the same page. Python `--page`/`--destination` and PowerShell `-Page`/`-Destination`
follow the same rule. CLI captures discard inherited Gallery environment options
and explicitly set their selected route. No option means Home. Group IDs and
unknown/empty destinations are rejected before opening a window.

```console
python scripts/capture_gallery_baseline.py --mode Smoke --destination home
python scripts/capture_gallery_baseline.py --mode Smoke --destination all-components
python scripts/capture_gallery_baseline.py --mode Smoke --destination navigation-view
python scripts/capture_gallery_baseline.py --mode Catalog
```

Catalog mode captures every metadata destination at normal/minimum sizes in both
Light and Dark (100% scale); omit route options for this mode. Navigation mode
accepts `--destination navigation-view` or legacy `--page 7` and keeps the 17 model
fixtures in both pane modes/themes. This does not add runtime configuration APIs
to Kit. PowerShell accepts the same modes and forwards its explicit route only.

Invalid configuration fails before window construction. Snapshot mode retries
transparent frames within a bound and reports output errors; capture tools impose
a child-process timeout. Reports include source/content/binary identity and scene
settings. `--reuse-existing` reuses matching evidence, never automatically approves
pixels. See [validation](VALIDATION.md) for runtime and native-window probes.

## First exercise: token to Badge to Gallery

1. Read Theme/UiConstants and `ui/primitives/badge.slint`; preserve existing changes.
2. Capture `python scripts/capture_gallery_baseline.py --mode Smoke --destination badge --output-directory target/learning/before`.
3. Temporarily change Badge's `border-radius: 12px` to `UiConstants.space_4`.
4. Run the UI boundary check and capture to `target/learning/modified`; inspect the
   resulting pill-to-rounded-rectangle difference.
5. Restore that line exactly and capture to `target/learning/restored`. Verify source
   and rendered restoration. If keeping a deliberate change, review it normally.

On Windows close the build-output Gallery before rebuilding, or run a copied
executable under target. This exercise needs only Kit, not Tasks.
