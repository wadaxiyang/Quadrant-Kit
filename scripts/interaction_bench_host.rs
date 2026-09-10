// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle,
    platform::{Key, WindowEvent},
};
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};
slint::include_modules!();
include!("windows_observation.rs");

fn key(window: &slint::Window, text: slint::SharedString) {
    window.dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window.dispatch_event(WindowEvent::KeyReleased { text });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = std::env::args().nth(1).ok_or("scene required")?;
    let ui = InteractionBench::new()?;
    let is_list = scene.starts_with("lists-");
    ui.set_list_scene(is_list);
    ui.set_group(scene == "text-group");
    if is_list {
        ui.set_row_count(scene[6..].parse()?);
    }
    if scene == "text-long" {
        ui.set_input_text("Long text 输入 ".repeat(200).into());
    }
    let original_text = ui.get_input_text();
    let creates = Rc::new(Cell::new(0));
    let counter = creates.clone();
    ui.on_delegate_created(move || counter.set(counter.get() + 1));
    let timer = Rc::new(slint::Timer::default());
    let weak = ui.as_weak();
    let begin = timer.clone();
    slint::Timer::single_shot(Duration::from_secs(1), move || {
        let ui = weak.unwrap();
        ui.window().take_snapshot().unwrap();
        println!("INITIAL delegates={}", creates.get());
        let step = Cell::new(0);
        let stop = begin.clone();
        begin.start(slint::TimerMode::Repeated, Duration::from_millis(32), move || {
            let ui = weak.unwrap();
            let i = step.get();
            if i == 200 {
                stop.stop();
                assert!(is_list || ui.get_edits() == 200);
                assert!(is_list || ui.get_input_text() == original_text);
                println!("COUNTERS edits={} delegates={} viewport_y={}", ui.get_edits(), creates.get(), ui.get_viewport_y());
                println!("RESULT=PASS");
                slint::quit_event_loop().unwrap();
                return;
            }
            let start = Instant::now();
            if is_list {
                ui.window().dispatch_event(WindowEvent::PointerScrolled {
                    position: slint::LogicalPosition::new(200., 200.),
                    delta_x: 0., delta_y: if i < 100 { -96. } else { 96. },
                });
            } else {
                ui.invoke_focus_other();
                key(ui.window(), Key::End.into());
                key(ui.window(), if i % 2 == 0 { "x".into() } else { Key::Backspace.into() });
                assert_eq!(ui.get_other_edits(), i + 1);
                ui.invoke_focus_input();
                key(ui.window(), Key::End.into());
                key(ui.window(), if i % 2 == 0 { "x".into() } else { Key::Backspace.into() });
                assert_eq!(ui.get_edits(), i + 1);
            }
            let dispatch_ms = start.elapsed().as_secs_f64() * 1000.;
            let start = Instant::now();
            ui.window().take_snapshot().expect("software frame");
            let frame_ms = start.elapsed().as_secs_f64() * 1000.;
            let (private, working) = memory_bytes();
            println!("SAMPLE {{\"index\":{i},\"dispatch_ms\":{dispatch_ms},\"frame_ms\":{frame_ms},\"private_bytes\":{private},\"working_set_bytes\":{working},\"delegates_created\":{}}}", creates.get());
            step.set(i + 1);
        });
    });
    ui.run()?;
    Ok(())
}
