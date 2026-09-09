// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
// Generated test consumer only; public WindowEvent dispatch, no UI runtime in Kit.
use slint::{
    ComponentHandle,
    platform::{Key, PointerEventButton, WindowEvent},
};
use std::{cell::Cell, rc::Rc, time::Duration};
slint::include_modules!();

fn key(window: &slint::Window, text: slint::SharedString) {
    window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window.dispatch_event(WindowEvent::KeyReleased { text });
}

fn pointer(window: &slint::Window, x: f32, y: f32, down: bool) {
    let position = slint::LogicalPosition::new(x, y);
    window.dispatch_event(WindowEvent::PointerMoved { position });
    window.dispatch_event(if down {
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
    let ui = ButtonCheck::new()?;
    if std::env::args().nth(1).is_none() {
        return Ok(ui.run()?);
    }
    let directory = std::path::PathBuf::from(std::env::args().nth(1).expect("capture directory"));
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
            let window = ui.window();
            let n = step.get();
            let check = |label: &str, ok: bool| {
                println!(
                    "{}: {} (kit={}, native={})",
                    if ok { "PASS" } else { "FAIL" },
                    label,
                    ui.get_kit_count(),
                    ui.get_native_count()
                );
                if !ok {
                    failed.set(true);
                }
            };
            let shot = |name: &str| {
                let pixels = window.take_snapshot().expect("software snapshot");
                let file = std::fs::File::create(directory.join(format!("{name}.png"))).unwrap();
                let mut encoder = png::Encoder::new(file, pixels.width(), pixels.height());
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                encoder
                    .write_header()
                    .unwrap()
                    .write_image_data(pixels.as_bytes())
                    .unwrap();
            };
            match n {
                0 => {
                    check(
                        "empty button keeps native minimum dimensions",
                        ui.get_empty_width() == 32. && ui.get_empty_height() == 32.,
                    );
                    shot("light-idle");
                    window.dispatch_event(WindowEvent::PointerMoved {
                        position: slint::LogicalPosition::new(60., 135.),
                    });
                }
                1 => {
                    shot("light-hover");
                    pointer(window, 60., 135., true);
                }
                2 => {
                    check(
                        "pointer press has native pressed state, no early command",
                        ui.get_kit_pressed() && ui.get_kit_count() == 0,
                    );
                    shot("light-pressed");
                    pointer(window, 60., 135., false);
                }
                3 => {
                    check(
                        "one pointer release gives one command",
                        ui.get_kit_count() == 1 && !ui.get_kit_pressed(),
                    );
                    ui.invoke_focus_kit();
                }
                4 => {
                    check("public focus forwarding", ui.get_kit_focused());
                    shot("light-focus");
                    key(window, Key::Return.into());
                    check("Return gives one command", ui.get_kit_count() == 2);
                }
                5 => {
                    key(window, " ".into());
                    check("Space gives one command", ui.get_kit_count() == 3);
                    ui.set_buttons_enabled(false);
                }
                6 => {
                    pointer(window, 60., 135., true);
                    pointer(window, 60., 135., false);
                    key(window, Key::Return.into());
                    key(window, " ".into());
                    check(
                        "disabled pointer and keys give no command",
                        ui.get_kit_count() == 3 && !ui.get_kit_pressed(),
                    );
                    shot("light-disabled");
                    ui.set_buttons_enabled(true);
                }
                7 => {
                    check(
                        "reenable has no stale press or command",
                        ui.get_kit_count() == 3 && !ui.get_kit_pressed(),
                    );
                    pointer(window, 60., 135., true);
                    pointer(window, 60., 135., false);
                    check(
                        "reenabled pointer gives one command",
                        ui.get_kit_count() == 4,
                    );
                    ui.invoke_focus_native();
                }
                8 => {
                    key(window, Key::Tab.into());
                    check(
                        "Tab moves native to Kit without action",
                        ui.get_kit_focused() && ui.get_kit_count() == 4,
                    );
                }
                9 => {
                    window.dispatch_event(WindowEvent::KeyPressed {
                        text: Key::Shift.into(),
                    });
                    key(window, Key::Tab.into());
                    window.dispatch_event(WindowEvent::KeyReleased {
                        text: Key::Shift.into(),
                    });
                    check(
                        "Shift+Tab returns to native without action",
                        ui.get_native_focused() && ui.get_native_count() == 0,
                    );
                }
                10 => {
                    pointer(window, 60., 135., true);
                    window.dispatch_event(WindowEvent::PointerMoved {
                        position: slint::LogicalPosition::new(610., 510.),
                    });
                    pointer(window, 610., 510., false);
                    check("release outside cancels command", ui.get_kit_count() == 4);
                    ui.set_command_text("Renamed by host".into());
                    ui.set_dangerous(true);
                }
                11 => {
                    check(
                        "programmatic label/danger changes do not invoke",
                        ui.get_kit_count() == 4,
                    );
                    shot("light-danger");
                    ui.set_dark(true);
                }
                12 => {
                    shot("dark-danger");
                    ui.set_buttons_enabled(false);
                }
                13 => {
                    shot("dark-disabled");
                    ui.set_buttons_enabled(true);
                    ui.set_dangerous(false);
                    ui.invoke_focus_kit();
                }
                14 => {
                    shot("dark-focus");
                    key(window, Key::Return.into());
                    check(
                        "theme/danger transitions preserve one action path",
                        ui.get_kit_count() == 5,
                    );
                    ui.set_modal_shown(true);
                }
                15 => {
                    key(window, Key::Escape.into());
                    check(
                        "modal Escape dismisses once",
                        ui.get_dismissed_count() == 1
                            && ui.get_accepted_count() == 0
                            && !ui.get_modal_shown(),
                    );
                }
                16 => {
                    ui.set_modal_shown(true);
                }
                17 => {
                    key(window, Key::Return.into());
                    check(
                        "modal Return accepts once",
                        ui.get_accepted_count() == 1
                            && ui.get_dismissed_count() == 1
                            && !ui.get_modal_shown(),
                    );
                }
                18 => {
                    ui.set_narrow_case(true);
                    ui.set_command_text(
                        "A deliberately long command label / 很长的操作名称".into(),
                    );
                }
                19 => {
                    check(
                        "narrow long labels preserve command counts",
                        ui.get_kit_count() == 5 && ui.get_native_count() == 0,
                    );
                    shot("narrow-long-label");
                    ui.set_narrow_case(false);
                    ui.set_command_text("Kit command".into());
                }
                20 => {
                    pointer(window, 60., 135., true);
                    ui.set_buttons_enabled(false);
                    pointer(window, 60., 135., false);
                }
                21 => {
                    check(
                        "disable during press cancels command",
                        ui.get_kit_count() == 5 && !ui.get_kit_pressed(),
                    );
                    ui.set_buttons_enabled(true);
                }
                22 => {
                    check(
                        "reenable after cancelled press stays idle",
                        ui.get_kit_count() == 5 && !ui.get_kit_pressed(),
                    );
                    pointer(window, 60., 135., true);
                    pointer(window, 60., 135., false);
                    check(
                        "fresh click after cancelled press fires once",
                        ui.get_kit_count() == 6,
                    );
                }
                23 => {
                    println!("RESULT={}", if failed.get() { "FAIL" } else { "PASS" });
                    slint::quit_event_loop().unwrap();
                }
                _ => unreachable!(),
            }
            step.set(n + 1);
        },
    );
    ui.run()?;
    if failure.get() {
        return Err("button runtime assertion failed".into());
    }
    Ok(())
}
