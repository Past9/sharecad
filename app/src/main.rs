use std::{collections::HashMap, marker::PhantomData};

use dioxus::prelude::*;

mod components;
use components::MainWindow;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::Event;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("Failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    /*
    let win = web_sys::window().unwrap(); //.document().unwrap();

    log::debug!("win {:?}", win);

    let on_mouse_down = Closure::wrap(Box::new(|_: Event| {
        log::debug!("MOUSE DOWN!");
    }) as Box<dyn FnMut(_)>);

    log::debug!("on_mouse_down {:?}", on_mouse_down);

    log::debug!("add handlers");

    win.add_event_listener_with_callback("mousedown", on_mouse_down.as_ref().unchecked_ref())
        .unwrap();

    log::debug!("handlers added");
    */

    /*
    let mut win_events = WindowEvents::new();

    let window = web_sys::window().unwrap();

    let mouse_move_handler = Closure::wrap(Box::new(|| {
        for handler in win_events.on_mouse_move.values_mut() {
            handler();
        }
    }) as Box<dyn FnMut()>);

    let mouse_up_handler = Closure::wrap(Box::new(|| {
        for handler in win_events.on_mouse_up.values_mut() {
            handler();
        }
    }) as Box<dyn FnMut()>);

    window
        .add_event_listener_with_callback("mousemove", mouse_move_handler.as_ref().unchecked_ref())
        .unwrap();

    window
        .add_event_listener_with_callback("mouseup", mouse_up_handler.as_ref().unchecked_ref())
        .unwrap();

    mouse_move_handler.forget();
    mouse_up_handler.forget();

    win_events.on_mouse_up(Box::new(|| {
        log::debug!("Foo");
    }));
     */

    cx.render(rsx! {
        MainWindow {}
    })
}

struct IdSource(u32);
impl IdSource {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn next(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}

pub struct WindowEvents<'a> {
    phantom: PhantomData<&'a ()>,
    //window: web_sys::Window,
    on_mouse_move_ids: IdSource,
    on_mouse_up_ids: IdSource,

    on_mouse_move: HashMap<u32, Box<dyn FnMut()>>,
    on_mouse_up: HashMap<u32, Box<dyn FnMut()>>,
}
impl<'a> WindowEvents<'a> {
    pub fn new() -> Self {
        //let window = web_sys::window().unwrap();

        let window_events = Self {
            phantom: PhantomData,
            //window,
            on_mouse_move_ids: IdSource::new(),
            on_mouse_up_ids: IdSource::new(),
            on_mouse_move: HashMap::new(),
            on_mouse_up: HashMap::new(),
        };

        /*
        let mouse_move_handler = Closure::wrap(Box::new(move || {
            for handler in window_events.on_mouse_move.values_mut() {
                handler();
            }
        }) as Box<dyn FnMut()>);

        mouse_move_handler.forget();
         */

        window_events
    }

    pub fn on_mouse_move(&mut self, handler: Box<dyn FnMut()>) -> u32 {
        let id = self.on_mouse_move_ids.next();
        self.on_mouse_move.insert(id, handler);
        id
    }

    pub fn on_mouse_up(&mut self, handler: Box<dyn FnMut()>) -> u32 {
        let id = self.on_mouse_up_ids.next();
        self.on_mouse_up.insert(id, handler);
        id
    }
}
