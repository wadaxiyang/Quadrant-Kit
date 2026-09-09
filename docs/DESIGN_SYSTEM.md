# Design system baseline — P0

This records the pre-migration source, not a completed WinUI visual redesign.
Kit source is `737d0aae0975232f520cc1e82640d99e8eb49467`; P0 does not change UI,
theme constants, assets or Gallery chrome. Current 4px control radius, 8px content
radius, Typography 12/14/18/20/28px and Motion 100/160/200ms remain the starting point.

| Reference/environment field | P0 record |
|---|---|
| WinUI 3 Gallery version/commit, screenshot source | NOT_RUN: no versioned same-machine reference was acquired in this phase. No “latest” or pixel-equivalence claim. P3 must acquire a pinned Gallery build/commit and actual captures. |
| Windows | Windows 11 Pro for Workstations, 10.0.26200 build 26200, x86_64 |
| CPU | Intel Core i5-13500 |
| GPU/driver inventory | Intel UHD 770 31.0.101.3616; NVIDIA RTX 5070 Ti 32.0.16.1047; GameViewer virtual adapter 15.6.5.199. Software-render captures do not establish which GPU would service another backend. |
| Toolchain | Rust 1.94.1, MSRV declaration 1.92, edition 2024; Slint/slint-build 1.17.1 |
| Gallery capture policy | Fluent, winit-software, Segoe UI Variable Text requested from system; no bundled fonts; actual font fallback not independently inspected |
| Baseline scene | controls, preview=1 (Gallery medium specimen width), Light, 1040×800 logical, scale 1.0; per-image manifests identify content hash/binary hash. Some Gallery state specimens still use forced preview properties; these are not real input evidence. |
| Native comparison | Independent A/B benchmark has same 820×440 Light window, 100% scale, font and geometry; capture/render availability recorded separately from timings |
| Untested matrix | Full Light/Dark × 100/200/225% × narrow/normal × Chinese/English state matrix; actual monitor transitions; pressed/hover/keyboard-focus input; screen reader |

Use [P0.md](implementation/kit-fluent-v1/P0.md) for actual image/log paths and
results. Old Navigation render evidence is historical and is not relabeled as
this phase's execution. Source review already shows NavigationItemRow background
animation and Toast height/opacity animation; it does not establish visual quality.
Native Button's 150ms animation is upstream-owned. P3 must record token/native
differences, and P6 must separately address host reduction preferences and lifecycle.

## P2 FluentButton delta

The [P2 report](implementation/kit-fluent-v1/P2.md) records the move from custom
button painting/input to the visible public Slint Fluent Button. Current hover,
pressed, focus, disabled, icon tint and animation follow that native control.
Danger is a neutral native surface with a passive red outline, replacing the old
filled red style. The report retains before/after and event-driven state images,
plus the native undersized-label limitation. This is a scoped std/Kit comparison;
no new Microsoft.UI.Xaml runtime/reference, screen-reader or OS chrome comparison
was performed. P0 reference identity and its limitations above remain historical.


## P3 reference identity and visual changes

Reference: Microsoft [WinUI Gallery v2.9.3](https://github.com/microsoft/WinUI-Gallery/releases/tag/v2.9.3),
peeled Git commit `14a4a1a2b8ddc527dc4a7d5f7e743d7c2bc97db7` verified through GitHub
on 2026-09-09. Its release has no downloadable assets; `Get-AppxPackage *WinUI*`
returned no installed reference. An official Store acquisition attempt for product
`9P3JFPWWDZRC` reached package execution but produced no further progress for over
six minutes; the CLI was interrupted and a subsequent Appx query remained empty.
This does not establish a completed installation. Actual WinUI capture/versioned runtime comparison
is NOT_RUN. The pinned source is a reproducible reference identity, not a local
WinUI rendering baseline. WinUI theme/DPI/font/screenshot-source fields are therefore
NOT_RUN. No claim of WinUI pixel equivalence or completed reference acceptance.

Kit captures use Windows build 26200, pinned Slint 1.17.1 Fluent/winit software,
Light/Dark, system Segoe UI Variable Text requested at 14px, 100% simulated scale
unless the image manifest says otherwise. Font fallback and real monitor transitions
remain NOT_RUN. No font file or Microsoft runtime is redistributed.

Before/after sources and retained images are in the P3 report. Icon/segment/window
surfaces now use native backgrounds, focus, disabled tint and 150ms native animation.
Selection is filled native checked presentation; danger stays a passive red outline.
IconButton grows naturally to 44×32 for its 20px icon rather than squeezing native
padding into 32px. No shared Palette output is overwritten to imitate old visuals.

The new Theme.text_disabled is #777777 (Dark) / #8a8a8a (Light) for custom presentation.
SettingRow and SurfaceCard stop dimming entire child subtrees. Default card elevation
remains off; no new shadows/blur or animations are added. Existing Typography
12/14/18/20/28, spacing and 4/8px radii remain. Field errors reserve wrapping height;
headers and metric/empty labels wrap; PageHeader stacks the action below 420px.
FluentIcon retains its offset container and Badge its two presentation elements:
removing those would lose useful layout/semantic behavior rather than remove debt.

Actual P3 Windows Gallery Light/Dark captures confirmed readable native caption
controls after DWM composition. Software snapshots of transparent caption pixels
do not represent that composed result. See the P3 report for source and observations.

## P5A current transient delta

The historical P0 Toast animation observation above remains scoped to that source.
P5A removes Toast height/opacity animations while establishing finite behavior;
P6 owns future bounded transitions. Native Tooltip service retains its own placement
and edge-window capture limits. See P5A for actual images/input and NOT_RUN metrics.
