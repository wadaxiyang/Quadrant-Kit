// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::ComponentHandle;
use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
    time::{Duration, Instant},
};
slint::include_modules!();
mod catalog;
mod navigation;
mod navigation_samples;
include!("windows_observation.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = DesignGalleryWindow::new()?;
    ui.set_ui_font_family("Segoe UI Variable Text".into());
    ui.global::<GalleryNavigationExamples>().on_entries(
        |case, footer, controls, inputs, settings, icon, selected_icon| {
            slint::ModelRc::new(slint::VecModel::from(navigation_samples::entries(
                case,
                footer,
                controls,
                inputs,
                settings,
                &icon,
                &selected_icon,
            )))
        },
    );
    navigation::install(&ui, "home")?;
    ui.invoke_apply_theme();
    let ticks: Rc<RefCell<HashSet<String>>> = Rc::new(RefCell::new(HashSet::new()));
    #[cfg(feature = "instance-probe")]
    {
        let seen = ticks.clone();
        ui.global::<PageLifetimeProbe>().on_tick(move |page| {
            seen.borrow_mut().insert(page.to_string());
        });
        ui.global::<PageLifetimeProbe>()
            .on_mounted(|page| println!("MOUNT {page}"));
    }
    let destinations: Vec<_> = catalog::CATALOG
        .iter()
        .filter(|e| e.is_destination() && e.id != "home")
        .collect();
    let timer = Rc::new(slint::Timer::default());
    let begin = timer.clone();
    let weak = ui.as_weak();
    slint::Timer::single_shot(Duration::from_secs(1), move || {
        let index = Cell::new(0usize);
        let stop = begin.clone();
        let expected = RefCell::new(String::new());
        let initial = memory_bytes();
        println!(
            "BASE private_bytes={} working_set_bytes={}",
            initial.0, initial.1
        );
        begin.start(slint::TimerMode::Repeated, Duration::from_millis(100), move || {
            let ui = weak.unwrap();
            let i = index.get();
            if i % 2 == 1 {
                #[cfg(feature="instance-probe")]
                {
                    let seen = ticks.borrow();
                    assert_eq!(seen.len(), 1, "one actual active page Timer; saw {seen:?}");
                    assert!(seen.contains(expected.borrow().as_str()), "retained wrong page {seen:?}");
                    println!("ACTIVE index={} count=1 page={}", i/2, expected.borrow());
                }
            } else if i < 400 {
                let transition = i / 2;
                let entry = if transition % 2 == 1 {
                    catalog::CATALOG.iter().find(|e| e.id == "home").unwrap()
                } else { destinations[(transition / 2) % destinations.len()] };
                let start = Instant::now();
                ui.global::<GalleryNavigation>().invoke_navigate(entry.id.into());
                let dispatch = start.elapsed().as_secs_f64()*1000.;
                let start = Instant::now();
                ui.window().take_snapshot().unwrap();
                let frame = start.elapsed().as_secs_f64()*1000.;
                let (private, working) = memory_bytes();
                println!("PAGE {{\"index\":{transition},\"destination\":\"{}\",\"dispatch_ms\":{dispatch},\"frame_ms\":{frame},\"private_bytes\":{private},\"working_set_bytes\":{working}}}", entry.id);
                *expected.borrow_mut() = entry.component.to_string();
                ticks.borrow_mut().clear();
            } else {
                stop.stop();
                #[cfg(feature="instance-probe")]
                ui.global::<PageLifetimeProbe>().set_enabled(false);
                println!("IDLE_BEGIN");
                // No active driver/probe Timer during this fixed sixty-second interval.
                slint::Timer::single_shot(Duration::from_millis(60_100), move || {
                    let (private, working) = memory_bytes();
                    println!("FINAL private_bytes={private} working_set_bytes={working}");
                    println!("RESULT=PASS");
                    slint::Timer::single_shot(Duration::from_millis(200), || slint::quit_event_loop().unwrap());
                });
                return;
            }
            index.set(i + 1);
        });
    });
    ui.run()?;
    Ok(())
}
