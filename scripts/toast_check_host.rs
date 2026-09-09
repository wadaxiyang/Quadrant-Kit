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
    let ui = ToastCheck::new()?;
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
                0 => { pointer(w,500.,300.,false);ui.invoke_focus_first();ui.set_shown(true); }
                1 => { check("showing toast does not steal focus",ui.get_first_focused());shot("shown"); }
                36 => { println!("DISMISSALS_AFTER_9S={}",ui.get_dismissals());check("auto dismissal requests once while host keeps shown true",ui.get_dismissals()==1);check("toast does not take logical state ownership",ui.get_shown());ui.set_shown(false); }
                37 => { pointer(w,80.,104.,true);pointer(w,80.,104.,false);check("hidden toast does not intercept underlying pointer",ui.get_underneath_actions()==1);ui.invoke_focus_first();key(w,Key::Tab.into());key(w,Key::Tab.into());key(w,Key::Tab.into());check("hidden dismiss action is absent from Tab order",ui.get_first_focused()); }
                38 => { ui.set_shown(true);pointer(w,100.,100.,false); }
                57 => { check("hover pauses timeout beyond four seconds",ui.get_dismissals()==1);ui.set_message("Changed within the same visible cycle".into());pointer(w,500.,300.,false); }
                75 => { check("leaving hover starts one full timeout",ui.get_dismissals()==2);ui.set_shown(false); }
                76 => { ui.set_automatic(false);ui.set_shown(true);ui.set_dark(true); }
                77 => { pointer(w,350.,108.,true);pointer(w,350.,108.,false);pointer(w,350.,108.,true);pointer(w,350.,108.,false); }
                78 => { check("manual close requests once per shown cycle",ui.get_dismissals()==3 && ui.get_shown());shot("dismiss-requested");ui.set_shown(false); }
                79 => { ui.set_shown(true); }
                80 => { pointer(w,350.,108.,true);pointer(w,350.,108.,false); }
                81 => { check("reopening admits a fresh close request",ui.get_dismissals()==4);ui.set_shown(false);ui.invoke_focus_first();pointer(w,650.,416.,false); }
                86 => { shot("native-tooltip-edge");check("native tooltip hover does not steal focus",ui.get_first_focused());pointer(w,500.,300.,false); }
                87 => { shot("tooltip-left");ui.set_tooltip_x(350.);pointer(w,400.,416.,false); }
                88..=107 => { ui.set_shown(step.get()%2==0);if step.get()==94 {shot("native-tooltip-inside");}if step.get()==106 {pointer(w,500.,300.,false);} }
                108 => { check("rapid cycles leave no stale dismissal",ui.get_dismissals()==4 && !ui.get_shown());shot("hidden-final"); }
                109 => { println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap(); }
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
