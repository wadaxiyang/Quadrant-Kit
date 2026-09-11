// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle,
    platform::{Key, PointerEventButton, WindowEvent},
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::Duration,
};
slint::include_modules!();

fn key(window: &slint::Window, text: slint::SharedString) {
    window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window.dispatch_event(WindowEvent::KeyReleased { text });
}

fn select_all(window: &slint::Window) {
    key(window, Key::End.into());
    window.dispatch_event(WindowEvent::KeyPressed {
        text: Key::Shift.into(),
    });
    key(window, Key::Home.into());
    window.dispatch_event(WindowEvent::KeyReleased {
        text: Key::Shift.into(),
    });
}

fn click_field(window: &slint::Window) {
    click_at(window, 80., 66.);
}

fn click_at(window: &slint::Window, x: f32, y: f32) {
    let position = slint::LogicalPosition::new(x, y);
    window.dispatch_event(WindowEvent::PointerMoved { position });
    window.dispatch_event(WindowEvent::PointerPressed {
        position,
        button: PointerEventButton::Left,
    });
    window.dispatch_event(WindowEvent::PointerReleased {
        position,
        button: PointerEventButton::Left,
    });
}

fn capture_region(
    window: &slint::Window,
    directory: &std::path::Path,
    name: &str,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Vec<u8> {
    let pixels = window.take_snapshot().expect("software snapshot");
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
    (y..y + height)
        .flat_map(|row| {
            let start = (row * pixels.width() as usize + x) * 4;
            pixels.as_bytes()[start..start + width * 4].iter().copied()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()?;
    let ui = TextFieldCheck::new()?;
    let Some(path) = std::env::args().nth(1) else {
        return Ok(ui.run()?);
    };
    let directory = std::path::PathBuf::from(path);
    std::fs::create_dir_all(&directory)?;
    let failure = Rc::new(Cell::new(false));
    let failed = failure.clone();
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let reference = RefCell::new(Vec::<u8>::new());
    let masked_eye = RefCell::new(Vec::<u8>::new());
    let visible_eye = RefCell::new(Vec::<u8>::new());
    let long_password = "Demo-Wide-Password-123!".repeat(32);
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(300),
        move || {
            let ui = weak.unwrap();
            let w = ui.window();
            // Only assertion names/counters are logged. All fixture values are fictional.
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
                let crop = |top: usize| {
                    (top..top + 36)
                        .flat_map(|y| {
                            let start = (y * pixels.width() as usize + 24) * 4;
                            pixels.as_bytes()[start..start + 320 * 4].iter().copied()
                        })
                        .collect::<Vec<u8>>()
                };
                let kit = crop(50);
                check(
                    "Kit rendering equals matched native LineEdit",
                    kit == crop(110),
                );
                kit
            };
            let eye = |name: &str| {
                capture_region(
                    w,
                    &directory,
                    name,
                    24 + ui.get_editor_width() as usize - 28,
                    58,
                    16,
                    20,
                )
            };
            let text_region = |name: &str| capture_region(w, &directory, name, 36, 58, 100, 20);
            match step.get() {
                0 => {
                    ui.invoke_park_focus();
                }
                1 => {
                    check("default input type is text", ui.get_text_mode());
                    *reference.borrow_mut() = shot("default-text");
                    ui.set_value("Fake-only-987!".into());
                }
                2 => {
                    check(
                        "ordinary text renders actual glyph differences",
                        shot("ordinary-changed") != *reference.borrow(),
                    );
                    check(
                        "programmatic text assignment emits no user callbacks",
                        ui.get_edits() == 0 && ui.get_accepts() == 0,
                    );
                    ui.invoke_focus_field();
                    key(w, Key::End.into());
                    key(w, "Q".into());
                    check(
                        "default mode editing preserves actual value",
                        ui.get_value() == "Fake-only-987!Q"
                            && ui.get_last_edited() == ui.get_value()
                            && ui.get_edits() == 1,
                    );
                    key(w, Key::Return.into());
                    check(
                        "default mode submits actual value once",
                        ui.get_last_accepted() == ui.get_value() && ui.get_accepts() == 1,
                    );
                    ui.invoke_park_focus();
                    ui.invoke_password_mode();
                    ui.set_value("Demo-only-123!".into());
                }
                3 => {
                    check("password input type is exposed", ui.get_password_active());
                    *reference.borrow_mut() = shot("password-light");
                    ui.set_value("Fake-only-987!".into());
                }
                4 => {
                    check(
                        "equal-length passwords render identical masked glyphs",
                        shot("password-light-changed") == *reference.borrow(),
                    );
                    check(
                        "password binding retains actual value without edits",
                        ui.get_value() == "Fake-only-987!"
                            && ui.get_edits() == 1
                            && ui.get_accepts() == 1,
                    );
                    ui.set_dark(true);
                }
                5 => {
                    *reference.borrow_mut() = shot("password-dark");
                    ui.set_value("Demo-only-123!".into());
                }
                6 => {
                    check(
                        "dark passwords retain native masking",
                        shot("password-dark-changed") == *reference.borrow(),
                    );
                    ui.invoke_focus_field();
                    key(w, Key::End.into());
                    key(w, "中".into());
                    check(
                        "password focus forwards Unicode editing with actual callback",
                        ui.get_value() == "Demo-only-123!中"
                            && ui.get_last_edited() == ui.get_value()
                            && ui.get_edits() == 2,
                    );
                    key(w, Key::Return.into());
                    check(
                        "password submission returns actual content once",
                        ui.get_last_accepted() == ui.get_value() && ui.get_accepts() == 2,
                    );
                    select_all(w);
                    key(w, "Fake-replaced!".into());
                    check(
                        "native selection replaces actual password",
                        ui.get_value() == "Fake-replaced!"
                            && ui.get_last_edited() == ui.get_value()
                            && ui.get_edits() == 3,
                    );
                    ui.set_editable(false);
                }
                7 => {
                    click_field(w);
                    ui.invoke_focus_field();
                    key(w, "X".into());
                    key(w, Key::Return.into());
                    key(w, Key::Backspace.into());
                    check(
                        "disabled password rejects pointer keyboard edits and submit",
                        ui.get_value() == "Fake-replaced!"
                            && ui.get_edits() == 3
                            && ui.get_accepts() == 2,
                    );
                    ui.invoke_park_focus();
                }
                8 => {
                    shot("password-disabled");
                    ui.set_value("".into());
                }
                9 => {
                    shot("password-empty-disabled");
                    check(
                        "programmatic clear while disabled emits no callbacks",
                        ui.get_value().is_empty() && ui.get_edits() == 3 && ui.get_accepts() == 2,
                    );
                    ui.set_editable(true);
                }
                10 => {
                    shot("password-empty-enabled");
                    check(
                        "reenabling emits no stale callback",
                        ui.get_edits() == 3 && ui.get_accepts() == 2,
                    );
                    click_field(w);
                    key(w, "Demo-new!".into());
                    check(
                        "pointer focus permits fresh password edit",
                        ui.get_value() == "Demo-new!" && ui.get_edits() == 4,
                    );
                    select_all(w);
                    key(w, Key::Backspace.into());
                    check(
                        "user clear reports empty actual value once",
                        ui.get_value().is_empty()
                            && ui.get_last_edited().is_empty()
                            && ui.get_edits() == 5,
                    );
                    key(w, Key::Return.into());
                    check(
                        "empty submit returns empty value once",
                        ui.get_last_accepted().is_empty() && ui.get_accepts() == 3,
                    );
                    ui.invoke_park_focus();
                }
                11 => {
                    check("focus can leave password field", ui.get_parked());
                    key(w, "Z".into());
                    check(
                        "unfocused password does not receive text",
                        ui.get_value().is_empty() && ui.get_edits() == 5,
                    );
                    ui.invoke_focus_field();
                    key(w, "Demo-return!".into());
                    check(
                        "programmatic focus can return to password",
                        ui.get_value() == "Demo-return!" && ui.get_edits() == 6,
                    );
                    ui.set_dark(false);
                    ui.set_editor_width(160.);
                    ui.set_value(long_password.clone().into());
                    key(w, Key::End.into());
                }
                12 => {
                    *reference.borrow_mut() = text_region("long-masked");
                    *masked_eye.borrow_mut() = eye("long-masked-eye");
                    click_at(w, 164., 68.);
                }
                13 => {
                    check(
                        "native eye reveals long password at minimum width",
                        text_region("long-visible") != *reference.borrow(),
                    );
                    *visible_eye.borrow_mut() = eye("long-visible-eye");
                    check(
                        "native eye switches its icon",
                        *visible_eye.borrow() != *masked_eye.borrow(),
                    );
                    check(
                        "reveal preserves actual value and emits no edit or submit",
                        ui.get_value() == long_password.as_str()
                            && ui.get_edits() == 6
                            && ui.get_accepts() == 3
                            && ui.get_password_active(),
                    );
                    key(w, Key::Home.into());
                }
                14 => {
                    check(
                        "long text scrolling to start does not overlap eye",
                        eye("long-visible-start") == *visible_eye.borrow(),
                    );
                    key(w, Key::End.into());
                }
                15 => {
                    check(
                        "long text scrolling to end does not overlap eye",
                        eye("long-visible-end") == *visible_eye.borrow(),
                    );
                    click_at(w, 164., 68.);
                }
                16 => {
                    check(
                        "second native eye click restores masking",
                        eye("long-hidden-again") == *masked_eye.borrow(),
                    );
                    click_at(w, 164., 68.);
                    ui.invoke_park_focus();
                }
                17 => {
                    ui.invoke_focus_field();
                    key(w, Key::End.into());
                }
                18 => {
                    check(
                        "focus loss resets native password reveal",
                        eye("long-refocused-hidden") == *masked_eye.borrow(),
                    );
                    ui.set_editor_width(320.);
                    ui.set_dark(true);
                }
                19 => {
                    *reference.borrow_mut() = text_region("wide-dark-hidden");
                    *masked_eye.borrow_mut() = eye("wide-dark-hidden-eye");
                    click_at(w, 324., 68.);
                }
                20 => {
                    check(
                        "native eye reveals in dark wide editor",
                        text_region("wide-dark-visible") != *reference.borrow(),
                    );
                    *visible_eye.borrow_mut() = eye("wide-dark-visible-eye");
                    key(w, Key::Home.into());
                }
                21 => {
                    check(
                        "wide dark long text does not overlap eye",
                        eye("wide-dark-visible-start") == *visible_eye.borrow(),
                    );
                    click_at(w, 324., 68.);
                }
                22 => {
                    check(
                        "wide dark eye hides again without changing value",
                        eye("wide-dark-hidden-again") == *masked_eye.borrow()
                            && ui.get_value() == long_password.as_str()
                            && ui.get_edits() == 6
                            && ui.get_accepts() == 3,
                    );
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
