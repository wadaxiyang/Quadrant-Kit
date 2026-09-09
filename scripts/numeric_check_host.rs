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
    let ui = NumericCheck::new()?;
    let Some(path) = std::env::args().nth(1) else {
        return Ok(ui.run()?);
    };
    let directory = std::path::PathBuf::from(path);
    std::fs::create_dir_all(&directory)?;
    let failure = Rc::new(Cell::new(false));
    let failed = failure.clone();
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let stable = std::cell::RefCell::new(Vec::<u8>::new());
    let slider_count = Cell::new(0);
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
                    shot("light-running");
                    ui.set_slider_value(4.);
                    ui.set_integer_value(6);
                }
                1 => {
                    check(
                        "programmatic values do not emit edits",
                        ui.get_slider_events() == 0
                            && ui.get_edits() == 0
                            && ui.get_slider_value() == 4.
                            && ui.get_integer_value() == 6,
                    );
                    ui.invoke_focus_slider();
                    key(w, Key::RightArrow.into());
                    check(
                        "native slider step",
                        ui.get_slider_value() == 6.
                            && ui.get_slider_events() == 1
                            && ui.get_releases() == 1,
                    );
                    key(w, Key::End.into());
                    check("slider End reaches maximum", ui.get_slider_value() == 10.);
                    key(w, Key::RightArrow.into());
                    check(
                        "slider cannot move beyond maximum",
                        ui.get_slider_value() == 10.,
                    );
                    key(w, Key::Home.into());
                    check("slider Home reaches minimum", ui.get_slider_value() == 0.);
                    pointer(w, 30., 46., true);
                    w.dispatch_event(WindowEvent::PointerMoved {
                        position: slint::LogicalPosition::new(240., 46.),
                    });
                    pointer(w, 240., 46., false);
                    check(
                        "native drag remains bounded and releases once",
                        ui.get_slider_value() == 10. && ui.get_releases() == 5,
                    );
                    slider_count.set(ui.get_slider_events());
                    ui.set_controls_enabled(false);
                }
                2 => {
                    key(w, Key::Home.into());
                    pointer(w, 30., 46., true);
                    pointer(w, 30., 46., false);
                    pointer(w, 192., 116., true);
                    pointer(w, 192., 116., false);
                    check(
                        "disabled numeric input ignored",
                        ui.get_slider_value() == 10.
                            && ui.get_integer_value() == 6
                            && ui.get_slider_events() == slider_count.get()
                            && ui.get_edits() == 0,
                    );
                    ui.set_slider_value(25.);
                    ui.set_integer_value(25);
                }
                3 => {
                    check(
                        "native host assignments are not range-normalized",
                        ui.get_slider_value() == 25.
                            && ui.get_integer_value() == 25
                            && ui.get_edits() == 0,
                    );
                    ui.set_slider_value(4.);
                    ui.set_integer_value(6);
                    ui.set_controls_enabled(true);
                    ui.set_read_only(true);
                }
                4 => {
                    pointer(w, 192., 116., true);
                    pointer(w, 192., 116., false);
                    ui.invoke_focus_spin();
                    key(w, Key::End.into());
                    key(w, "7".into());
                    check(
                        "readonly SpinBox ignores buttons and text",
                        ui.get_integer_value() == 6 && ui.get_edits() == 0,
                    );
                    ui.set_read_only(false);
                }
                5 => {
                    pointer(w, 192., 116., true);
                    pointer(w, 192., 116., false);
                    check(
                        "native integer step",
                        ui.get_integer_value() == 8 && ui.get_edits() == 1,
                    );
                    pointer(w, 192., 116., true);
                    pointer(w, 192., 116., false);
                    pointer(w, 192., 116., true);
                    pointer(w, 192., 116., false);
                    check("SpinBox stops at maximum", ui.get_integer_value() == 10);
                    ui.invoke_focus_spin();
                    key(w, Key::End.into());
                    key(w, Key::Backspace.into());
                    key(w, Key::Backspace.into());
                    key(w, "5".into());
                    key(w, Key::Return.into());
                }
                6 => {
                    check(
                        "native text edits integer value",
                        ui.get_integer_value() == 5,
                    );
                    check("native progress is active", ui.get_animating());
                    ui.set_running(false);
                    ui.invoke_focus_slider();
                    ui.set_dark(true);
                }
                7 => {
                    check(
                        "stopped progress no longer requests indeterminate motion",
                        !ui.get_animating(),
                    );
                }
                8 => {
                    *stable.borrow_mut() = w.take_snapshot().unwrap().as_bytes().to_vec();
                    shot("dark-stopped");
                }
                9 => {}
                10 => {
                    check(
                        "stopped native progress renders a stable frame",
                        *stable.borrow() == w.take_snapshot().unwrap().as_bytes(),
                    );
                    ui.set_progress_visible(false);
                    ui.set_running(true);
                }
                11 => {
                    check(
                        "hidden progress unloads native active presentation",
                        !ui.get_animating(),
                    );
                    shot("hidden");
                    ui.set_progress_visible(true);
                }
                12 => {
                    check(
                        "showing resumes native indeterminate progress",
                        ui.get_animating(),
                    );
                    shot("reopened");
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
