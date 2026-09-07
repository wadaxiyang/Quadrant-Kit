// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Development-only Fluent component gallery host.

#![deny(unsafe_code)]
#![allow(missing_docs)]

use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
    time::Duration,
};

use slint::{ComponentHandle, LogicalSize, Rgba8Pixel, SharedPixelBuffer, Weak};

slint::include_modules!();

mod catalog;
mod config;
mod navigation;
mod navigation_samples;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;
    let gallery = DesignGalleryWindow::new()?;
    gallery.global::<GalleryNavigationExamples>().on_entries(
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
    gallery
        .global::<GalleryNavigationExamples>()
        .set_scenario_index(config.navigation_case);
    gallery
        .global::<GalleryNavigationExamples>()
        .set_compact(config.navigation_compact);
    let system_theme = GallerySystemTheme::new()?;
    gallery.set_system_dark(system_theme.get_system_dark());
    let gallery_weak = gallery.as_weak();
    system_theme.on_scheme_changed(move |dark| {
        if let Some(gallery) = gallery_weak.upgrade() {
            gallery.set_system_dark(dark);
        }
    });
    #[cfg(target_os = "windows")]
    gallery.set_ui_font_family("Segoe UI Variable Text".into());
    if let Some((width, height)) = config.size {
        gallery.window().set_size(LogicalSize::new(width, height));
    }
    gallery.set_gallery_theme(match config.theme {
        config::ThemeChoice::Light => ThemeMode::Light,
        config::ThemeChoice::Dark => ThemeMode::Dark,
        config::ThemeChoice::System => ThemeMode::System,
    });
    navigation::install(&gallery, config.destination)?;
    gallery.set_preview_mode(config.preview);
    // Explicit even when Light equals the default and changed does not run.
    gallery.invoke_apply_theme();
    if let Some(snapshot_path) = config.snapshot {
        gallery.show()?;
        gallery.window().request_redraw();
        schedule_snapshot(
            gallery.as_weak(),
            snapshot_path,
            3,
            Duration::from_millis(1200),
        );
        slint::run_event_loop()?;
    } else {
        gallery.run()?;
    }
    Ok(())
}

fn schedule_snapshot(
    gallery_weak: Weak<DesignGalleryWindow>,
    snapshot_path: PathBuf,
    attempts_remaining: u8,
    delay: Duration,
) {
    slint::Timer::single_shot(delay, move || {
        let Some(gallery) = gallery_weak.upgrade() else {
            eprintln!("failed to save Design Gallery snapshot: gallery closed before snapshot");
            std::process::exit(2);
        };

        match gallery.window().take_snapshot() {
            Ok(pixels) if snapshot_has_visible_pixels(&pixels) => {
                if let Err(error) = write_snapshot(&snapshot_path, &pixels) {
                    eprintln!("failed to save Design Gallery snapshot: {error}");
                    std::process::exit(2);
                }
                drop(slint::quit_event_loop());
            }
            Ok(_) if attempts_remaining > 1 => {
                gallery.window().request_redraw();
                drop(gallery);
                schedule_snapshot(
                    gallery_weak,
                    snapshot_path,
                    attempts_remaining - 1,
                    Duration::from_millis(800),
                );
            }
            Ok(_) => {
                eprintln!(
                    "failed to save Design Gallery snapshot: renderer returned an all-transparent frame"
                );
                std::process::exit(2);
            }
            Err(error) => {
                eprintln!("failed to save Design Gallery snapshot: {error}");
                std::process::exit(2);
            }
        }
    });
}

fn snapshot_has_visible_pixels(pixels: &SharedPixelBuffer<Rgba8Pixel>) -> bool {
    pixels.as_bytes().chunks_exact(4).any(|rgba| rgba[3] != 0)
}

fn write_snapshot(
    path: &Path,
    pixels: &SharedPixelBuffer<Rgba8Pixel>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = BufWriter::new(File::create(path)?);
    let mut encoder = png::Encoder::new(file, pixels.width(), pixels.height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()?
        .write_image_data(pixels.as_bytes())?;
    Ok(())
}
