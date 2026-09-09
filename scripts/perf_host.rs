// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
// Copied only to ignored, generated measurement projects by run_perf.py.
use slint::ComponentHandle;
use std::{
    cell::Cell,
    rc::Rc,
    time::{Duration, Instant},
};
slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let window = PerfWindow::new()?;
    println!("construct_ms={}", start.elapsed().as_secs_f64() * 1000.0);
    let seen = Rc::new(Cell::new(false));
    let marker = seen.clone();
    let hook = window.window().set_rendering_notifier(move |state, _| {
        if matches!(state, slint::RenderingState::AfterRendering) && !marker.replace(true) {
            println!(
                "first_render_callback_ms={}",
                start.elapsed().as_secs_f64() * 1000.0
            );
        }
    });
    println!("render_hook_supported={}", hook.is_ok());
    window.show()?;
    println!("show_return_ms={}", start.elapsed().as_secs_f64() * 1000.0);
    // A supported software-rendered pixel buffer, not OS presentation or loader time.
    // Keep the unsupported AfterRendering hook separate, never substitute its value.
    match window.window().take_snapshot() {
        Ok(pixels) if pixels.as_slice().iter().any(|pixel| pixel.a != 0) => {
            println!(
                "first_software_frame_ms={}",
                start.elapsed().as_secs_f64() * 1000.0
            );
        }
        _ => println!("software_frame_status=NOT_RUN"),
    }
    slint::Timer::single_shot(Duration::from_millis(1500), move || {
        println!(
            "settled_marker_ms={}",
            start.elapsed().as_secs_f64() * 1000.0
        );
        // The external sampler observes this live process at a fixed point.
    });
    slint::Timer::single_shot(Duration::from_millis(2500), move || {
        slint::quit_event_loop().expect("quit measurement event loop");
    });
    slint::run_event_loop()?;
    Ok(())
}
