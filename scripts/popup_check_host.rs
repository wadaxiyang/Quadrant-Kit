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
// Native Windows menus run an OS modal loop: WindowEvent dispatch is not its input.
// Send test keys only while this test process owns the foreground window.
#[cfg(target_os = "windows")]
fn select_native_menu() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_millis(350));
        #[link(name = "user32")]
        unsafe extern "system" {
            fn GetForegroundWindow() -> *mut std::ffi::c_void;
            fn GetWindowThreadProcessId(window: *mut std::ffi::c_void, process: *mut u32) -> u32;
            fn keybd_event(key: u8, scan: u8, flags: u32, extra: usize);
        }
        unsafe {
            let mut process = 0;
            GetWindowThreadProcessId(GetForegroundWindow(), &mut process);
            if process != std::process::id() { println!("OS_MENU_INPUT=NOT_RUN_FOREGROUND"); return; }
            println!("OS_MENU_INPUT=OWN_PROCESS");
            for key in [0x28u8, 0x0d] {
                keybd_event(key, 0, 0, 0);
                keybd_event(key, 0, 2, 0);
                std::thread::sleep(Duration::from_millis(40));
            }
        }
    });
}
#[cfg(not(target_os = "windows"))]
fn select_native_menu() {}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()?;
    let ui = PopupCheck::new()?;
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
            let click = |x,y| { pointer(w,x,y,true);pointer(w,x,y,false); };
            match step.get() {
                0 => { check("initial popups closed",!ui.get_fly_open()&&!ui.get_drop_open()&&!ui.get_split_open());click(80.,36.); }
                1 => {check("flyout opens via native button",ui.get_fly_open());shot("flyout");key(w,Key::Tab.into());}
                2 => {key(w,Key::Return.into());check("native popup child Return activates once",ui.get_fly_commands()==1);check("inside click policy retains popup",ui.get_fly_open());key(w,Key::Escape.into());}
                3 => {check("native Escape closes popup",!ui.get_fly_open());check("native close restores opener",ui.get_opener_focused());key(w,Key::Space.into());}
                4 => {check("restored opener Space reopens",ui.get_fly_open());click(500.,350.);}
                5 => {check("outside click closes popup",!ui.get_fly_open());ui.invoke_open_fly();ui.set_controls_enabled(false);}
                6 => {check("disable closes popup",!ui.get_fly_open());click(80.,36.);check("disabled opener stays closed",!ui.get_fly_open());ui.set_controls_enabled(true);ui.invoke_open_fly();}
                7 => {ui.invoke_close_fly();}
                8 => {check("imperative close updates actual native state",!ui.get_fly_open());check("imperative close restores focus",ui.get_opener_focused());ui.set_anchor_x(620.);ui.set_anchor_y(420.);ui.invoke_open_fly();}
                9 => {check("edge popup opens",ui.get_fly_open());shot("edge-flyout");key(w,Key::Escape.into());ui.set_anchor_x(20.);ui.set_anchor_y(20.);}
                10 => {click(80.,216.);}
                11 => {check("drop-down opens popup",ui.get_drop_open());shot("dropdown");key(w,Key::Tab.into());key(w,Key::Return.into());}
                12 => {check("drop-down child acts and closes",ui.get_drop_commands()==1&&!ui.get_drop_open());click(80.,276.);}
                13 => {check("split primary is independent",ui.get_primary_commands()==1&&!ui.get_split_open());click(180.,276.);}
                14 => {check("split secondary opens only popup",ui.get_primary_commands()==1&&ui.get_split_open());shot("split");key(w,Key::Tab.into());key(w,Key::Return.into());}
                15 => {check("split option command closes popup",ui.get_split_commands()==1&&!ui.get_split_open());ui.set_controls_enabled(false);click(80.,276.);click(180.,276.);}
                16 => {check("disabled split suppresses both regions",ui.get_primary_commands()==1&&!ui.get_split_open());ui.set_controls_enabled(true);select_native_menu();ui.invoke_open_menu();}
                17 => {shot("after-native-menu");}
                18 => {check("native menu keyboard activates first command",ui.get_menu_commands()==1);for _ in 0..10 {ui.invoke_open_fly();ui.invoke_close_fly();}}
                19 => {check("rapid popup cycles finish closed without extra commands",!ui.get_fly_open()&&ui.get_fly_commands()==1);shot("closed");ui.set_dark(true);ui.set_long_content(true);ui.invoke_open_fly();}
                20 => {shot("scroll-before");w.dispatch_event(WindowEvent::PointerScrolled {position:slint::LogicalPosition::new(80.,220.),delta_x:0.,delta_y:-1000.});}
                21 => {check("native scroll inside flyout advances viewport",ui.get_scroll_y() < -500.);shot("scroll-after");key(w,Key::Escape.into());}
                22 => {check("scrolled flyout closes and restores opener",!ui.get_fly_open()&&ui.get_opener_focused());}
                _ => {println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap();}
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
