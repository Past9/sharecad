use std::cell::Cell;

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

    /*
    pub fn use_window_event<T, F, D>(
        cx: &ScopeState,
        event_name: &str,
        dependencies: D,
        future: impl FnOnce(web_sys::Event, D::Out) -> F,
    ) where
        T: 'static,
        F: std::future::Future<Output = ()> + 'static,
        D: UseFutureDep,
    {
         */

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

/*
pub struct UseWindowEvent {
    _coroutine: Coroutine<()>,
}

pub fn use_win_event<F, D>(
    cx: &ScopeState,
    evt: &str,
    dependencies: D,
    future: impl FnOnce(D::Out) -> F,
) where
    F: std::future::Future<Output = ()> + 'static,
    D: UseFutureDep,
{
    struct UseWinEvent {
        needs_regen: bool,
        task: Cell<Option<TaskId>>,
        dependencies: Vec<Box<dyn std::any::Any>>,
    }

    let state = cx.use_hook(move || UseWinEvent {
        needs_regen: true,
        task: Cell::new(None),
        dependencies: Vec::new(),
    });

    if dependencies.clone().apply(&mut state.dependencies) || state.needs_regen {
        state.needs_regen = false;
        let fut = future(dependencies.out());
        state.task.set(Some(cx.push_future(async move {
            fut.await;
        })));
    }
}
*/

/*
pub fn use_window_event<F, D>(
    cx: &ScopeState,
    event_name: &str,
    dependencies: D,
    future: impl (FnOnce(D::Out) -> F) + 'static,
) where
    F: FnMut(web_sys::Event) + 'static,
    D: UseFutureDep,
{
    let event_name = event_name.to_string();
    use_future(cx, dependencies, |dep| async {
        let f = future(dep);
        WindowEvents::on(event_name).listen(f)
    });
}
*/

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

/*
POTENTIALLY CORRECT BUT DOES NOT USE DEPS
pub fn use_window_event<F, D>(cx: &ScopeState, event_name: &str, dependencies: D, cb: F)
where
    F: FnMut(web_sys::Event) + 'static,
    D: UseFutureDep,
{
    let event_name = event_name.to_string();
    let cr = use_future(cx, dependencies, |deps| {
        //
        async {
            WindowEvents::on(event_name).listen(cb).await;
        }
    });
}
 */

/*
pub fn use_window_mouseup<F, D>(cx: &ScopeState, dependencies: D, future: impl FnOnce(D::Out) -> F)
where
    F: FnMut(MouseData) + 'static,
    D: UseFutureDep,
{
    use_window_event(cx, "mouseup", dependencies, |dep| {
        let mut f = future(dep);
        |web_sys_evt| {
            let mouse_data = make_mousedata(web_sys_evt);
            f(mouse_data);
        }
    })
}
 */

/*
pub fn use_window_mousemove<F, D>(
    cx: &ScopeState,
    dependencies: D,
    future: impl FnOnce(D::Out) -> F,
) where
    F: FnMut(MouseData) + 'static,
    D: UseFutureDep,
{
    use_window_event(cx, "mousemove", dependencies, |dep| {
        let mut f = future(dep);
        let cb = |web_sys_evt| {
            let mouse_data = make_mousedata(web_sys_evt);
            f(mouse_data)
        };

        cb
    })
}
*/

/*
pub fn use_window_event<'a, F: FnMut(web_sys::Event) + 'static>(
    cx: &'a ScopeState,
    event_name: &str,
    cb: F,
) -> UseWindowEvent {
    let event_name = event_name.to_string();
    let cr = use_coroutine(cx, |_rx: UnboundedReceiver<()>| {
        //
        async {
            WindowEvents::on(event_name).listen(cb).await;
        }
    });
    UseWindowEvent {
        _coroutine: cr.to_owned(),
    }
}

pub fn use_window_mouseup<'a, F: FnMut(MouseData) + 'static>(cx: &'a ScopeState, mut cb: F) {
    use_window_event(cx, "mouseup", move |evt| cb(make_mousedata(evt)))
}

pub fn use_window_mousemove<'a, F: FnMut(MouseData) + 'static>(cx: &'a ScopeState, mut cb: F) {
    use_window_event(cx, "mousemove", move |evt| cb(make_mousedata(evt)))
}
 */

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
