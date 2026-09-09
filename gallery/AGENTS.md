# Kit Gallery and verification host

Gallery consumes Kit only through @quadrant-kit. Do not copy Kit implementations
into specimens or import private Kit paths. std-widgets is allowed for labeled
native comparisons, not to count unwrapped controls as completed Kit components.

Use the existing catalog and conditional page creation. The catalog only makes
examples discoverable; it is not the Kit public export registry. New components
are exported statically from ui/kit.slint and instantiated in their examples.
No per-host registration service, startup scan or component factory is needed.

Gallery, API probe and behavior tests follow the current Kit API. When names,
properties, callbacks or defaults change, update current specimens and assertions.
Do not freeze old consumer source or keep legacy pages/aliases/preview flags to
preserve historical call signatures. Keep useful behavior assertions and clear
Breaking changes documentation; do not use API evolution to hide regressions.

Every current public visual component needs a real reachable specimen. Keep
compiled probes distinct from input tests. Use actual/test-driven input for
states; static references must be labeled and Gallery-owned. Do not make every
page resident or add production preview APIs just to simplify screenshots.

Host code owns OS theme observation, fonts and top-level globals. Coordinate Theme
and public Palette per window. Native DWM/AppKit chrome and platform dependencies
remain host-only; do not replace system caption controls with hand-drawn ones.

A Slint surface may not contain OS-composited chrome. Use real window captures for
that scope. Simulated DPI is not a real monitor change; cross compilation is not
native interaction or accessibility validation. Keep static analysis, compilation,
snapshots, input, IME, reader behavior and performance evidence separate.
Record exact source/environment and NOT_RUN items. Never auto-accept screenshots
or rewrite historical reports.

Performance comparisons use matched native and Kit scenes, not a tiny native
window against the full Gallery. Generated current verification consumers may
use a narrowly allowed test-only source path; do not relax Product source rules.
Keep test tooling, capture, polling and performance work out of normal startup.

Keep the pinned Slint/toolchain and renderer choices. Document new CLI options only
after implementation. Run only the requested phase/subphase and stop after the
report. Keep README's editable Mermaid diagram and public-entry explanation in
sync when architecture changes; it must not imply a runtime registration layer.
