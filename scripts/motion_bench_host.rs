// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::ComponentHandle;
use std::{cell::Cell,rc::Rc,time::{Duration,Instant}};
slint::include_modules!();
#[cfg(target_os="windows")]
fn cpu_seconds()->f64 {
    #[repr(C)] #[derive(Default)] struct FileTime {low:u32,high:u32}
    #[link(name="kernel32")] unsafe extern "system" {
        fn GetCurrentProcess()->*mut std::ffi::c_void;
        fn GetProcessTimes(process:*mut std::ffi::c_void,creation:*mut FileTime,exit:*mut FileTime,kernel:*mut FileTime,user:*mut FileTime)->i32;
    }
    let (mut c,mut e,mut k,mut u)=(FileTime::default(),FileTime::default(),FileTime::default(),FileTime::default());
    let ok=unsafe{GetProcessTimes(GetCurrentProcess(),&mut c,&mut e,&mut k,&mut u)};assert_ne!(ok,0);
    let ticks=|v:FileTime| ((v.high as u64)<<32)|v.low as u64;
    (ticks(k)+ticks(u)) as f64/10_000_000.
}
#[cfg(not(target_os="windows"))] fn cpu_seconds()->f64 {f64::NAN}
fn main()->Result<(),Box<dyn std::error::Error>> {
    slint::BackendSelector::new().backend_name("winit".into()).renderer_name("software".into()).select()?;
    let count=std::env::args().nth(1).unwrap_or_else(||"1".into()).parse::<i32>()?;
    let ui=MotionBench::new()?;ui.set_count(count);
    let renders=Rc::new(Cell::new(0u64));let copy=renders.clone();
    let hook=ui.window().set_rendering_notifier(move|state,_|{if matches!(state,slint::RenderingState::AfterRendering){copy.set(copy.get()+1);}}).is_ok();
    println!("ENV count={count} hook_supported={hook} logical_cpus={}",std::thread::available_parallelism()?.get());
    let timer=Rc::new(slint::Timer::default());let started=timer.clone();let weak=ui.as_weak();
    slint::Timer::single_shot(Duration::from_secs(1),move||{
        let tick=Cell::new(0u32);let weak=weak.clone();let stopped=started.clone();
        started.start(slint::TimerMode::Repeated,Duration::from_millis(16),move||{
            let ui=weak.unwrap();let index=tick.get();
            if index<200 {
                ui.set_shown(index%2==0);
                let start=Instant::now();let frame=ui.window().take_snapshot().expect("software frame");
                let ms=start.elapsed().as_secs_f64()*1000.;let pixel=frame.as_slice()[0];
                println!("FRAME count={count} sample={index} ms={ms:.6} pixel={}",pixel.r);
                tick.set(index+1);
            } else {
                ui.set_shown(false);stopped.stop();
                let renders=renders.clone();
                slint::Timer::single_shot(Duration::from_secs(1),move||{
                    let cpu=cpu_seconds();let start=Instant::now();let render_start=renders.get();
                    println!("IDLE_START count={count} cpu_seconds={cpu:.7}");
                    slint::Timer::single_shot(Duration::from_secs(60),move||{
                        let wall=start.elapsed().as_secs_f64();let delta=cpu_seconds()-cpu;
                        println!("IDLE count={count} seconds={wall:.6} cpu_seconds={delta:.7} one_core_percent={:.6} render_callbacks={} hook_supported={hook}",100.*delta/wall,renders.get()-render_start);
                        println!("RESULT=PASS");slint::quit_event_loop().unwrap();
                    });
                });
            }
        });
    });
    ui.run()?;Ok(())
}
