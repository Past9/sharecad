use dioxus::prelude::*;

mod components;

use components::main_window;

fn main() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let name = use_state(cx, || "blah".to_string());

    cx.render(rsx! {
        main_window {}
    })
}
