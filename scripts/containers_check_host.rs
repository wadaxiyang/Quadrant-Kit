// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle, Model,
    platform::{Key, PointerEventButton, WindowEvent},
};
use std::{cell::Cell, rc::Rc, time::Duration};
slint::include_modules!();

fn item(text: &str) -> slint::StandardListViewItem {
    let mut item = slint::StandardListViewItem::default();
    item.text = text.into();
    item
}

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
    let ui = ContainersCheck::new()?;
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
                0 => {
                    shot("light-initial");
                    println!(
                        "INITIAL_CREATED kit={} native={} extent={}/{}",
                        ui.get_created(),
                        ui.get_native_created(),
                        ui.get_list_extent(),
                        ui.get_native_extent()
                    );
                    check(
                        "10k list creates only visible native delegates",
                        ui.get_created() > 0
                            && ui.get_created() < 100
                            && ui.get_created() == ui.get_native_created(),
                    );
                    check(
                        "10k native and Kit scroll extent agree",
                        ui.get_list_extent() == ui.get_native_extent()
                            && ui.get_list_extent() >= 240000.,
                    );
                    ui.set_current_item(2);
                }
                1 => {
                    check(
                        "programmatic list selection emits no user event",
                        ui.get_current_item() == 2 && ui.get_selections() == 0,
                    );
                    ui.set_list_y(-239800.);
                    ui.set_native_y(-239800.);
                }
                2 => {
                    println!(
                        "BOTTOM_CREATED kit={} native={} last={}/{}",
                        ui.get_created(),
                        ui.get_native_created(),
                        ui.get_last_seen(),
                        ui.get_native_last_seen()
                    );
                    check(
                        "virtual list reaches the final row",
                        ui.get_last_seen() == 9999 && ui.get_native_last_seen() == 9999,
                    );
                    check(
                        "scrolling does not instantiate all 10k rows",
                        ui.get_created() < 200 && ui.get_native_created() < 200,
                    );
                    shot("bottom");
                    ui.set_count(0);
                    ui.set_rows(slint::ModelRc::new(slint::VecModel::<
                        slint::StandardListViewItem,
                    >::from(vec![])));
                }
                3 => {
                    check(
                        "empty native and Kit extents agree",
                        ui.get_list_extent() == ui.get_native_extent(),
                    );
                    ui.invoke_focus_standard();
                    key(w, Key::DownArrow.into());
                    check(
                        "empty model input does not select an invalid row",
                        ui.get_selections() == 0,
                    );
                    shot("empty");
                    ui.set_current_item(-1);
                    ui.set_rows(slint::ModelRc::new(slint::VecModel::from(vec![
                        item("A"),
                        item("B"),
                        item("C"),
                    ])));
                    ui.set_count(10000);
                    ui.set_list_y(0.);
                    ui.set_native_y(0.);
                }
                4 => {
                    pointer(w, 650., 32., true);
                    pointer(w, 650., 32., false);
                    ui.invoke_focus_standard();
                    key(w, Key::DownArrow.into());
                }
                5 => {
                    check(
                        "native row pointer and keyboard selection",
                        ui.get_current_item() == 1 && ui.get_selections() == 2,
                    );
                    ui.get_rows().set_row_data(1, item("Updated / 更新"));
                    w.dispatch_event(WindowEvent::PointerScrolled {
                        position: slint::LogicalPosition::new(150., 320.),
                        delta_x: 0.,
                        delta_y: -100.,
                    });
                    ui.invoke_focus_child();
                    key(w, Key::Space.into());
                }
                6 => {
                    check(
                        "native ScrollView consumes wheel input",
                        ui.get_scroll_y() < 0.,
                    );
                    check(
                        "host-coordinated disabled group child stays inactive",
                        !ui.get_child_checked(),
                    );
                    ui.set_group_enabled(true);
                }
                7 => {
                    ui.invoke_focus_child();
                    key(w, Key::Space.into());
                    check(
                        "reenabled native group child activates",
                        ui.get_child_checked(),
                    );
                    ui.set_tab(1);
                    ui.set_dark(true);
                }
                8 => {
                    check("host updates native tab selection", ui.get_tab() == 1);
                    check(
                        "fixed native tabs retain both content children",
                        ui.get_tab_children() == 2,
                    );
                    shot("dark-updated");
                    pointer(w, 40., 456., true);
                    pointer(w, 40., 456., false);
                }
                9 => {
                    check("native first tab pointer selection", ui.get_tab() == 0);
                    pointer(w, 330., 456., true);
                    pointer(w, 330., 456., false);
                }
                10 => {
                    check("native second tab pointer selection", ui.get_tab() == 1);
                    ui.set_count(3);
                }
                11 => {
                    check(
                        "short model updates native virtual extent",
                        ui.get_list_extent() == ui.get_native_extent()
                            && ui.get_list_extent() <= 200.,
                    );
                    shot("short");
                }
                _ => {
                    println!("RESULT={}", if failed.get() { "FAIL" } else { "PASS" });
                    slint::quit_event_loop().unwrap();
                }
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
