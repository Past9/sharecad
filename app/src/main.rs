mod components;

use async_channel::Receiver;
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
    let win = web_sys::window().unwrap();
    let onmousemove = OnMouseMove::new(&win);

    use_shared_state_provider(cx, || onmousemove);

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

struct OnMouseMove {
    _listener: EventListener,
    receiver: Receiver<()>,
}
impl OnMouseMove {
    pub fn new(target: &EventTarget) -> Self {
        let (sender, receiver) = async_channel::unbounded();

        let listener = EventListener::new(&target, "mousemove", move |_evt| {
            let res = block_on(sender.send(()));
            log::debug!("send move {:?}", res);
        });

        Self {
            _listener: listener,
            receiver,
        }
    }

    pub fn receiver(&self) -> &Receiver<()> {
        &self.receiver
    }
}
/*
impl Stream for OnMouseMove {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}
 */
