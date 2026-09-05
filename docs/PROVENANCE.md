# Provenance

Extraction source: `https://github.com/wadaxiyang/Quadrant-Tasks` (historical URL `https://github.com/wadaxiyang/Quadrant`), commit `5a2262cd480d639673fa4f5dd406a9c7196361b5`. The audited code commit is `79632ca3dec2decce7f830879575c517f4e2a29e`; only migration records/ignore rules changed between these commits. No product implementation changes intervened.

`scripts/extraction_manifest.json` maps each copied file to its old and new path, source byte hash, extracted byte hash, and modification status. `scripts/asset_manifest.json` records all 32 generic SVG property mappings, byte hashes, MIT license, and recorded upstream commit. New helper, build integration, configuration parsing, probe, scripts, and documentation are Quadrant contributor additions under GPL-3.0-only.

| Source area | Destination and change |
|---|---|
| `ui/kit/foundation` | `ui/foundation`; remove product colors, timer size, Focus breakpoint, product aliases and branding. Rewrite generic image paths. |
| `ui/kit/primitives` | `ui/primitives`; preserve behavior and visual defaults. |
| Generic `ui/kit/patterns`, `overlays` | Corresponding `ui/` layers; retain original GPL/owu headers and derivation notes. Task patterns are excluded. |
| `ui/kit/kit.slint` | `ui/kit.slint`; remove four product exports, retain 28 public names. |
| `ui/gallery` | `gallery/ui`; exclude Inbox page, neutralize product examples, map imports to the facade, and initialize host theme. |
| Gallery Rust main | `gallery/src/main.rs`; preserve snapshot PNG/retry behavior, validate configuration and initialize theme explicitly. |
| Generic SVGs and LICENSE-MIT | `assets/icons`; bytes unchanged, source paths preserved. Product aliases/assets remain in Tasks. |

The SPEC records upstream `owu/wsl-dashboard` commit `948589a255a4bd8a3ff9c3de49e2e13109378fcd` (v0.11.0) for derived UI code, and `microsoft/fluentui-system-icons` commit `4d685f77b2cb8f3f412a74ec8d920c8c91149528` (1.1.339) for Microsoft icons. Original source comments and GPL/MIT notices are preserved. No new upstream downloads or independent SVG-to-upstream byte matching were performed; unknown original upstream filenames are not guessed.

SVG bytes include their original trailing whitespace. `.gitattributes` disables text conversion and whitespace diagnostics only for these audited SVG assets, preserving hashes across Windows checkouts rather than rewriting third-party files for formatting. Other text files use LF.

Tasks retains its branding/native icons, task types, database migrations, IPC, Agent, and packaging. During Phase 1 it also retains its embedded Kit as the unchanged build source. That temporary extraction staging is not permission for permanent duplicated Kit maintenance after cutover.

Kit's final own commit SHA is recorded by Git and later by its consumers, not embedded into the same commit. No release reference or remote publication is claimed by this document.
