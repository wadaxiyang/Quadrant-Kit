// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle,
    platform::{Key, PointerEventButton, WindowEvent},
};
use std::{cell::Cell, rc::Rc, time::Duration};
slint::include_modules!();

fn key(w: &slint::Window, text: slint::SharedString) {
    w.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    w.dispatch_event(WindowEvent::KeyReleased { text });
}
fn pointer(w: &slint::Window, x: f32, y: f32, down: bool) {
    let position = slint::LogicalPosition::new(x, y);
    w.dispatch_event(WindowEvent::PointerMoved { position });
    w.dispatch_event(if down {
        WindowEvent::PointerPressed {
            position,
            button: PointerEventButton::Left,
        }
    } else {
        WindowEvent::PointerReleased {
            position,
            button: PointerEventButton::Left,
        }
    });
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()?;
    let ui = FoundationCheck::new()?;
    let Some(path) = std::env::args().nth(1) else {
        return Ok(ui.run()?);
    };
    let directory = std::path::PathBuf::from(path);
    std::fs::create_dir_all(&directory)?;
    let failure = Rc::new(Cell::new(false));
    let failed = failure.clone();
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(250),
        move || {
            let ui = weak.unwrap();
            let w = ui.window();
            let check = |name: &str, ok: bool| {
                println!("{}: {name}", if ok { "PASS" } else { "FAIL" });
                if !ok {
                    failed.set(true);
                }
            };
            let shot = |name: &str| {
                let pixels = w.take_snapshot().expect("software snapshot");
                let mut encoder = png::Encoder::new(
                    std::fs::File::create(directory.join(format!("{name}.png"))).unwrap(),
                    pixels.width(),
                    pixels.height(),
                );
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                encoder
                    .write_header()
                    .unwrap()
                    .write_image_data(pixels.as_bytes())
                    .unwrap();
            };
            match step.get() {
                0 => {
                    check(
                        "wrapped error reserves more than one caption line",
                        ui.get_error_height() > 60.,
                    );
                    shot("light-layout");
                    pointer(w, 40., 56., true);
                }
                1 => {
                    check(
                        "icon press does not activate early",
                        ui.get_icon_pressed() && ui.get_icons() == 0,
                    );
                    shot("icon-pressed");
                    pointer(w, 40., 56., false);
                }
                2 => {
                    check("icon release activates once", ui.get_icons() == 1);
                    ui.invoke_focus_icon();
                    key(w, Key::Return.into());
                    check("icon Return once", ui.get_icons() == 2);
                    key(w, Key::Space.into());
                    check("icon Space once", ui.get_icons() == 3);
                }
                3 => {
                    check("native focus forwarded", ui.get_icon_focused());
                    shot("icon-focus");
                    pointer(w, 160., 56., true);
                    pointer(w, 160., 56., false);
                    check(
                        "segment only requests selection",
                        ui.get_segments() == 1 && !ui.get_selected_value(),
                    );
                    ui.set_selected(true);
                }
                4 => {
                    check(
                        "host selection change emits no command",
                        ui.get_selected_value() && ui.get_segments() == 1,
                    );
                    ui.invoke_focus_segment();
                    key(w, Key::Space.into());
                    check(
                        "selected segment never toggles itself",
                        ui.get_selected_value() && ui.get_segments() == 2,
                    );
                    shot("selected");
                    ui.set_controls_enabled(false);
                }
                5 => {
                    pointer(w, 40., 56., true);
                    pointer(w, 40., 56., false);
                    key(w, Key::Return.into());
                    check(
                        "disabled pointer and keyboard ignored",
                        ui.get_icons() == 3 && ui.get_segments() == 2,
                    );
                    shot("disabled");
                    ui.set_controls_enabled(true);
                }
                6 => {
                    check(
                        "reenabling emits no stale request",
                        ui.get_icons() == 3 && ui.get_segments() == 2,
                    );
                    pointer(w, 160., 56., true);
                    pointer(w, 160., 56., false);
                    check(
                        "reenabled segment accepts fresh click",
                        ui.get_segments() == 3,
                    );
                    pointer(w, 40., 56., true);
                }
                7 => {
                    pointer(w, 700., 230., false);
                    check("release outside cancels icon", ui.get_icons() == 3);
                    ui.invoke_focus_window();
                    key(w, Key::Return.into());
                    check("window action forwards once", ui.get_windows() == 1);
                    ui.invoke_focus_back();
                    key(w, Key::Space.into());
                    check("back action forwards once", ui.get_backs() == 1);
                    ui.invoke_focus_pane();
                    key(w, Key::Return.into());
                    check("pane action forwards once", ui.get_panes() == 1);
                }
                8 => {
                    ui.set_field_text("Host".into());
                    ui.set_area_text("Host area".into());
                }
                9 => {
                    check(
                        "programmatic text is not edited",
                        ui.get_field_edits() == 0 && ui.get_area_edits() == 0,
                    );
                    ui.invoke_focus_field();
                    key(w, Key::End.into());
                    key(w, "中".into());
                    check(
                        "native field unicode edit",
                        ui.get_field_text() == "Host中" && ui.get_field_edits() == 1,
                    );
                    key(w, Key::Return.into());
                    check(
                        "native field accepted exactly once",
                        ui.get_field_accepts() == 1,
                    );
                }
                10 => {
                    ui.invoke_focus_area();
                    key(w, Key::End.into());
                    key(w, Key::Return.into());
                    key(w, "文".into());
                    check(
                        "native area handles newline and unicode",
                        ui.get_area_text().contains('\n')
                            && ui.get_area_text().contains('文')
                            && ui.get_area_edits() >= 2,
                    );
                    shot("edited");
                    ui.set_controls_enabled(false);
                }
                11 => {
                    let text = ui.get_area_text();
                    key(w, "X".into());
                    check("disabled editor ignores typing", ui.get_area_text() == text);
                    ui.invoke_focus_slot();
                    key(w, Key::Space.into());
                    check("disabled slot ignores command", ui.get_slots() == 0);
                    ui.set_controls_enabled(true);
                }
                12 => {
                    ui.invoke_focus_slot();
                    key(w, Key::Space.into());
                    check("enabled slot commands once", ui.get_slots() == 1);
                    ui.invoke_focus_card();
                    key(w, Key::Return.into());
                    check("optional card command once", ui.get_cards() == 1);
                    ui.set_interactive_card(false);
                }
                13 => {
                    pointer(w, 80., 580., true);
                    pointer(w, 80., 580., false);
                    key(w, Key::Return.into());
                    check("passive card has no action", ui.get_cards() == 1);
                    ui.set_interactive_card(true);
                }
                14 => {
                    pointer(w, 80., 580., true);
                    pointer(w, 80., 580., false);
                    check("recreated card helper acts once", ui.get_cards() == 2);
                    ui.set_controls_enabled(false);
                }
                15 => {
                    ui.invoke_focus_card();
                    key(w, Key::Space.into());
                    pointer(w, 80., 580., true);
                    pointer(w, 80., 580., false);
                    check("disabled card rejects actions", ui.get_cards() == 2);
                    ui.set_controls_enabled(true);
                    ui.set_dark(true);
                    ui.set_toast_shown(true);
                }
                16 => {
                    shot("dark-toast");
                    pointer(w, 640., 677., true);
                    pointer(w, 640., 677., false);
                    check("toast body is not a dismiss action", ui.get_dismissals() == 0);
                    // P5A's corrected native tooltip sizing leaves the close
                    // button at the right edge of this 400 + 336 px specimen.
                    pointer(w, 706., 677., true);
                    pointer(w, 706., 677., false);
                    check(
                        "toast native dismiss forwards once",
                        ui.get_dismissals() == 1,
                    );
                }
                17 => {
                    check(
                        "theme updates do not synthesize commands",
                        ui.get_icons() == 3 && ui.get_segments() == 3 && ui.get_windows() == 1,
                    );
                    shot("dark-layout");
                    println!("RESULT={}", if failed.get() { "FAIL" } else { "PASS" });
                    slint::quit_event_loop().unwrap();
                }
                _ => unreachable!(),
            }
            step.set(step.get() + 1);
        },
    );
    ui.run()?;
    if failure.get() {
        std::process::exit(1);
    }
    Ok(())
}
