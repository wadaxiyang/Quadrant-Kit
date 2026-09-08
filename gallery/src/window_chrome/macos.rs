// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Native `AppKit` traffic lights with a full-size Slint content view.

use crate::DesignGalleryWindow;
use objc2_app_kit::{
    NSAppearance, NSAppearanceCustomization, NSAppearanceNameAqua, NSAppearanceNameDarkAqua,
    NSApplication, NSView, NSWindowButton,
};
use objc2_foundation::{NSUserDefaults, ns_string};
use slint::winit_030::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use slint::{ComponentHandle, winit_030::WinitWindowAccessor};

pub struct TitleBarGuard {
    _metrics: slint::Timer,
}

pub async fn install(
    gallery: &DesignGalleryWindow,
) -> Result<TitleBarGuard, Box<dyn std::error::Error>> {
    let window = gallery.window().winit_window().await?;
    let RawWindowHandle::AppKit(handle) = window.window_handle()?.as_raw() else {
        return Err("Gallery's window is not an AppKit window".into());
    };
    let view_ptr = handle.ns_view;
    let weak = std::sync::Arc::downgrade(&window);
    let gallery_weak = gallery.as_weak();
    let timer = slint::Timer::default();
    let mut last_dark = None;
    timer.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        move || {
            if let (Some(window), Some(gallery)) = (weak.upgrade(), gallery_weak.upgrade()) {
                // SAFETY: winit supplies an NSView pointer. The live Arc keeps it valid;
                // all accesses occur on Slint's AppKit main thread. Do not retain NSWindow.
                let view = unsafe { view_ptr.cast::<NSView>().as_ref() };
                if let Some(native) = view.window() {
                    let dark = gallery.get_chrome_dark();
                    if last_dark != Some(dark) {
                        // SAFETY: AppKit's exported immutable appearance-name constants.
                        let name = unsafe {
                            if dark {
                                NSAppearanceNameDarkAqua
                            } else {
                                NSAppearanceNameAqua
                            }
                        };
                        if let Some(appearance) = NSAppearance::appearanceNamed(name) {
                            native.setAppearance(Some(&appearance));
                        }
                        last_dark = Some(dark);
                    }
                    let mut right = 0.0_f64;
                    for kind in [
                        NSWindowButton::CloseButton,
                        NSWindowButton::MiniaturizeButton,
                        NSWindowButton::ZoomButton,
                    ] {
                        if let Some(button) = native.standardWindowButton(kind)
                            && !button.isHiddenOrHasHiddenAncestor()
                        {
                            let rect = button.convertRect_toView(button.bounds(), Some(view));
                            right = right.max(rect.origin.x + rect.size.width);
                        }
                    }
                    // AppKit reports points; convert through physical pixels to support
                    // Slint's explicit scale override as well as Retina transitions.
                    #[allow(clippy::cast_possible_truncation)]
                    let inset = ((right + if right > 0.0 { 12.0 } else { 0.0 })
                        * window.scale_factor()
                        / f64::from(gallery.window().scale_factor()))
                        as f32;
                    gallery.set_toolbar_leading_inset(inset);
                }
            }
        },
    );
    let weak = std::sync::Arc::downgrade(&window);
    gallery.on_caption_pressed(move || {
        let Some(window) = weak.upgrade() else { return };
        // SAFETY: same lifetime and thread contract as the metrics callback above.
        let view = unsafe { view_ptr.cast::<NSView>().as_ref() };
        let Some(native) = view.window() else { return };
        let Some(main_thread) = objc2::MainThreadMarker::new() else {
            return;
        };
        let Some(event) = NSApplication::sharedApplication(main_thread).currentEvent() else {
            return;
        };
        if event.clickCount() == 2 {
            let action = NSUserDefaults::standardUserDefaults()
                .stringForKey(ns_string!("AppleActionOnDoubleClick"))
                .map(|value| value.to_string());
            match action.as_deref() {
                Some("Minimize") => native.performMiniaturize(None),
                Some("None") => (),
                _ => native.performZoom(None),
            }
        } else if let Err(error) = window.drag_window() {
            eprintln!("Could not begin native caption drag: {error}");
        }
    });
    Ok(TitleBarGuard { _metrics: timer })
}
