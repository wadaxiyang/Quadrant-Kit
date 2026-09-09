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
    let ui = ModalCheck::new()?;
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
                0 => { check("initial hidden modal emits no restore",ui.get_restores()==0);ui.invoke_focus_opener();ui.set_shown(true); }
                1 => { shot("initial-cancel-focused");key(w,Key::Return.into()); }
                2 => { check("initial Cancel owns Return and does not accept",ui.get_dismissed_count()==1 && ui.get_accepted_count()==0);check("modal keeps host shown ownership",ui.get_shown());key(w,Key::Return.into());key(w,Key::Escape.into());pointer(w,490.,308.,true);pointer(w,490.,308.,false);check("pending action cannot emit duplicate or alternate request",ui.get_dismissed_count()==1 && ui.get_accepted_count()==0);ui.set_shown(false); }
                3 => { check("logical close restores host opener once",ui.get_restores()==1 && ui.get_opener_focused());pointer(w,80.,36.,true);pointer(w,80.,36.,false);check("hidden modal releases pointer input",ui.get_background_count()==1);ui.set_shown(true); }
                4 => { key(w,Key::Tab.into());shot("tab-primary-focused");key(w,Key::Return.into()); }
                5 => { check("Tab reaches native primary and Return accepts once",ui.get_accepted_count()==1 && ui.get_dismissed_count()==1);ui.set_shown(false); }
                6 => { check("second close restores once",ui.get_restores()==2);ui.set_dark(true);ui.set_shown(true); }
                7 => { w.dispatch_event(WindowEvent::KeyPressed{text:Key::Shift.into()});key(w,Key::Tab.into());w.dispatch_event(WindowEvent::KeyReleased{text:Key::Shift.into()});shot("shift-tab-primary");key(w,Key::Return.into()); }
                8 => { check("ShiftTab cycles to native primary",ui.get_accepted_count()==2 && ui.get_dismissed_count()==1);ui.set_shown(false); }
                9 => { ui.set_shown(true); }
                10 => { for _ in 0..10 {key(w,Key::Tab.into());}check("repeated Tab stays inside fixed actions",!ui.get_opener_focused());key(w,Key::Return.into()); }
                11 => { check("even Tab cycles return to Cancel",ui.get_dismissed_count()==2 && ui.get_accepted_count()==2);ui.set_shown(false); }
                12 => { check("four closes restore four times",ui.get_restores()==4);ui.set_secondary(false);ui.set_shown(true); }
                13 => { shot("single-action");key(w,Key::Return.into()); }
                14 => { check("single-action modal initially focuses primary",ui.get_accepted_count()==3);ui.set_shown(false); }
                15 => { ui.set_shown(true); }
                16 => { for _ in 0..5 {key(w,Key::Tab.into());}w.dispatch_event(WindowEvent::KeyPressed{text:Key::Shift.into()});for _ in 0..5 {key(w,Key::Tab.into());}w.dispatch_event(WindowEvent::KeyReleased{text:Key::Shift.into()});key(w,Key::Space.into()); }
                17 => { check("single-action Tab ShiftTab and native Space remain contained",ui.get_accepted_count()==4 && !ui.get_opener_focused());ui.set_shown(false); }
                18 => { check("six closes restore six times",ui.get_restores()==6);ui.set_secondary(true);ui.set_shown(true); }
                19 => { key(w,Key::Escape.into());check("Escape requests dismissal exactly once",ui.get_dismissed_count()==3 && ui.get_accepted_count()==4);pointer(w,80.,36.,true);pointer(w,80.,36.,false);check("shown scrim blocks background pointer",ui.get_background_count()==1);ui.set_shown(false); }
                20 => { ui.set_shown(true); }
                21 => { ui.set_shown(false); }
                22 => { check("programmatic close only restores focus",ui.get_restores()==8 && ui.get_accepted_count()==4 && ui.get_dismissed_count()==3); }
                23..=42 => { ui.set_shown(step.get()%2==1); }
                43 => { check("ten rapid cycles restore once each with no commands",ui.get_restores()==18 && ui.get_accepted_count()==4 && ui.get_dismissed_count()==3 && ui.get_opener_focused());key(w,Key::Space.into());check("hidden modal releases keyboard input",ui.get_background_count()==2);shot("closed-final"); }
                44 => { println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap(); }
                _ => {}
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
