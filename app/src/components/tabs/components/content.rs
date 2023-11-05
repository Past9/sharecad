use dioxus::prelude::*;

use crate::components::TabId;

#[derive(Props)]
pub struct TabContentProps<'a> {
    pub tab_id: TabId,
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn TabContent<'a>(cx: Scope<'a, TabContentProps<'a>>) -> Element<'a> {
    log::debug!("render tab {:?}", cx.props.tab_id);
    cx.render(rsx! {
        div {
            class: "tab-content",
            &cx.props.children
        }
    })
}
