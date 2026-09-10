// SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
// SPDX-License-Identifier: GPL-3.0-only
use slint::{
    ComponentHandle, Model, ModelTracker,
    platform::{Key, PointerEventButton, WindowEvent},
};
use std::{cell::Cell, rc::Rc, time::Duration};
slint::include_modules!();

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
struct CountedModel {
    data: slint::VecModel<NavigationEntry>,
    reads: Cell<usize>,
}
impl Model for CountedModel {
    type Data = NavigationEntry;
    fn row_count(&self) -> usize { self.data.row_count() }
    fn row_data(&self, index:usize) -> Option<NavigationEntry> { self.reads.set(self.reads.get()+1);self.data.row_data(index) }
    fn set_row_data(&self,index:usize,data:NavigationEntry) {self.data.set_row_data(index,data);}
    fn model_tracker(&self) -> &dyn ModelTracker {self.data.model_tracker()}
}
fn model(count:usize) -> Rc<CountedModel> {
    Rc::new(CountedModel { data:slint::VecModel::from((0..count).map(|i|NavigationEntry {
        id:format!("row-{i}").into(),text:format!("Row {i}").into(),enabled:true,
        kind:NavigationEntryKind::Destination,..NavigationEntry::default()
    }).collect::<Vec<_>>()),reads:Cell::new(0) })
}
fn entry(id:&str,kind:NavigationEntryKind,parent:&str,depth:i32) -> NavigationEntry {
    NavigationEntry{id:id.into(),text:id.into(),kind,parent_id:parent.into(),depth,enabled:true,..NavigationEntry::default()}
}
fn fixture() -> Rc<CountedModel> {
    use NavigationEntryKind::{Destination,DestinationGroup,Group,Header,Separator};
    let mut g=entry("g",Group,"",0);g.has_children=true;g.expanded=true;
    let mut b=entry("b",Destination,"g",1);b.enabled=false;
    let mut ig=entry("ig",DestinationGroup,"g",1);ig.has_children=true;
    let mut long=entry("long",Destination,"",0);long.text="A deliberately long navigation label / 很长的导航名称 must elide".into();
    Rc::new(CountedModel{data:slint::VecModel::from(vec![entry("heading",Header,"",0),g,entry("a",Destination,"g",1),b,ig,entry("c",Destination,"ig",2),entry("separator",Separator,"",0),entry("z",Destination,"",0),long]),reads:Cell::new(0)})
}
fn expanded(m:&CountedModel,index:usize,value:bool) {let mut row=m.data.row_data(index).unwrap();row.expanded=value;m.set_row_data(index,row);}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()?;
    let ui = NavigationCheck::new()?;
    let Some(path) = std::env::args().nth(1) else {
        return Ok(ui.run()?);
    };
    let directory = std::path::PathBuf::from(path);
    std::fs::create_dir_all(&directory)?;
    let failure = Rc::new(Cell::new(false));
    let failed = failure.clone();
    let weak = ui.as_weak();
    let step = Cell::new(0);
    let current = std::cell::RefCell::new(model(0));
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
            let counts=[16usize,64,256,257];
            let batch=step.get()/5;
            if batch<counts.len() {
                let count=counts[batch];
                match step.get()%5 {
                    0 => { let m=model(count);let start=std::time::Instant::now();ui.set_entries(slint::ModelRc::from(m.clone()));let setter_ms=start.elapsed().as_secs_f64()*1000.;let validation_start=std::time::Instant::now();let valid=ui.get_model_valid();println!("MEASURE n={count} phase=validation valid={valid} ms={:.4}",validation_start.elapsed().as_secs_f64()*1000.);let render_start=std::time::Instant::now();let _=w.take_snapshot().unwrap();println!("MEASURE n={count} phase=assign setter_ms={setter_ms:.4} snapshot_ms={:.4} reads={}",render_start.elapsed().as_secs_f64()*1000.,m.reads.get());*current.borrow_mut()=m; }
                    1 => { let m=current.borrow();println!("MEASURE n={count} phase=settled reads={}",m.reads.get());shot(&format!("rows-{count}"));let before=ui.get_commands();pointer(w,80.,20.,true);pointer(w,80.,20.,false);check(&format!("whole model size {count} obeys bound"),ui.get_commands()==before+if count<=256 {1}else{0});m.reads.set(0);ui.set_selected(format!("row-{}",batch+1).into()); }
                    2 => { println!("MEASURE n={count} phase=selected-only reads={}",current.borrow().reads.get());let m=model(count);let start=std::time::Instant::now();ui.set_entries(slint::ModelRc::from(m.clone()));println!("MEASURE n={count} phase=replace setter_ms={:.4}",start.elapsed().as_secs_f64()*1000.);*current.borrow_mut()=m; }
                    3 => { let m=current.borrow();println!("MEASURE n={count} phase=replaced reads={}",m.reads.get());m.reads.set(0);let mut invalid=m.data.row_data(1).unwrap();invalid.id="row-0".into();m.set_row_data(1,invalid); }
                    _ => { let before=ui.get_commands();pointer(w,80.,20.,true);pointer(w,80.,20.,false);println!("MEASURE n={count} phase=invalid-row-change reads={} commands={}",current.borrow().reads.get(),ui.get_commands()-before);check(&format!("row-change duplicate ID rejects {count} model"),ui.get_commands()==before); }
                }
            } else {
                match step.get() {
                    20 => { let m=fixture();ui.set_entries(slint::ModelRc::from(m.clone()));*current.borrow_mut()=m;let mut disabled=entry("f2",NavigationEntryKind::Destination,"",0);disabled.enabled=false;ui.set_footer(slint::ModelRc::new(slint::VecModel::from(vec![entry("f1",NavigationEntryKind::Destination,"",0),disabled])));ui.set_selected("".into()); }
                    21 => { check("nested fixture is valid",ui.get_model_valid());shot("expanded");let before=ui.get_commands();pointer(w,80.,48.,true);pointer(w,80.,48.,false);check("group native button requests collapse without invocation",ui.get_commands()==before && ui.get_last_expansion()=="g" && !ui.get_requested_expanded());check("expansion remains host-owned",current.borrow().data.row_data(1).unwrap().expanded);expanded(&current.borrow(),1,false); }
                    22 => { shot("collapsed");key(w,Key::DownArrow.into()); }
                    23 => { key(w,Key::Return.into());check("Down skips collapsed children and separator",ui.get_last_id()=="z");key(w,Key::Home.into()); }
                    24 => { key(w,Key::RightArrow.into());check("Home and Right request group expansion",ui.get_last_expansion()=="g" && ui.get_requested_expanded());expanded(&current.borrow(),1,true); }
                    25 => { key(w,Key::DownArrow.into()); }
                    26 => { key(w,Key::Return.into());check("Down reaches first enabled child",ui.get_last_id()=="a");key(w,Key::DownArrow.into()); }
                    27 => { key(w,Key::Return.into());check("Down skips disabled child",ui.get_last_id()=="ig");check("destination invocation does not seize selected state",ui.get_selected().is_empty());key(w,Key::RightArrow.into());check("destination-group expansion remains separate",ui.get_last_expansion()=="ig" && ui.get_requested_expanded());expanded(&current.borrow(),4,true); }
                    28 => { key(w,Key::DownArrow.into()); }
                    29 => { key(w,Key::Return.into());check("expanded grandchild is keyboard reachable",ui.get_last_id()=="c");key(w,Key::LeftArrow.into()); }
                    30 => { key(w,Key::Return.into());check("Left recovers enabled parent focus",ui.get_last_id()=="ig");key(w,Key::LeftArrow.into());check("Left on expanded branch requests collapse",ui.get_last_expansion()=="ig" && !ui.get_requested_expanded());expanded(&current.borrow(),4,false);key(w,Key::Home.into()); }
                    31 => { key(w,Key::Tab.into());let before=ui.get_expansions();key(w,Key::Return.into());check("Tab visits separate group arrow once",ui.get_expansions()==before+1 && ui.get_last_expansion()=="g");key(w,Key::Tab.into());key(w,Key::Space.into());check("Tab and Space reach next child",ui.get_last_id()=="a");let mut disabled=current.borrow().data.row_data(2).unwrap();disabled.enabled=false;current.borrow().set_row_data(2,disabled); }
                    32 => { let before=ui.get_expansions();key(w,Key::Return.into());check("disabling focused child recovers enabled ancestor",ui.get_expansions()==before+1 && ui.get_last_expansion()=="g");let m=model(64);ui.set_entries(slint::ModelRc::from(m.clone()));*current.borrow_mut()=m;ui.invoke_focus_anchor(); }
                    33 => { key(w,Key::Tab.into());key(w,Key::End.into()); }
                    34 => { key(w,Key::Return.into());check("End focuses last primary row",ui.get_last_id()=="row-63");shot("scrolled-end");pointer(w,80.,420.,true);pointer(w,80.,420.,false);check("focused last row is inside the scrolled viewport",ui.get_last_id()=="row-63");key(w,Key::DownArrow.into()); }
                    35 => { key(w,Key::Return.into());check("Down crosses from primary end to enabled footer",ui.get_last_id()=="f1");key(w,Key::UpArrow.into()); }
                    36 => { key(w,Key::Return.into());check("Up crosses from footer to primary end",ui.get_last_id()=="row-63");let m=fixture();ui.set_entries(slint::ModelRc::from(m.clone()));*current.borrow_mut()=m;ui.set_compact(true);ui.set_dark(true); }
                    37 => { shot("compact");
                        let pixels=w.take_snapshot().unwrap();
                        for (name,cy) in [("group",20usize),("nested destination-group",140),("destination",180)] {
                            let mut xs=Vec::new();
                            for y in cy-10..cy+10 { for x in 8usize..46 {
                                let offset=(y*pixels.width() as usize+x)*4;
                                let rgba=&pixels.as_bytes()[offset..offset+4];
                                if rgba[0]>140 && rgba[1]>140 && rgba[2]>140 { xs.push(x); }
                            } }
                            let center=xs.iter().min().zip(xs.iter().max()).map(|(l,r)|(*l+*r+1) as f32/2.);
                            check(&format!("compact {name} painted icon centers on the 54px rail"),center.is_some_and(|x|(x-27.).abs()<=0.5));
                        }
                        pointer(w,14.,140.,true);pointer(w,14.,140.,false);check("compact label region invokes destination-group",ui.get_last_id()=="ig");let before=ui.get_commands();let expansions=ui.get_expansions();pointer(w,38.,140.,true);pointer(w,38.,140.,false);check("compact former chevron area belongs to the destination",ui.get_commands()==before+1 && ui.get_expansions()==expansions);key(w,Key::RightArrow.into());check("compact destination-group retains keyboard expansion",ui.get_last_expansion()=="ig" && ui.get_expansions()==expansions+1);let before=ui.get_commands();pointer(w,14.,100.,true);pointer(w,14.,100.,false);check("disabled compact label emits no command",ui.get_commands()==before);ui.set_selected("ig".into()); }
                    38 => { shot("compact-selected");let mut invalid=ui.get_footer().row_data(0).unwrap();invalid.id="ig".into();ui.get_footer().set_row_data(0,invalid); }
                    39 => { check("cross-region row change invalidates both models",!ui.get_model_valid());let before=ui.get_commands();pointer(w,14.,140.,true);pointer(w,14.,140.,false);check("invalid combined model emits no command",ui.get_commands()==before);ui.set_footer(slint::ModelRc::new(slint::VecModel::<NavigationEntry>::from(vec![])));ui.set_compact(false); }
                    40 => { let m=model(6);ui.set_entries(slint::ModelRc::from(m.clone()));*current.borrow_mut()=m;ui.set_selected("row-0".into());ui.set_dark(false); }
                    41 => {
                        pointer(w,80.,20.,true);pointer(w,80.,20.,false);
                        shot("flat-selected-pointer");
                        let before=ui.get_commands();
                        w.dispatch_event(WindowEvent::KeyPressed {text:Key::Space.into()});
                        w.dispatch_event(WindowEvent::KeyPressed {text:Key::Space.into()});
                        check("holding/repeating Space does not invoke before release",ui.get_commands()==before);
                        w.dispatch_event(WindowEvent::KeyReleased {text:Key::Space.into()});
                        check("Space release invokes exactly once",ui.get_commands()==before+1);
                        let before=ui.get_commands();
                        w.dispatch_event(WindowEvent::KeyPressed {text:Key::Space.into()});key(w,Key::Escape.into());
                        w.dispatch_event(WindowEvent::KeyReleased {text:Key::Space.into()});
                        check("Escape cancels pending navigation activation",ui.get_commands()==before);
                        pointer(w,80.,20.,true);pointer(w,810.,300.,false);
                        check("pointer release outside navigation target cancels",ui.get_commands()==before);
                        w.dispatch_event(WindowEvent::KeyPressed {text:Key::Space.into()});ui.invoke_focus_anchor();
                        w.dispatch_event(WindowEvent::KeyReleased {text:Key::Space.into()});
                        check("focus loss cancels held navigation key",ui.get_commands()==before);
                    }
                    42 => { ui.invoke_focus_anchor();key(w,Key::Tab.into()); }
                    43 => {
                        shot("flat-keyboard-focus");let before=ui.get_commands();key(w,Key::Return.into());
                        check("keyboard focus and Return invoke one current row",ui.get_commands()==before+1 && ui.get_last_id()=="row-0");
                        check("selection remains host controlled",ui.get_selected()=="row-0");
                        w.dispatch_event(WindowEvent::KeyPressed {text:Key::Space.into()});
                        let mut row=current.borrow().data.row_data(0).unwrap();row.enabled=false;current.borrow().set_row_data(0,row);
                    }
                    44 => {let before=ui.get_commands();w.dispatch_event(WindowEvent::KeyReleased {text:Key::Space.into()});check("disable while held cancels navigation activation",ui.get_commands()==before);}
                    45 => { for n in [16usize,64,256,257] { for sample in 0..30 { let m=model(n);ui.set_entries(slint::ModelRc::from(m.clone()));let start=std::time::Instant::now();let valid=ui.get_model_valid();println!("BENCH n={n} sample={sample} valid={valid} ms={:.6} reads={}",start.elapsed().as_secs_f64()*1000.,m.reads.get());check(&format!("sample {sample} validates {n}"),valid==(n<=256)); } } }
                    46 => { let m=fixture();ui.set_entries(slint::ModelRc::from(m.clone()));*current.borrow_mut()=m;ui.set_compact(true);ui.set_show_header(true);ui.set_query("retained query".into()); }
                    47 => {
                        shot("compact-header");
                        let before=ui.get_expansions();pointer(w,27.,64.,true);pointer(w,27.,64.,false);
                        check("compact first row directly follows one header without search placeholder",ui.get_expansions()==before+1 && ui.get_last_expansion()=="g");
                        check("compact group icon requests expansion once",ui.get_last_expansion()=="g");
                        pointer(w,27.,22.,true);pointer(w,27.,22.,false);
                        check("public NavigationView toggle expands through host",!ui.get_compact() && ui.get_pane_requests()==1);
                    }
                    48 => { shot("expanded-header");check("expansion restores retained search state",ui.get_query()=="retained query");pointer(w,27.,22.,true);pointer(w,27.,22.,false);check("same public toggle collapses through host",ui.get_compact() && ui.get_pane_requests()==2); }
                    49 => { println!("RESULT={}",if failed.get(){"FAIL"}else{"PASS"});slint::quit_event_loop().unwrap(); }
                    _ => {}
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
