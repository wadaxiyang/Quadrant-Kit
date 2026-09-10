// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle,
    platform::{Key, PointerEventButton, WindowEvent},
};
use std::{cell::Cell, rc::Rc, time::Duration};
slint::include_modules!();
fn key(w: &slint::Window, k: Key) {
    let text: slint::SharedString = k.into();
    w.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    w.dispatch_event(WindowEvent::KeyReleased { text });
}
fn pointer(w: &slint::Window, x: f32, y: f32, down: bool) {
    let position = slint::LogicalPosition::new(x, y);
    w.dispatch_event(WindowEvent::PointerMoved { position });
    w.dispatch_event(if down {
        WindowEvent::PointerPressed { position, button: PointerEventButton::Left }
    } else {
        WindowEvent::PointerReleased { position, button: PointerEventButton::Left }
    });
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = GallerySettingsCheck::new()?;
    let directory = std::path::PathBuf::from(std::env::args().nth(1).unwrap());
    std::fs::create_dir_all(&directory)?;
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let failed = Rc::new(Cell::new(false));
    let result = failed.clone();
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
                let pixels = w.take_snapshot().unwrap();
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
            match step.get() {
                0 => {
                    check(
                        "toolbar reserves open title area",
                        ui.get_drag_width() > 600.,
                    );
                    ui.invoke_focus_anchor();
                    key(w, Key::Tab);
                    key(w, Key::DownArrow);
                }
                1 => {
                    check(
                        "native theme selection reaches host and Palette",
                        ui.get_mode() == ThemeMode::Dark && ui.get_dark(),
                    );
                    shot("settings-dark");
                    key(w, Key::UpArrow);
                    key(w, Key::UpArrow);
                }
                2 => {
                    check("System theme selected", ui.get_mode() == ThemeMode::System);
                    ui.set_system_dark(true);
                }
                3 => {
                    check("System follows host OS theme", ui.get_dark());
                    key(w, Key::Tab);
                    key(w, Key::DownArrow);
                }
                4 => {
                    check(
                        "native preview selection reaches host",
                        ui.get_preview() == 2,
                    );
                    ui.set_mounted(false);
                }
                5 => {
                    check(
                        "unmount retains theme and preview",
                        ui.get_mode() == ThemeMode::System && ui.get_preview() == 2,
                    );
                    ui.set_mounted(true);
                }
                6 => {
                    ui.invoke_focus_anchor();
                    key(w, Key::Tab);
                    key(w, Key::DownArrow);
                    key(w, Key::Tab);
                    key(w, Key::UpArrow);
                }
                7 => {
                    check(
                        "recreated controls read retained host values",
                        ui.get_mode() == ThemeMode::Light && ui.get_preview() == 1,
                    );
                    ui.set_preview(0);
                }
                8 => {
                    key(w, Key::DownArrow);
                }
                9 => {
                    check(
                        "programmatic preview update remains reactive",
                        ui.get_preview() == 1,
                    );
                    ui.set_mode(ThemeMode::Dark);
                }
                10 => {
                    ui.invoke_focus_anchor();
                    key(w, Key::Tab);
                    key(w, Key::UpArrow);
                }
                11 => {
                    check(
                        "programmatic theme update remains reactive",
                        ui.get_mode() == ThemeMode::Light && !ui.get_dark(),
                    );
                    shot("settings-light");
                    ui.set_mounted(false);
                }
                12 => {
                    pointer(w, 24., 16., true); pointer(w, 24., 16., false);
                    check("disabled caption Back ignores pointer", ui.get_back_count() == 0);
                    ui.set_back_enabled(true);
                }
                13 => {
                    pointer(w, 24., 16., true); pointer(w, 24., 16., false);
                    check("caption Back clicks once", ui.get_back_count() == 1);
                    pointer(w, 24., 16., true); pointer(w, 100., 60., false);
                    check("caption Back pointer release outside cancels", ui.get_back_count() == 1);
                    key(w, Key::Return);
                    check("caption Back Return activates once", ui.get_back_count() == 2);
                    let space: slint::SharedString = Key::Space.into();
                    for _ in 0..3 { w.dispatch_event(WindowEvent::KeyPressed { text: space.clone() }); }
                    w.dispatch_event(WindowEvent::KeyReleased { text: space.clone() });
                    check("caption Back held Space does not repeat", ui.get_back_count() == 3);
                    w.dispatch_event(WindowEvent::KeyPressed { text: space.clone() });
                    key(w, Key::Escape);
                    w.dispatch_event(WindowEvent::KeyReleased { text: space.clone() });
                    check("caption Back Escape cancels", ui.get_back_count() == 3);
                    w.dispatch_event(WindowEvent::KeyPressed { text: space.clone() });
                    ui.invoke_focus_anchor();
                    w.dispatch_event(WindowEvent::KeyReleased { text: space });
                    check("caption Back focus loss cancels", ui.get_back_count() == 3);
                    pointer(w, 24., 16., true); pointer(w, 24., 16., false);
                    w.dispatch_event(WindowEvent::KeyPressed { text: Key::Space.into() });
                    ui.set_back_enabled(false);
                }
                14 => {
                    w.dispatch_event(WindowEvent::KeyReleased { text: Key::Space.into() });
                    check("caption Back disable cancels held key", ui.get_back_count() == 4);
                    ui.set_back_enabled(true);
                }
                15 => {
                    w.dispatch_event(WindowEvent::KeyReleased { text: Key::Space.into() });
                    check("caption Back reenable does not replay", ui.get_back_count() == 4);
                    println!("RESULT={}", if failed.get() { "FAIL" } else { "PASS" });
                    slint::quit_event_loop().unwrap();
                }
                _ => {}
            }
            step.set(step.get() + 1);
        },
    );
    ui.run()?;
    if result.get() {
        std::process::exit(1);
    }
    Ok(())
}
