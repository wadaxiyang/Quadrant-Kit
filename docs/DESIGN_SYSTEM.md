# Design system

Aim for restrained Fluent desktop presentation using Slint 1.17.1 public controls.
Respect native input/sizing limits and [native reuse](NATIVE_REUSE.md); this is not
Microsoft.UI.Xaml or a claim of pixel equivalence.

| Area | Current rule |
|---|---|
| Theme | Host initializes Theme and Palette from one decision per window; no circular binding or runtime parser |
| Typography | Shared 12/14/18/20/28 hierarchy; host selects fonts and verifies fallback; no bundled system fonts |
| Layout | Shared 4px spacing rhythm and existing 4/8px control/card radii; honor native preferred/minimum sizes |
| Commands | Public native painting/input; danger uses a passive outline; native selected state where available |
| Navigation | Flat rows, a small selection marker, keyboard focus cue, centered compact icons; borderless Back is a reviewed exception |
| Cards and slots | Flat by default; optional limited elevation; host explicitly gates arbitrary child input |
| Icons | Local attributed SVGs at appropriate 16/20/24 sizes; semantic aliases and optical offsets, no runtime asset scan |
| Motion | Native animation stays native-owned; Kit policy is defined in [MOTION](MOTION.md) |

Long labels and error captions need real layout space. IconButton's populated-icon
native size is 44x32, not a forced 32x32. Narrow text can exceed the native minimum;
allow wrapping/scrolling where supported or use shorter visible labels with a full
accessible name. Do not cover native widgets to fake styling or add a second focus ring.

## Visual review

Check Normal/Hover/Pressed/Disabled/Keyboard focus, plus selected and input states
where applicable. Use Light/Dark, Chinese/English, short/long text, normal/narrow
width and representative 100/200/225% scale. Full combinations belong on representative
controls; extend the matrix by risk. Screenshots, real input, reader and physical
monitor transitions are separate evidence categories. See [acceptance](specs/ACCEPTANCE.md).

Historical reference: WinUI Gallery v2.9.3, source commit
`14a4a1a2b8ddc527dc4a7d5f7e743d7c2bc97db7`, identified on 2026-09-09.
No completed local WinUI runtime/reference capture was recorded. Source comparison
does not close runtime visual acceptance. Pinned Slint source references live in
[NATIVE_REUSE](NATIVE_REUSE.md); outcomes and remaining gaps live in [STATUS](STATUS.md).
