mod components;

use async_channel::{Receiver, Recv};
use components::MainWindow;
use dioxus::prelude::*;
use futures::{channel::mpsc::TrySendError, executor::block_on};
use gloo::events::EventListener;
use std::{
    collections::HashMap,
    pin::Pin,
    task::{Context, Poll},
};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::EventTarget;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("Failed to init logger");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    //let win = web_sys::window().unwrap();
    //let onmousemove = OnMouseMove::new();

    //use_shared_state_provider(cx, || onmousemove);

    cx.render(rsx! {
        MainWindow {}
    })
}

struct IdSource {
    id: u32,
}
impl IdSource {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn next(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}

struct WindowEvents {
    _listener: EventListener,
    receiver: Receiver<web_sys::Event>,
}
impl WindowEvents {
    pub fn on(event_name: &str) -> Self {
        let event_name = event_name.to_string();
        let win = web_sys::window().unwrap();
        let (sender, receiver) = async_channel::unbounded::<web_sys::Event>();

        let listener = EventListener::new(&win, event_name, move |evt| {
            if let Err(err) = block_on(sender.send(evt.to_owned())) {
                log::error!("window error: {}", err);
                log::error!("window event: {:?}", evt);
            }
        });

        Self {
            _listener: listener,
            receiver,
        }
    }

    pub async fn listen<F: FnMut(web_sys::Event)>(&self, mut cb: F) {
        loop {
            if let Ok(next) = self.next().await {
                cb(next);
            }
        }
    }

    pub fn next(&self) -> Recv<'_, web_sys::Event> {
        self.receiver.recv()
    }

    pub fn receiver(&self) -> &Receiver<web_sys::Event> {
        &self.receiver
    }
}

pub struct UseWindowEvent {
    coroutine: Coroutine<()>,
}

pub fn use_window_event<F: FnMut(web_sys::Event) + 'static>(
    cx: &ScopeState,
    event_name: &str,
    cb: F,
) -> UseWindowEvent {
    let cr = use_coroutine(cx, |rx: UnboundedReceiver<()>| async move {
        WindowEvents::on(event_name).listen(cb).await;
    });
    UseWindowEvent {
        coroutine: cr.to_owned(),
    }
}
