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
    let ui = SelectionCheck::new()?;
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
                    shot("light");
                    ui.set_checked(true);
                    ui.set_switched(true);
                    ui.set_index(1);
                }
                1 => {
                    check(
                        "programmatic checked/selection does not emit user actions",
                        ui.get_checks() == 0
                            && ui.get_switches() == 0
                            && ui.get_combos() == 0
                            && ui.get_checked()
                            && ui.get_switched()
                            && ui.get_value() == "Second",
                    );
                    ui.invoke_focus_check();
                    key(w, Key::Space.into());
                    check(
                        "checkbox Space toggles once",
                        !ui.get_checked() && ui.get_checks() == 1,
                    );
                    pointer(w, 30., 40., true);
                    pointer(w, 30., 40., false);
                    check(
                        "checkbox pointer toggles once",
                        ui.get_checked() && ui.get_checks() == 2,
                    );
                    ui.invoke_focus_switch();
                    key(w, Key::Space.into());
                    check(
                        "switch Space toggles once",
                        !ui.get_switched() && ui.get_switches() == 1,
                    );
                    pointer(w, 40., 100., true);
                    pointer(w, 40., 100., false);
                    check(
                        "switch pointer toggles once",
                        ui.get_switched() && ui.get_switches() == 2,
                    );
                    pointer(w, 40., 224., true);
                    pointer(w, 40., 224., false);
                }
                2 => {
                    check(
                        "radio native selection",
                        ui.get_second() && ui.get_radios() == 1,
                    );
                    pointer(w, 40., 224., true);
                    pointer(w, 40., 224., false);
                    ui.invoke_focus_combo();
                    key(w, Key::DownArrow.into());
                }
                3 => {
                    check(
                        "radio repeat selection emits no new transition",
                        ui.get_radios() == 1,
                    );
                    check(
                        "combo native keyboard selection",
                        ui.get_index() == 2 && ui.get_value() == "Third" && ui.get_combos() == 1,
                    );
                    ui.set_controls_enabled(false);
                }
                4 => {
                    ui.invoke_focus_check();
                    key(w, Key::Space.into());
                    pointer(w, 30., 40., true);
                    pointer(w, 30., 40., false);
                    ui.invoke_focus_switch();
                    key(w, Key::Space.into());
                    pointer(w, 40., 100., true);
                    pointer(w, 40., 100., false);
                    ui.invoke_focus_combo();
                    key(w, Key::DownArrow.into());
                    check(
                        "disabled actions ignored",
                        ui.get_checks() == 2 && ui.get_switches() == 2 && ui.get_combos() == 1,
                    );
                    pointer(w, 40., 184., true);
                    pointer(w, 40., 184., false);
                    check(
                        "disabled radio ignores pointer selection",
                        ui.get_radios() == 1 && ui.get_second(),
                    );
                    shot("disabled");
                    ui.set_choices(slint::ModelRc::new(
                        slint::VecModel::<slint::SharedString>::from(vec![]),
                    ));
                }
                5 => {
                    check("empty model gives empty value", ui.get_value().is_empty());
                    ui.set_choices(slint::ModelRc::new(slint::VecModel::from(vec![
                        "Replacement".into(),
                    ])));
                }
                6 => {
                    check(
                        "replacement clamps index and updates value without event",
                        ui.get_index() == 0
                            && ui.get_value() == "Replacement"
                            && ui.get_combos() == 1,
                    );
                    ui.set_controls_enabled(true);
                    ui.set_dark(true);
                }
                7 => {
                    ui.invoke_focus_check();
                    key(w, Key::Space.into());
                    check(
                        "reenabled native input",
                        ui.get_checks() == 3 && !ui.get_checked(),
                    );
                    shot("dark");
                    ui.invoke_select_first();
                }
                8 => {
                    check(
                        "radio programmatic selection is a native transition",
                        ui.get_radios() == 2 && !ui.get_second(),
                    );
                    ui.invoke_focus_switch();
                    key(w, Key::Tab.into());
                }
                9 => {
                    check("Tab enters native radio group", ui.get_radio_focus());
                    key(w, Key::DownArrow.into());
                }
                10 => {
                    check(
                        "native radio does not implement arrow selection",
                        ui.get_radios() == 2 && !ui.get_second(),
                    );
                    key(w, Key::Tab.into());
                    key(w, Key::Space.into());
                }
                11 => {
                    check(
                        "Tab and Space select the second native radio",
                        ui.get_radios() == 3 && ui.get_second(),
                    );
                    shot("radio-keyboard");
                }
                _ => {
                    println!("RESULT={}", if failed.get() { "FAIL" } else { "PASS" });
                    slint::quit_event_loop().unwrap();
                }
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
