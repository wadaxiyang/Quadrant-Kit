// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle, Model,
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
    let ui = PickersCheck::new()?;
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
                0 => { shot("initial"); ui.set_selected_row(1); }
                1 => { check("host table selection emits no event", ui.get_selected_row()==1 && ui.get_selections()==0); pointer(w,80.,40.,true);pointer(w,80.,40.,false); }
                2 => { check("first native header click requests ascending",ui.get_ascending()==1 && ui.get_descending()==0);check("sort does not mutate host row order",ui.get_rows().row_data(0).unwrap().row_data(0).unwrap().text=="Zebra");pointer(w,80.,40.,true);pointer(w,80.,40.,false); }
                3 => { check("second native header click requests descending",ui.get_ascending()==1 && ui.get_descending()==1);pointer(w,80.,80.,true);pointer(w,80.,80.,false);ui.invoke_focus_table();key(w,Key::DownArrow.into()); }
                4 => { check("table pointer and keyboard select native row", ui.get_selected_row()==1 && ui.get_selections()==2);pointer(w,80.,316.,true);pointer(w,80.,316.,false); }
                5 => { shot("date-open");pointer(w,192.,643.,true);pointer(w,192.,643.,false);pointer(w,326.,736.,true);pointer(w,326.,736.,false); }
                6 => { check("date Cancel preserves committed leap day and restores focus",ui.get_date().day==29 && ui.get_canceled_dates()==1 && ui.get_accepted_dates()==0 && ui.get_date_focused());ui.invoke_open_date(); }
                7 => { shot("date-reopen");pointer(w,267.,736.,true);pointer(w,267.,736.,false); }
                8 => { check("reopen discards canceled date draft",ui.get_requested_date().day==29 && ui.get_requested_date().month==2 && ui.get_accepted_dates()==1);check("accepted date restores opener focus",ui.get_date_focused());pointer(w,280.,316.,true);pointer(w,280.,316.,false); }
                9 => { shot("time-open");pointer(w,416.,490.,true);pointer(w,416.,490.,false);pointer(w,462.,732.,true);pointer(w,462.,732.,false); }
                10 => { check("time Cancel preserves committed value and restores focus",ui.get_time().hour==23 && ui.get_canceled_times()==1 && ui.get_accepted_times()==0 && ui.get_time_focused());ui.invoke_open_time(); }
                11 => { pointer(w,353.,732.,true);pointer(w,353.,732.,false); }
                12 => { check("reopen discards canceled time draft",ui.get_requested_time().hour==23 && ui.get_requested_time().minute==45 && ui.get_accepted_times()==1);check("accepted time restores opener focus",ui.get_time_focused());ui.invoke_open_date(); }
                13 => { ui.invoke_close_date();check("imperative close does not emit canceled",ui.get_canceled_dates()==1);ui.set_dark(true);ui.set_picker_x(800.);ui.set_picker_y(710.);ui.invoke_open_date(); }
                14 => { shot("edge-date");ui.invoke_close_date();ui.set_picker_x(20.);ui.set_picker_y(300.);ui.invoke_open_date(); }
                15 => { pointer(w,332.,118.,true);pointer(w,332.,118.,false); }
                16 => { shot("date-input");pointer(w,100.,454.,true);pointer(w,100.,454.,false);key(w,"02/30/2024".into()); }
                17 => { pointer(w,103.,494.,true);pointer(w,103.,494.,false); }
                18 => { check("native parser blocks invalid February date",ui.get_accepted_dates()==1);shot("invalid-date");pointer(w,100.,454.,true);pointer(w,100.,454.,false);w.dispatch_event(WindowEvent::KeyPressed{text:Key::Control.into()});key(w,"a".into());w.dispatch_event(WindowEvent::KeyReleased{text:Key::Control.into()});key(w,"02/28/2024".into()); }
                19 => { pointer(w,103.,494.,true);pointer(w,103.,494.,false); }
                20 => { check("native parser accepts valid date exactly once",ui.get_accepted_dates()==2 && ui.get_requested_date().day==28 && ui.get_requested_date().month==2);check("accept requests host commit without seizing date state",ui.get_date().day==29);ui.invoke_open_date(); }
                21 => { ui.set_enabled(false); }
                22 => { shot("disabled");ui.invoke_focus_date();key(w,Key::Space.into());pointer(w,80.,316.,true);pointer(w,80.,316.,false);ui.invoke_open_date(); }
                23 => { shot("disabled-after-input");check("disabled picker produces no acceptance or cancellation",ui.get_accepted_dates()==2 && ui.get_canceled_dates()==1);ui.set_enabled(true);for _ in 0..10 {ui.invoke_open_date();ui.invoke_close_date();}ui.set_rows(slint::ModelRc::new(slint::VecModel::<slint::ModelRc<slint::StandardListViewItem>>::from(vec![])));ui.set_selected_row(-1); }
                24 => { ui.invoke_focus_table();key(w,Key::DownArrow.into());check("empty table rejects invalid selection",ui.get_selected_row()==-1 && ui.get_selections()==2);check("rapid popup cycles do not emit commands",ui.get_accepted_dates()==2 && ui.get_canceled_dates()==1);shot("empty-table"); }
                _ => { println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap(); }
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
