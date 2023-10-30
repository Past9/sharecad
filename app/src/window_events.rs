use async_channel::{Receiver, Recv};
use dioxus::prelude::*;
use futures::executor::block_on;
use gloo::events::EventListener;

struct WindowEvents {
    _listener: EventListener,
    receiver: Receiver<web_sys::Event>,
}
impl WindowEvents {
    pub fn on(event_name: String) -> Self {
        log::debug!("new WindowEvents for {}", event_name);
        let win = web_sys::window().unwrap();
        let (sender, receiver) = async_channel::unbounded::<web_sys::Event>();

        let listener = EventListener::new(&win, event_name, move |evt| {
            if let Err(err) = block_on(sender.send(evt.to_owned())) {
                log::error!("window error: {}", err);
                log::error!("previous error occurred on window event: {:#?}", evt);
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

pub struct UseWindowEvent<'a> {
    _use_future: &'a UseFuture<()>,
}

pub fn use_window_event<'a, F1, F2, D>(
    cx: &'a ScopeState,
    event_name: &str,
    dependencies: D,
    f: F1,
) -> UseWindowEvent<'a>
where
    F1: FnOnce(D::Out) -> F2,
    F2: FnMut(web_sys::Event) + 'static,
    D: UseFutureDep,
{
    let event_name = event_name.to_string();
    let use_fut = use_future(cx, dependencies, |deps| {
        //
        let cb = f(deps);
        async {
            WindowEvents::on(event_name).listen(cb).await;
        }
    });
    UseWindowEvent {
        _use_future: use_fut,
    }
}

pub fn use_window_mousemove<'a, F1, F2, D>(
    cx: &ScopeState,
    dependencies: D,
    f: F1,
) -> UseWindowEvent<'_>
where
    F1: FnOnce(D::Out) -> F2,
    F2: FnMut(MouseData) + 'static,
    D: UseFutureDep,
{
    use_window_event(cx, "mousemove", dependencies, |deps| {
        let mut f2 = f(deps);
        move |websys_evt| {
            //
            f2(make_mousedata(websys_evt));
        }
    })
}

pub fn use_window_mouseup<F1, F2, D>(cx: &ScopeState, dependencies: D, f: F1) -> UseWindowEvent<'_>
where
    F1: FnOnce(D::Out) -> F2,
    F2: FnMut(MouseData) + 'static,
    D: UseFutureDep,
{
    use_window_event(cx, "mouseup", dependencies, |deps| {
        let mut f2 = f(deps);
        move |websys_evt| {
            //
            f2(make_mousedata(websys_evt));
        }
    })
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
