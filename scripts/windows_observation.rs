// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
// Test hosts only. No working-set trim, cache flush or production polling.
#[cfg(target_os = "windows")]
fn memory_bytes() -> (usize, usize) {
    #[repr(C)]
    #[derive(Default)]
    struct Counters {
        cb: u32,
        faults: u32,
        peak_working: usize,
        working: usize,
        peak_paged: usize,
        paged: usize,
        peak_nonpaged: usize,
        nonpaged: usize,
        pagefile: usize,
        peak_pagefile: usize,
        private: usize,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
    }
    #[link(name = "psapi")]
    unsafe extern "system" {
        fn GetProcessMemoryInfo(
            process: *mut std::ffi::c_void,
            counters: *mut Counters,
            size: u32,
        ) -> i32;
    }
    let mut c = Counters {
        cb: std::mem::size_of::<Counters>() as u32,
        ..Counters::default()
    };
    let result = unsafe { GetProcessMemoryInfo(GetCurrentProcess(), &mut c, c.cb) };
    assert_ne!(result, 0, "GetProcessMemoryInfo");
    (c.private, c.working)
}
#[cfg(not(target_os = "windows"))]
compile_error!("This measurement host requires the Windows memory sampler");
