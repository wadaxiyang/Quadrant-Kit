// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! Gallery-owned Windows caption hit testing; Kit has no native window code.

use crate::DesignGalleryWindow;

#[cfg(target_os = "windows")]
type InstallResult = std::rc::Rc<std::cell::RefCell<Option<Result<native::TitleBarGuard, String>>>>;

pub struct TitleBarInstallation {
    #[cfg(target_os = "windows")]
    result: InstallResult,
}

impl TitleBarInstallation {
    #[cfg_attr(
        not(target_os = "windows"),
        allow(clippy::unnecessary_wraps, clippy::unused_self)
    )]
    pub fn check(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        match self.result.borrow().as_ref() {
            Some(Ok(_)) => (),
            Some(Err(error)) => return Err(error.clone().into()),
            None => return Err("Native title bar initialization did not complete".into()),
        }
        Ok(())
    }
}

#[cfg_attr(not(target_os = "windows"), allow(clippy::unnecessary_wraps))]
pub fn install(
    gallery: &DesignGalleryWindow,
) -> Result<TitleBarInstallation, slint::EventLoopError> {
    #[cfg(target_os = "windows")]
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
    #[cfg(not(target_os = "windows"))]
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
fn is_caption(x: f32, y: f32, start: f32, width: f32, top: f32) -> bool {
    x >= start && x < start + width && y >= top && y < 48.0
}

// The FFI is confined to this host-specific module. The guard owns the callback
// data until the subclass is removed; all other Gallery code denies unsafe.
#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
mod native {
    use super::{DesignGalleryWindow, is_caption, screen_coordinates};
    use slint::winit_030::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use slint::{ComponentHandle, Weak, winit_030::WinitWindowAccessor};
    use std::cell::Cell;
    use windows_sys::Win32::{
        Foundation::{HWND, LPARAM, LRESULT, POINT, WPARAM},
        Graphics::Gdi::ScreenToClient,
        UI::{
            Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
            WindowsAndMessaging::{HTCAPTION, WM_NCDESTROY, WM_NCHITTEST, WM_NCRBUTTONUP},
        },
    };

    struct State {
        gallery: Weak<DesignGalleryWindow>,
        hwnd: Cell<HWND>,
        window: std::sync::Weak<slint::winit_030::winit::window::Window>,
    }

    pub struct TitleBarGuard(Box<State>);

    pub async fn install(
        gallery: &DesignGalleryWindow,
    ) -> Result<TitleBarGuard, Box<dyn std::error::Error>> {
        let window = gallery.window().winit_window().await?;
        let handle = window.window_handle()?.as_raw();
        let RawWindowHandle::Win32(handle) = handle else {
            return Err("Gallery's window is not a Win32 window".into());
        };
        let hwnd = handle.hwnd.get() as HWND;
        let state = Box::new(State {
            gallery: gallery.as_weak(),
            hwnd: Cell::new(hwnd),
            window: std::sync::Arc::downgrade(&window),
        });
        let data = std::ptr::from_ref(state.as_ref()) as usize;
        // SAFETY: this is our live window, on its UI thread. The box remains
        // stable and owned by the returned guard until removal in Drop.
        if unsafe { SetWindowSubclass(hwnd, Some(caption_proc), 1, data) } == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(TitleBarGuard(state))
    }

    impl Drop for TitleBarGuard {
        fn drop(&mut self) {
            if !self.0.hwnd.get().is_null() {
                // SAFETY: same UI thread; callback data is still alive. A
                // destroyed window clears the handle in WM_NCDESTROY below.
                unsafe { RemoveWindowSubclass(self.0.hwnd.get(), Some(caption_proc), 1) };
            }
        }
    }

    unsafe extern "system" fn caption_proc(
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
        subclass_id: usize,
        data: usize,
    ) -> LRESULT {
        // SAFETY: SetWindowSubclass receives this guard-owned, stable pointer;
        // the guard removes this callback before freeing it, on this thread.
        let state = unsafe { &*(data as *const State) };
        if message == WM_NCDESTROY {
            state.hwnd.set(std::ptr::null_mut());
            // SAFETY: removing the currently executing subclass is supported.
            unsafe { RemoveWindowSubclass(hwnd, Some(caption_proc), subclass_id) };
        } else if message == WM_NCRBUTTONUP && wparam == HTCAPTION as usize {
            let (x, y) = screen_coordinates(lparam);
            let mut point = POINT {
                x: i32::from(x),
                y: i32::from(y),
            };
            // SAFETY: current live window and writable stack point. Frameless
            // windows need an explicit request for the standard caption menu.
            if unsafe { ScreenToClient(hwnd, &raw mut point) } != 0
                && let Some(window) = state.window.upgrade()
            {
                window.show_window_menu(slint::winit_030::winit::dpi::PhysicalPosition::new(
                    point.x, point.y,
                ));
            }
            return 0;
        } else if message == WM_NCHITTEST
            && let Some(gallery) = state.gallery.upgrade()
        {
            let (x, y) = screen_coordinates(lparam);
            let mut point = POINT {
                x: i32::from(x),
                y: i32::from(y),
            };
            // SAFETY: hwnd is the current live window and point is writable.
            if unsafe { ScreenToClient(hwnd, &raw mut point) } != 0 {
                let scale = gallery.window().scale_factor();
                // Screen coordinates are bounded by Win32's signed 16-bit
                // message contract; conversion to f32 is exact here.
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (point.x as f32 / scale, point.y as f32 / scale);
                if is_caption(
                    x,
                    y,
                    gallery.get_title_drag_x(),
                    gallery.get_title_drag_width(),
                    gallery.get_title_drag_top(),
                ) {
                    return LRESULT::try_from(HTCAPTION).unwrap_or_default();
                }
            }
        }
        // SAFETY: preserve the original message and continue the subclass chain.
        unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
    }
}

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
        assert!(is_caption(270.0, 24.0, 84.0, 506.0, 5.0));
        for (x, y) in [(64.0, 24.0), (600.0, 24.0), (270.0, 48.0), (270.0, 4.0)] {
            assert!(!is_caption(x, y, 84.0, 506.0, 5.0));
        }
        assert!(is_caption(270.0, 0.0, 84.0, 506.0, 0.0));
    }
}
