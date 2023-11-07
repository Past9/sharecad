use crate::{
    command::CommandBus,
    components::{Config, LayoutComponent, TabId, TabsCommand},
};
use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct TabContentProps {
    pub tab_id: TabId,
}

#[allow(non_snake_case)]
pub fn DefaultTabContent<'a>(cx: Scope<'a, TabContentProps>) -> Element {
    cx.render(rsx! {
        div {
            "TAB CONTENT {cx.props.tab_id.num()}"
        }
    })
}

#[derive(Props)]
pub struct TabsProps<'a> {
    config: &'a Config,
    on_config_changed: EventHandler<'a, Config>,
    render_content: fn(Scope<'a, TabContentProps>) -> Element<'a>,
}

#[allow(non_snake_case)]
pub fn Tabs<'a>(cx: Scope<'a, TabsProps<'a>>) -> Element<'a> {
    let bus = cx.use_hook(|| CommandBus::new()).listen(cx, |cmd| {
        let new_config = cx.props.config.modify(cmd);
        if new_config != *cx.props.config {
            cx.props.on_config_changed.call(new_config);
        }
    });

    let generic_layout = use_memo(cx, &cx.props.config.layout, |layout| layout.clone().into());

    cx.render(rsx! {
        div {
            class: "tab-area",
            LayoutComponent {
                layout: generic_layout,
                tab_drop_offer: cx.props.config.drop_tab_offer.clone(),
                dragging_tab: cx.props.config.dragging_tab.clone(),
                bus: bus.clone(),
                render_content: cx.props.render_content
            }
        }
    })
}
