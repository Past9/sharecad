use dioxus::{core::DynamicNode, prelude::*};

use crate::components::{Command, Config, LayoutComponent, Tab, TabId};

use super::CommandBus;

#[derive(Props, PartialEq)]
pub struct TabContentProps {
    tab_id: TabId,
}

#[allow(non_snake_case)]
pub fn TabContent<'a>(cx: Scope<'a, TabContentProps>) -> Element {
    cx.render(rsx! {
        div {
            "TAB CONTENT {cx.props.tab_id.num()}"
        }
    })
}

//trait Component: FnMut(Scope<TabContentProps>) -> Element {} // + Sized {}

#[derive(Props)]
pub struct TabsProps<'a> {
    config: &'a Config,
    on_config_changed: EventHandler<'a, Config>,
    render_content: fn(Scope<'a, TabContentProps>) -> Element<'a>,
}

#[allow(non_snake_case)]
pub fn Tabs<'a>(cx: Scope<'a, TabsProps<'a>>) -> Element<'a> {
    let (sender, receiver) = cx.use_hook(|| async_channel::unbounded::<Command>());
    let bus = CommandBus::new(sender.clone());

    let next_command = use_state::<Option<Command>>(cx, || None);

    let _cr = {
        to_owned![next_command, receiver];
        use_coroutine(cx, |_rx: UnboundedReceiver<()>| async move {
            loop {
                if let Ok(command) = receiver.recv().await {
                    next_command.set(Some(command));
                }
            }
        })
    };

    if let Some(command) = &**next_command {
        next_command.set(None);
        let new_config = cx.props.config.modify(command);
        if new_config != *cx.props.config {
            cx.props.on_config_changed.call(new_config);
        }
    }

    let generic_layout = use_memo(cx, &cx.props.config.layout, |layout| layout.clone().into());

    /*
    for child in cx.props.children.iter() {
        log::debug!("child {:#?}", child);
        for node in child.dynamic_nodes.iter() {
            if let DynamicNode::Component(comp) = node {
                //
            }
            log::debug!("node {:#?}", node);
        }
        for attr in child.dynamic_attrs.iter() {
            log::debug!("attr {} = {:?}", attr.name, attr.value);
        }
    }

    let tab_children = use_memo(cx, (&cx.props.children), |(children)| {
        children
            .iter()
            .filter(|child| child.dynamic_attrs.iter().any(|attr| attr.name == "tab_id"))
            .collect::<Vec<_>>()
    });
     */

    //(cx.props.render_content)(TabId::new(10));

    /*
    cx.scope.component(
        cx.props.render_content,
        TabContentProps {
            tab_id: TabId::new(1),
        },
        "ActiveTab",
    );
     */

    let ActiveTab = cx.props.render_content;

    cx.render(rsx! {
        //&cx.props.children
        ActiveTab {
            tab_id: TabId::new(12)
        }
        div {
            class: "tab-area",
            LayoutComponent {
                layout: generic_layout,
                tab_drop_offer: cx.props.config.drop_tab_offer.clone(),
                dragging_tab: cx.props.config.dragging_tab.clone(),
                bus: bus,
            }
        }
    })
}
