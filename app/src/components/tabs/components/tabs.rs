use dioxus::prelude::*;

use crate::components::{Command, Config, LayoutComponent};

use super::CommandBus;

#[derive(Props)]
pub struct TabsProps<'a> {
    config: &'a Config,
    on_config_changed: EventHandler<'a, Config>,
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

    cx.render(rsx! {
        div {
            class: "tab-area",
            LayoutComponent {
                layout: generic_layout,
                tab_drop_offer: cx.props.config.drop_tab_offer.clone(),
                dragging_tab: cx.props.config.dragging_tab.clone(),
                bus: bus
            }
        }
    })
}
