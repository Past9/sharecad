use async_channel::{Receiver, Recv};
use dioxus::prelude::*;
use futures::executor::block_on;
use gloo::events::EventListener;

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
}

pub struct UseWindowEvent {
    _coroutine: Coroutine<()>,
}

pub fn use_window_event<F: FnMut(web_sys::Event) + 'static>(
    cx: &ScopeState,
    event_name: &str,
    cb: F,
) -> UseWindowEvent {
    let event_name = event_name.to_string();
    let cr = use_coroutine(cx, |_rx: UnboundedReceiver<()>| {
        //
        async move {
            WindowEvents::on(&event_name).listen(cb).await;
        }
    });
    UseWindowEvent {
        _coroutine: cr.to_owned(),
    }
}

pub fn use_window_mouseup<F: FnMut(MouseData) + 'static>(
    cx: &ScopeState,
    mut cb: F,
) -> UseWindowEvent {
    use_window_event(cx, "mouseup", move |evt| cb(make_mousedata(evt)))
}

pub fn use_window_mousemove<F: FnMut(MouseData) + 'static>(
    cx: &ScopeState,
    mut cb: F,
) -> UseWindowEvent {
    use_window_event(cx, "mousemove", move |evt| cb(make_mousedata(evt)))
}

fn make_mousedata(event: web_sys::Event) -> MouseData {
    use dioxus::events::*;

    match event.type_().as_str() {
        "click" | "contextmenu" | "dblclick" | "doubleclick" | "mousedown" | "mouseenter"
        | "mouseleave" | "mousemove" | "mouseout" | "mouseover" | "mouseup" => {
            MouseData::from(event)
        }

        _ => panic!(
            "Event type {} cannot be turned into MouseData",
            event.type_()
        ),
    }
}

fn make_dragdata(event: web_sys::Event) -> DragData {
    use dioxus::events::*;

    match event.type_().as_str() {
        "drag" | "dragend" | "dragenter" | "dragexit" | "dragleave" | "dragover" | "dragstart"
        | "drop" => DragData {
            mouse: MouseData::from(event),
        },

        _ => panic!(
            "Event type {} cannot be turned into DragData",
            event.type_()
        ),
    }
}
