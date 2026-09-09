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
    let ui = InlineCheck::new()?;
    let Some(path) = std::env::args().nth(1) else {
        return Ok(ui.run()?);
    };
    let directory = std::path::PathBuf::from(path);
    std::fs::create_dir_all(&directory)?;
    let failure = Rc::new(Cell::new(false));
    let failed = failure.clone();
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let stopped_ticks = Cell::new(0);
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
            let click=|x,y|{pointer(w,x,y,true);pointer(w,x,y,false);};
            match step.get() {
                0=>{check("initial collapse has no child",ui.get_child_mounts()==0);click(100.,40.);}
                1=>{check("native header requests expansion once",ui.get_expand_requests()==1&&ui.get_requested_expanded());check("ignored expansion remains controlled",!ui.get_expanded());ui.set_expanded(true);}
                2=>{check("accepted expansion mounts child once",ui.get_child_mounts()==1);shot("expanded");click(100.,88.);}
                3=>{check("native slot action works",ui.get_child_commands()==1);ui.invoke_focus_header();key(w,Key::Return.into());}
                4=>{check("header Return requests controlled collapse",ui.get_expand_requests()==2&&!ui.get_requested_expanded()&&ui.get_expanded());ui.invoke_focus_header();ui.set_expanded(false);}
                5=>{stopped_ticks.set(ui.get_child_ticks());check("host collapse restores known header focus",ui.get_header_focused());shot("collapsed");}
                6=>{check("unloaded child timer stops",ui.get_child_ticks()==stopped_ticks.get());click(100.,88.);check("collapsed slot has no active input",ui.get_child_commands()==1);ui.set_header_enabled(false);ui.invoke_focus_header();key(w,Key::Space.into());click(100.,40.);}
                7=>{check("disabled header emits no requests",ui.get_expand_requests()==2);ui.set_header_enabled(true);ui.invoke_focus_header();key(w,Key::Space.into());}
                8=>{check("native Space requests next expansion",ui.get_expand_requests()==3&&ui.get_requested_expanded());ui.set_expanded(true);}
                9=>{check("reopen remounts content",ui.get_child_mounts()==2);click(472.,248.);}
                10=>{check("InfoBar requests one dismissal",ui.get_dismiss_requests()==1);check("InfoBar shown stays host owned",ui.get_info_shown());click(472.,248.);}
                11=>{check("ignored duplicate dismissal suppressed",ui.get_dismiss_requests()==1);ui.set_info_shown(false);}
                12=>{click(472.,248.);check("hidden InfoBar input is gone",ui.get_dismiss_requests()==1);ui.set_info_shown(true);ui.set_closable(false);}
                13=>{click(472.,248.);check("nonclosable InfoBar emits no dismissal",ui.get_dismiss_requests()==1);shot("nonclosable");ui.set_closable(true);}
                14=>{click(472.,248.);}
                15=>{check("next shown cycle admits one dismissal",ui.get_dismiss_requests()==2);ui.set_info_shown(false);ui.invoke_focus_header();ui.set_expanded(false);stopped_ticks.set(ui.get_child_ticks());}
                16=>{stopped_ticks.set(ui.get_child_ticks());}
                17=>{check("closed inline components remain idle",ui.get_child_ticks()==stopped_ticks.get());shot("closed");}
                _=>{println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap();}
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
