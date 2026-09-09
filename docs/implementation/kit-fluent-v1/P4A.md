# P4A — native selection

Requested sequence: commit P3, then all P4/P5/P6 units, committing each completed
unit. P3 base: `1f288c93e7fd97fadf47cb84ef44b5f1f47a851a`. Original root SPEC
addition/deletion remain excluded. This unit adds only selection controls and
their current documentation, Gallery, scanner and verification evidence.

## Contract and implementation

FluentCheckBox, FluentSwitch and FluentComboBox directly inherit their public
Slint 1.17.1 native controls. FluentRadioGroup statically re-exports public
RadioGroup through its primitive module, preserving compiler-special RadioButton
children. No native source is copied, patched, hidden or imported privately.
No required host registration, root runtime dependency, assets or toolchain change.

The reviewed snapshot adds exactly four components to the P3 API: 39 public names,
25 visual components; existing declarations/defaults are unchanged. The Gallery
`native-selection` page and current probe use every new export. Native state is
shared using two-way bindings; no shadow checked/index state or activation handler.
PUBLIC_API records inherited properties and native event differences: RadioGroup
selected reports transitions including programmatic/initial checked selection;
CheckBox/Switch toggled and ComboBox selected report user actions. Radio uses
Tab with Space/Enter; arrow selection is not implemented upstream.

The initial subclass-with-children attempt failed generated Rust compilation
(`sp::RadioButton` absent). A model/repeater composition with exposed orientation
also failed generated Rust (`()` cast to i32), retained in
`target/p4a-gallery-build.log`. A build-script-only prototype missed the first
failure because it did not compile the generated runtime. The final static public
re-export passes actual Rust/runtime compilation. The scanner admits only this
verified native component identity, with owner/missing-record negative fixtures.
This re-export is the current native implementation, not a placeholder or a legacy
alias. The current RadioGroup contract is static children; dynamic repeaters are
not accepted based on the failed composition probe.

## Checks and evidence

`target/verify-p4a/result.json` identifies source content
`35214ae851fb399331cad6f65d3bb4735a863cccaf89c8fbed6165322f54cad8`, unchanged throughout
the complete check/capture/performance run. Later changes add this report and one
disabled-radio assertion; production UI is unchanged.

| Command / scenario | Result | Evidence |
|---|---|---|
| cargo fmt --all --check | PASS / 0 | verify-p4a/0.log |
| cargo clippy --workspace --all-targets --all-features --locked -- -D warnings | PASS / 0 | verify-p4a/1.log |
| cargo test --workspace --locked | PASS / 0; 20 tests | verify-p4a/2.log |
| python scripts/check_ui_boundaries.py | PASS / 0 | verify-p4a/3.log |
| python scripts/check_native_reuse.py | PASS / 0 | verify-p4a/4.log |
| python -m unittest discover -s scripts/tests -p "test_*.py" | PASS / 0; 72 tests | verify-p4a/5.log |
| cargo build --locked -p quadrant-kit-gallery | PASS / 0 | verify-p4a/6.log |
| capture_gallery_baseline.py --mode Matrix --destination native-selection | PASS / 0; 40 images | verify-p4a/7.log and visual/ |
| run_perf.py --scenes selection-100 --samples 30 | PASS / 0; 60 release processes | verify-p4a/8.log |

Paths in this table are relative to target/. Runtime selection checks use public
WindowEvent dispatch, not calls to Kit action callbacks. The final runtime report
is `target/button-checks/20260909T162522763625Z/result.json`: PASS, 17 assertions,
four runtime state images. Earlier failed expectations are retained: a
pointer-selected radio did not take focus from Switch, and arrow selection is not
a native feature. Tests now exercise explicit Tab/Space and repeated pointer
selection; no duplicate input implementation was added to satisfy an assumption.

Visual environment: Windows build 26200, Slint 1.17.1 Fluent, winit/software,
requested Segoe UI Variable Text. Light/Dark × 760×520/900×600/1100×720/1440×900 ×
100/125/150/200/225% simulated scaling. Manually inspected compact Light 100%,
900×600 Dark 225% and isolated runtime Dark: native text/state/focus are readable;
lower Gallery controls scroll below the viewport. Transparent native caption
pixels retain the previously recorded software-snapshot composition limitation.
WinUI runtime comparison, physical monitor transitions, IME/reader certification
and non-Windows native input are NOT_RUN.

## Paired performance

Raw data: `target/perf-harness/20260909T161826442822Z/result.json`.
30 native/Kit pairs, alternating order, all samples retained; same external graph
`c4a8357c7edc1abf6220b729c74fdd387f32d2fd861fb89a0ff1ecc970b89534`, release,
820×440, 100%, Light, same font/content/geometry and 100 real CheckBoxes.

| Metric | Native A | Kit B | B−A |
|---|---:|---:|---:|
| Software-buffer readiness p50 ms | 10.4644 | 10.2214 | -0.2430 |
| Software-buffer readiness p95 ms | 11.2945 | 14.4415 | 3.1470 |
| Private Bytes at 2s p50 | 6,627,328 | 6,631,424 | 4,096 |
| Binary bytes | 13,940,736 | 13,942,784 | 2,048 |

Timing and binary differences fit the unchanged provisional allowances. All 60
memory samples are unchanged between 1s and 2s, supporting this short settled
comparison; no full 60s idle claim. Software-buffer readiness is not display
present; AfterRendering/interaction-frame/long-idle CPU remain NOT_RUN. Negative
timing difference is not evidence that the wrapper is faster.

Gate: PASS for the scoped native selection contract, with the explicit native
grammar/keyboard limitations above. Packaging is checked after the authorized
commit. P4B starts only after this unit is committed; P4 is not yet complete.
