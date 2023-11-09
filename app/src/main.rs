mod command;
mod components;
mod on_resize;
mod window_events;

use components::MainWindow;
use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(log::LevelFilter::Debug).expect("Failed to init logger");

    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        MainWindow {}
    })
}
