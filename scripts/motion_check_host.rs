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
    let ui = MotionCheck::new()?;
    let Some(path) = std::env::args().nth(1) else {
        return Ok(ui.run()?);
    };
    let directory = std::path::PathBuf::from(path);
    std::fs::create_dir_all(&directory)?;
    let failure = Rc::new(Cell::new(false));
    let failed = failure.clone();
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let background_red = Cell::new(255u8);
    let timer = slint::Timer::default();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_millis(40),
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
                0=>{check("initial hidden has no transition",!ui.get_toast_presented()&&!ui.get_modal_presented()&&!ui.get_toast_animating());let pixels=w.take_snapshot().unwrap();background_red.set(pixels.as_slice()[44*pixels.width() as usize+22].r);ui.set_toast_shown(true);}
                1=>{check("toast entering retains presentation",ui.get_toast_presented()&&ui.get_toast_animating());let pixels=w.take_snapshot().unwrap();let red=pixels.as_slice()[44*pixels.width() as usize+22].r;println!("PIXEL entering_red={red} background_red={}",background_red.get());check("entering opacity produces intermediate pixels",red>0&&red<background_red.get());shot("toast-entering");}
                5=>{check("toast settles visible",ui.get_toast_presented()&&!ui.get_toast_animating());shot("toast-visible");ui.set_toast_shown(false);}
                6=>{check("logical close keeps only bounded exit presentation",ui.get_toast_presented()&&ui.get_toast_animating());click(345.,45.);check("exit does not intercept background pointer",ui.get_background_commands()==1&&ui.get_toast_dismissals()==0);shot("toast-exiting");ui.set_toast_shown(true);}
                7=>{check("rapid reopen cancels stale exit",ui.get_toast_presented()&&ui.get_toast_animating());click(350.,45.);}
                8=>{check("reopened native close requests once",ui.get_toast_dismissals()==1);key(w,Key::Space.into());key(w,Key::Space.into());check("motion does not duplicate close requests",ui.get_toast_dismissals()==1);ui.set_toast_shown(false);}
                9=>{ui.set_reduced(true);}
                10=>{check("reduce during exit cleans immediately",!ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_toast_shown(true);}
                11=>{check("reduced motion opens directly stable",ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_motion_enabled(false);ui.set_reduced(false);}
                12=>{ui.set_toast_shown(false);}
                13=>{check("disabled motion closes directly hidden",!ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_motion_enabled(true);ui.invoke_focus_opener();}
                14=>{ui.set_modal_shown(true);}
                15=>{check("modal entering presentation is active",ui.get_modal_presented()&&ui.get_modal_animating());shot("modal-entering");key(w,Key::Return.into());}
                16=>{check("native Cancel works during entry",ui.get_modal_dismissals()==1&&ui.get_modal_accepts()==0);key(w,Key::Return.into());check("modal request is not duplicated by motion",ui.get_modal_dismissals()==1);ui.set_modal_shown(false);}
                17=>{check("logical modal close restores focus immediately",ui.get_restores()==1&&ui.get_opener_focused());shot("modal-exiting");click(480.,36.);check("exiting scrim does not block background",ui.get_background_commands()==2);ui.set_modal_shown(true);}
                18=>{key(w,Key::Return.into());}
                19=>{check("retained modal reopen refocuses native Cancel",ui.get_modal_dismissals()==2&&ui.get_modal_accepts()==0);ui.set_modal_shown(false);ui.set_reduced(true);}
                20=>{check("reduced modal cleanup restores once",!ui.get_modal_presented()&&!ui.get_modal_animating()&&ui.get_restores()==2);ui.set_reduced(false);}
                21=>{ui.set_toast_shown(true);}
                22=>{ui.set_reduced(true);}
                23=>{check("reduce during entry settles visible immediately",ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_reduced(false);}
                24..=223=>{if step.get()==24 {check("reenabling policy does not replay settled entry",ui.get_toast_presented()&&!ui.get_toast_animating());}let open=(step.get()-24)%2==0;ui.set_toast_shown(open);ui.set_modal_shown(open);ui.set_batch_shown(open);if step.get()==26 {check("20 simultaneous toasts are presented",ui.get_batch_presented()==20);} }
                228=>{check("100 reversals finish with every transient hidden",!ui.get_toast_presented()&&!ui.get_modal_presented()&&ui.get_batch_presented()==0);check("100 reversals stop transition timers",!ui.get_toast_animating()&&!ui.get_modal_animating());check("100 cycles restore exactly once each",ui.get_restores()==102);check("animation completion emits no business command",ui.get_toast_dismissals()==1&&ui.get_modal_dismissals()==2&&ui.get_modal_accepts()==0);ui.set_toast_shown(true);}
                233=>{ui.set_toast_shown(false);}
                234=>{ui.set_toast_shown(true);}
                237=>{check("reopen restarts cleanup deadline",ui.get_toast_presented()&&ui.get_toast_animating());}
                240=>{check("reopened transition settles without stale removal",ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_toast_shown(false);}
                245=>{shot("settled-hidden");check("final hidden transition is inactive",!ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_toast_shown(true);}
                246=>{ui.set_toast_visible(false);}
                247=>{check("own hidden toast cleans without waiting for exit",!ui.get_toast_presented()&&!ui.get_toast_animating());ui.set_toast_shown(false);ui.set_toast_visible(true);ui.set_modal_shown(true);}
                248=>{ui.set_modal_visible(false);}
                249=>{check("own hidden modal cleans without waiting for exit",!ui.get_modal_presented()&&!ui.get_modal_animating());ui.set_modal_shown(false);}
                250=>{check("logical close after own hide still restores once",ui.get_restores()==103);println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap();}
                _=>{}
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
