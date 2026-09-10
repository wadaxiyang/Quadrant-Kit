// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
//! DWM-owned caption controls above a Slint toolbar; the HWND stays decorated.

use super::{DesignGalleryWindow, is_caption, screen_coordinates};
use slint::winit_030::winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use slint::{ComponentHandle, Weak, winit_030::WinitWindowAccessor};
use std::cell::Cell;
use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM},
    Graphics::{
        Dwm::{
            DWMNCRP_ENABLED, DWMWA_ALLOW_NCPAINT, DWMWA_CAPTION_BUTTON_BOUNDS, DWMWA_CAPTION_COLOR,
            DWMWA_NCRENDERING_POLICY, DWMWA_USE_IMMERSIVE_DARK_MODE, DwmDefWindowProc,
            DwmExtendFrameIntoClientArea, DwmGetWindowAttribute, DwmSetWindowAttribute,
        },
        Gdi::{
            GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromRect, ScreenToClient,
        },
    },
    UI::{
        Controls::MARGINS,
        HiDpi::{GetDpiForWindow, GetSystemMetricsForDpi},
        Shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
        WindowsAndMessaging::{
            GetClientRect, GetWindowRect, HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION,
            HTCLIENT, HTCLOSE, HTLEFT, HTMAXBUTTON, HTMINBUTTON, HTRIGHT, HTTOP, HTTOPLEFT,
            HTTOPRIGHT, IsIconic, IsZoomed, MINMAXINFO, NCCALCSIZE_PARAMS, SM_CXPADDEDBORDER,
            SM_CXSIZEFRAME, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER,
            SendMessageW, SetWindowPos, TITLEBARINFOEX, WM_GETMINMAXINFO, WM_GETTITLEBARINFOEX,
            WM_NCCALCSIZE, WM_NCDESTROY, WM_NCHITTEST, WM_NCRBUTTONUP,
        },
    },
};

struct State {
    gallery: Weak<DesignGalleryWindow>,
    window: std::sync::Weak<slint::winit_030::winit::window::Window>,
    hwnd: Cell<HWND>,
}

pub struct TitleBarGuard {
    state: Box<State>,
    metrics: slint::Timer,
}

pub async fn install(
    gallery: &DesignGalleryWindow,
) -> Result<TitleBarGuard, Box<dyn std::error::Error>> {
    let window = gallery.window().winit_window().await?;
    let RawWindowHandle::Win32(handle) = window.window_handle()?.as_raw() else {
        return Err("Gallery's window is not a Win32 window".into());
    };
    let hwnd = handle.hwnd.get() as HWND;
    let state = Box::new(State {
        gallery: gallery.as_weak(),
        window: std::sync::Arc::downgrade(&window),
        hwnd: Cell::new(hwnd),
    });
    let data = std::ptr::from_ref(state.as_ref()) as usize;
    // SAFETY: the live HWND and stable box belong to this UI thread. Drop removes the subclass.
    if unsafe { SetWindowSubclass(hwnd, Some(caption_proc), 1, data) } == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let guard = TitleBarGuard {
        state,
        metrics: slint::Timer::default(),
    };
    // Keep the requested content size after removing the separate native caption.
    let requested_size = window.inner_size();
    // SAFETY: request recalculation using our NCCALCSIZE handler without activation.
    if unsafe {
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            0,
            0,
            i32::try_from(requested_size.width)?,
            i32::try_from(requested_size.height)?,
            SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
        )
    } == 0
    {
        return Err(std::io::Error::last_os_error().into());
    }
    let mut last_style = None;
    update_metrics(hwnd, gallery, &mut last_style)?;
    let weak = gallery.as_weak();
    let native_window = std::sync::Arc::downgrade(&window);
    guard.metrics.start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        move || {
            // Never retain the winit Arc: Slint must be able to destroy its window on close.
            if let (Some(gallery), Some(_window)) = (weak.upgrade(), native_window.upgrade())
                && let Err(error) = update_metrics(hwnd, &gallery, &mut last_style)
            {
                eprintln!("Could not update native caption: {error}");
            }
        },
    );
    Ok(guard)
}

type Style = (u32, u32, bool, u32, u32);

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn update_metrics(
    hwnd: HWND,
    gallery: &DesignGalleryWindow,
    last_style: &mut Option<Style>,
) -> Result<(), String> {
    // SAFETY: caller holds a live native window on its UI thread.
    if unsafe { IsIconic(hwnd) } != 0 {
        return Ok(());
    }
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    let scale = gallery.window().scale_factor();
    let caption_height = gallery.get_title_bar_height();
    let dark = gallery.get_chrome_dark();
    let color = gallery.get_chrome_background();
    let color_ref =
        u32::from(color.red()) | (u32::from(color.green()) << 8) | (u32::from(color.blue()) << 16);
    if *last_style
        != Some((
            dpi,
            scale.to_bits(),
            dark,
            color_ref,
            caption_height.to_bits(),
        ))
    {
        let margins = MARGINS {
            cxLeftWidth: 0,
            cxRightWidth: 0,
            cyBottomHeight: 0,
            cyTopHeight: (caption_height * scale).ceil() as i32,
        };
        set_attribute(hwnd, DWMWA_NCRENDERING_POLICY, DWMNCRP_ENABLED);
        set_attribute(hwnd, DWMWA_ALLOW_NCPAINT, 1);
        // SAFETY: valid HWND and correctly sized stack margins. Without DWM there
        // would be no visible caption controls; initialization must not silently succeed.
        let result = unsafe { DwmExtendFrameIntoClientArea(hwnd, &raw const margins) };
        if result < 0 {
            return Err(format!("DwmExtendFrameIntoClientArea failed: {result:#x}"));
        }
        // Cosmetic attributes are optional on older Windows; default native colors remain.
        set_attribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, i32::from(dark));
        set_attribute(
            hwnd,
            DWMWA_CAPTION_COLOR,
            i32::try_from(color_ref).unwrap_or_default(),
        );
        *last_style = Some((
            dpi,
            scale.to_bits(),
            dark,
            color_ref,
            caption_height.to_bits(),
        ));
    }
    // Match the individual native button, including restored/maximized clipping.
    let mut caption = TITLEBARINFOEX {
        cbSize: u32::try_from(std::mem::size_of::<TITLEBARINFOEX>()).unwrap(),
        ..Default::default()
    };
    // SAFETY: synchronous query on this UI thread with a correctly sized buffer.
    unsafe { SendMessageW(hwnd, WM_GETTITLEBARINFOEX, 0, (&raw mut caption) as isize) };
    let minimize = caption.rgrect[2];
    let mut origin = POINT {
        x: minimize.left,
        y: minimize.top,
    };
    if minimize.right > minimize.left
        && minimize.bottom > minimize.top
        && unsafe { ScreenToClient(hwnd, &raw mut origin) } != 0
    {
        gallery.set_caption_back_width((minimize.right - minimize.left) as f32 / scale);
        gallery.set_caption_back_height((minimize.bottom - minimize.top) as f32 / scale);
        gallery.set_caption_back_top(origin.y as f32 / scale);
    }
    let mut bounds = RECT::default();
    let mut window_rect = RECT::default();
    // DWM reports physical window-relative bounds. Convert to client coordinates,
    // then to Slint logical pixels, including an explicit Slint scale override.
    // SAFETY: all output buffers have their documented sizes.
    if unsafe {
        DwmGetWindowAttribute(
            hwnd,
            u32::try_from(DWMWA_CAPTION_BUTTON_BOUNDS).unwrap(),
            (&raw mut bounds).cast(),
            u32::try_from(std::mem::size_of::<RECT>()).unwrap(),
        )
    } >= 0
        && bounds.right > bounds.left
        && unsafe { GetWindowRect(hwnd, &raw mut window_rect) } != 0
    {
        let mut point = POINT {
            x: window_rect.left + bounds.left,
            y: window_rect.top,
        };
        let mut client = RECT::default();
        if unsafe { ScreenToClient(hwnd, &raw mut point) } != 0
            && unsafe { GetClientRect(hwnd, &raw mut client) } != 0
        {
            gallery.set_toolbar_trailing_inset(
                ((client.right - point.x) as f32 / scale + 8.0).max(0.0),
            );
        }
    }
    Ok(())
}

fn set_attribute(hwnd: HWND, attribute: i32, value: i32) {
    // SAFETY: all attributes used here take a 32-bit value; HWND is live on this thread.
    unsafe {
        DwmSetWindowAttribute(
            hwnd,
            u32::try_from(attribute).unwrap(),
            (&raw const value).cast(),
            u32::try_from(std::mem::size_of::<i32>()).unwrap(),
        )
    };
}

impl Drop for TitleBarGuard {
    fn drop(&mut self) {
        self.metrics.stop();
        if !self.state.hwnd.get().is_null() {
            // SAFETY: callback data remains alive until the subclass is removed.
            unsafe { RemoveWindowSubclass(self.state.hwnd.get(), Some(caption_proc), 1) };
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
    // SAFETY: data points to the guard-owned box until removal, on this thread only.
    let state = unsafe { &*(data as *const State) };
    if message == WM_NCDESTROY {
        state.hwnd.set(std::ptr::null_mut());
        unsafe { RemoveWindowSubclass(hwnd, Some(caption_proc), subclass_id) };
    } else if message == WM_GETMINMAXINFO {
        // winit adds the default caption/borders to Slint's content minimum. Our
        // client covers the frame, so use Slint's actual minimum without that extra.
        let result = unsafe { DefSubclassProc(hwnd, message, wparam, lparam) };
        if let Some(gallery) = state.gallery.upgrade() {
            let limits = unsafe { &mut *(lparam as *mut MINMAXINFO) };
            let scale = gallery.window().scale_factor();
            #[allow(clippy::cast_possible_truncation)]
            {
                limits.ptMinTrackSize.x = (gallery.get_chrome_min_width() * scale).ceil() as i32;
                limits.ptMinTrackSize.y = (gallery.get_chrome_min_height() * scale).ceil() as i32;
            }
        }
        return result;
    } else if message == WM_NCCALCSIZE && wparam != 0 {
        // Do not first call DefSubclassProc: its caption calculation changes DWM
        // state and prevents native caption painting after expanding the client.
        let params = unsafe { &mut *(lparam as *mut NCCALCSIZE_PARAMS) };
        if unsafe { IsZoomed(hwnd) } != 0 {
            let monitor =
                unsafe { MonitorFromRect(&raw const params.rgrc[0], MONITOR_DEFAULTTONEAREST) };
            let mut info = MONITORINFO {
                cbSize: u32::try_from(std::mem::size_of::<MONITORINFO>()).unwrap(),
                ..Default::default()
            };
            if unsafe { GetMonitorInfoW(monitor, &raw mut info) } != 0 {
                params.rgrc[0] = info.rcWork;
            }
        }
        return 0;
    } else if message == WM_NCRBUTTONUP && wparam == usize::try_from(HTCAPTION).unwrap() {
        // The expanded client suppresses the default caption context menu. Ask
        // winit to show the real system menu at the client-relative pointer.
        let (x, y) = screen_coordinates(lparam);
        let mut point = POINT {
            x: i32::from(x),
            y: i32::from(y),
        };
        if unsafe { ScreenToClient(hwnd, &raw mut point) } != 0
            && let Some(window) = state.window.upgrade()
        {
            window.show_window_menu(slint::winit_030::winit::dpi::PhysicalPosition::new(
                point.x, point.y,
            ));
        }
        return 0;
    } else {
        let mut result = 0;
        // Forward all messages including NCMOUSELEAVE: DWM owns button hit testing,
        // hover, press, system commands and the Windows 11 maximize hover affordance.
        if unsafe { DwmDefWindowProc(hwnd, message, wparam, lparam, &raw mut result) } != 0 {
            return result;
        }
        if message == WM_NCHITTEST
            && let Some(gallery) = state.gallery.upgrade()
        {
            return hit_test(hwnd, lparam, &gallery);
        }
    }
    // SAFETY: pass untouched arguments to the remaining subclass chain.
    unsafe { DefSubclassProc(hwnd, message, wparam, lparam) }
}

fn hit_test(hwnd: HWND, lparam: LPARAM, gallery: &DesignGalleryWindow) -> LRESULT {
    let (x, y) = screen_coordinates(lparam);
    // DWM can paint the maximized caption yet decline its hit test after the
    // client is constrained to the work area. Use the OS's individual button
    // rectangles in that case, never guessed button widths or Slint glyphs.
    if let Some(button) = caption_button(hwnd, i32::from(x), i32::from(y)) {
        return isize::try_from(button).unwrap_or_default();
    }
    let mut point = POINT {
        x: i32::from(x),
        y: i32::from(y),
    };
    let mut client = RECT::default();
    // SAFETY: live HWND and writable stack buffers.
    if unsafe { ScreenToClient(hwnd, &raw mut point) } == 0
        || unsafe { GetClientRect(hwnd, &raw mut client) } == 0
    {
        return 0;
    }
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    let edge = unsafe {
        GetSystemMetricsForDpi(SM_CXSIZEFRAME, dpi) + GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi)
    };
    if unsafe { IsZoomed(hwnd) } == 0
        && let Some(border) = resize_border(point.x, point.y, client.right, client.bottom, edge)
    {
        return isize::try_from(border).unwrap_or_default();
    }
    let scale = gallery.window().scale_factor();
    #[allow(clippy::cast_precision_loss)]
    let (x, y) = (point.x as f32 / scale, point.y as f32 / scale);
    let hit = if is_caption(
        x,
        y,
        gallery.get_title_drag_x(),
        gallery.get_title_drag_width(),
        0.0,
        gallery.get_title_bar_height(),
    ) {
        HTCAPTION
    } else {
        HTCLIENT
    };
    isize::try_from(hit).unwrap_or_default()
}

fn caption_button(hwnd: HWND, x: i32, y: i32) -> Option<u32> {
    let mut info = TITLEBARINFOEX {
        cbSize: u32::try_from(std::mem::size_of::<TITLEBARINFOEX>()).unwrap(),
        ..Default::default()
    };
    // SAFETY: synchronous query on this thread's live HWND; Windows initializes
    // the stack buffer. This message goes through to the default window procedure.
    unsafe { SendMessageW(hwnd, WM_GETTITLEBARINFOEX, 0, (&raw mut info) as isize) };
    for (index, hit) in [(2, HTMINBUTTON), (3, HTMAXBUTTON), (5, HTCLOSE)] {
        let rect = info.rgrect[index];
        if x >= rect.left && x < rect.right && y >= rect.top && y < rect.bottom {
            return Some(hit);
        }
    }
    None
}

fn resize_border(x: i32, y: i32, width: i32, height: i32, edge: i32) -> Option<u32> {
    let left = x < edge;
    let right = x >= width - edge;
    let top = y < edge;
    let bottom = y >= height - edge;
    match (left, right, top, bottom) {
        (true, _, true, _) => Some(HTTOPLEFT),
        (_, true, true, _) => Some(HTTOPRIGHT),
        (true, _, _, true) => Some(HTBOTTOMLEFT),
        (_, true, _, true) => Some(HTBOTTOMRIGHT),
        (true, _, _, _) => Some(HTLEFT),
        (_, true, _, _) => Some(HTRIGHT),
        (_, _, true, _) => Some(HTTOP),
        (_, _, _, true) => Some(HTBOTTOM),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_resize_edges_and_corners_leave_content_untouched() {
        for (x, y, expected) in [
            (0, 0, HTTOPLEFT),
            (799, 0, HTTOPRIGHT),
            (0, 599, HTBOTTOMLEFT),
            (799, 599, HTBOTTOMRIGHT),
            (0, 300, HTLEFT),
            (799, 300, HTRIGHT),
            (400, 0, HTTOP),
            (400, 599, HTBOTTOM),
        ] {
            assert_eq!(resize_border(x, y, 800, 600, 8), Some(expected));
        }
        assert_eq!(resize_border(8, 8, 800, 600, 8), None);
        assert_eq!(resize_border(791, 591, 800, 600, 8), None);
        assert_eq!(resize_border(400, 300, 800, 600, 8), None);
    }
}
