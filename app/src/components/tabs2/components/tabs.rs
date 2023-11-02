use dioxus::prelude::*;

use crate::components::Config;

#[allow(non_snake_case)]
#[inline_props]
pub fn Tabs<'a>(
    cx: Scope,
    config: &'a Config,
    on_config_changed: EventHandler<'a, Config>,
) -> Element {
    cx.render(rsx! {
        div {
            class: "tab-area",
            "TABS"
        }
    })
}
