// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Host-owned native window integration. Kit and `GalleryToolbar` own no OS chrome.

use crate::DesignGalleryWindow;

/// Configure platform attributes before Slint creates any windows.
#[cfg_attr(
    not(any(target_os = "windows", target_os = "macos")),
    allow(clippy::unnecessary_wraps)
)]
pub fn configure_backend() -> Result<(), slint::PlatformError> {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        let backend = slint::BackendSelector::new().backend_name("winit".into());
        #[cfg(target_os = "windows")]
        let backend = backend
            .with_winit_window_attributes_hook(|attributes| attributes.with_transparent(true));
        #[cfg(target_os = "macos")]
        let backend = backend.with_winit_window_attributes_hook(|attributes| {
            use slint::winit_030::winit::platform::macos::WindowAttributesExtMacOS;
            attributes
                .with_titlebar_transparent(true)
                .with_title_hidden(true)
                .with_fullsize_content_view(true)
                .with_titlebar_buttons_hidden(false)
        });
        backend.select()?;
    }
    Ok(())
}

pub fn configure_toolbar(gallery: &DesignGalleryWindow) {
    // Conservative initial exclusions, replaced by live native measurements.
    #[cfg(target_os = "windows")]
    {
        gallery.set_toolbar_trailing_inset(140.0);
        gallery.set_native_caption_surface(true);
    }
    #[cfg(target_os = "macos")]
    {
        gallery.set_toolbar_leading_inset(80.0);
        gallery.set_client_caption_input(true);
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let _ = gallery;
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
type InstallResult = std::rc::Rc<std::cell::RefCell<Option<Result<native::TitleBarGuard, String>>>>;

pub struct TitleBarInstallation {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    result: InstallResult,
}

impl TitleBarInstallation {
    #[cfg_attr(
        not(any(target_os = "windows", target_os = "macos")),
        allow(clippy::unnecessary_wraps, clippy::unused_self)
    )]
    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        match self.result.borrow().as_ref() {
            Some(Ok(_)) => (),
            Some(Err(error)) => return Err(error.clone().into()),
            None => return Err("Native title bar initialization did not complete".into()),
        }
        Ok(())
    }
}

#[cfg_attr(
    not(any(target_os = "windows", target_os = "macos")),
    allow(clippy::unnecessary_wraps)
)]
pub fn install(
    gallery: &DesignGalleryWindow,
) -> Result<TitleBarInstallation, slint::EventLoopError> {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        use slint::ComponentHandle;
        let result = std::rc::Rc::new(std::cell::RefCell::new(None));
        let slot = result.clone();
        let weak = gallery.as_weak();
        slint::spawn_local(async move {
            let installed = if let Some(gallery) = weak.upgrade() {
                native::install(&gallery)
                    .await
                    .map_err(|error| error.to_string())
            } else {
                Err("Gallery closed before native title bar initialization".into())
            };
            if installed.is_err() {
                drop(slint::quit_event_loop());
            }
            *slot.borrow_mut() = Some(installed);
        })?;
        Ok(TitleBarInstallation { result })
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let _ = gallery;
        Ok(TitleBarInstallation {})
    }
}

#[cfg(any(target_os = "windows", test))]
fn screen_coordinates(value: isize) -> (i16, i16) {
    let bytes = value.to_le_bytes();
    (
        i16::from_le_bytes([bytes[0], bytes[1]]),
        i16::from_le_bytes([bytes[2], bytes[3]]),
    )
}

#[cfg(any(target_os = "windows", test))]
fn is_caption(x: f32, y: f32, start: f32, width: f32, top: f32, height: f32) -> bool {
    x >= start && x < start + width && y >= top && y < height
}

#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
#[path = "window_chrome/windows.rs"]
mod native;

#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
#[path = "window_chrome/macos.rs"]
mod native;

#[cfg(test)]
mod tests {
    use super::{is_caption, screen_coordinates};

    #[test]
    fn screen_coordinates_preserve_negative_monitor_positions() {
        assert_eq!(screen_coordinates(0x00b4_01aa), (426, 180));
        assert_eq!(screen_coordinates(0x012c_f880), (-1920, 300));
        assert_eq!(screen_coordinates(-70_778_580), (300, -1080));
    }

    #[test]
    fn caption_excludes_buttons_content_and_resize_edge() {
        assert!(is_caption(270.0, 16.0, 84.0, 506.0, 5.0, 32.0));
        for (x, y) in [
            (64.0, 16.0),
            (600.0, 16.0),
            (270.0, 32.0),
            (270.0, 40.0),
            (270.0, 4.0),
        ] {
            assert!(!is_caption(x, y, 84.0, 506.0, 5.0, 32.0));
        }
        assert!(is_caption(270.0, 0.0, 84.0, 506.0, 0.0, 32.0));
        assert!(is_caption(270.0, 40.0, 84.0, 506.0, 0.0, 48.0));
    }
}
