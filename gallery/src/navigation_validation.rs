// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Opt-in native input/render harness for the public `NavigationView`.
use crate::{
    NavigationValidationWindow,
    config::{Config, ThemeChoice},
};
use slint::{ComponentHandle, LogicalSize, ModelRc, VecModel};
use std::time::Duration;

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let window = NavigationValidationWindow::new()?;
    window.on_rows(
        |case, footer, controls, inputs, settings, disabled, icon, selected_icon| {
            let mut rows = crate::navigation_samples::entries(
                case,
                footer,
                controls,
                inputs,
                settings,
                &icon,
                &selected_icon,
            );
            for row in &mut rows {
                if row.id == "button" {
                    row.enabled = !disabled;
                }
                if row.id == "controls" {
                    row.text =
                        "Controls — Komponentenbibliothek / Composants et paramètres étendus"
                            .into();
                }
                if row.id == "inputs" {
                    row.text =
                        "Inputs — Eingabeeinstellungen / Champs de saisie supplémentaires".into();
                }
            }
            ModelRc::new(VecModel::from(rows))
        },
    );
    #[cfg(target_os = "windows")]
    window.set_ui_font_family("Segoe UI Variable Text".into());
    window.set_dark(config.theme == ThemeChoice::Dark);
    window.invoke_apply_theme();
    window.set_variant(config.navigation_validation.unwrap_or(0));
    window.set_scenario(config.navigation_case);
    window.set_compact(config.navigation_compact);
    if let Some((width, height)) = config.size {
        window.window().set_size(LogicalSize::new(width, height));
    }
    if let Some(path) = config.snapshot.clone() {
        window.show()?;
        let weak = window.as_weak();
        slint::Timer::single_shot(Duration::from_millis(1200), move || {
            let result = weak
                .upgrade()
                .ok_or_else(|| "validation window closed".into())
                .and_then(|window| {
                    let pixels = window.window().take_snapshot()?;
                    if !crate::snapshot_has_visible_pixels(&pixels) {
                        return Err("empty validation snapshot".into());
                    }
                    crate::write_snapshot(&path, &pixels)
                });
            if let Err(error) = result {
                eprintln!("Navigation validation capture failed: {error}");
                std::process::exit(2);
            }
            drop(slint::quit_event_loop());
        });
        slint::run_event_loop()?;
    } else {
        window.run()?;
    }
    Ok(())
}
