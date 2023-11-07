use super::TabContentProps;
use crate::{
    command::CommandBus,
    components::{
        DraggingTab, DropTabOffer, GenericLayout, GroupComponent, SplitComponent, TabsCommand,
    },
};
use dioxus::prelude::*;

#[derive(Props)]
pub struct LayoutComponentProps<'a> {
    layout: &'a GenericLayout,
    #[props(!optional)]
    tab_drop_offer: Option<DropTabOffer>,
    #[props(!optional)]
    dragging_tab: Option<DraggingTab>,
    bus: CommandBus<TabsCommand>,
    render_content: fn(Scope<'a, TabContentProps>) -> Element<'a>,
}

#[allow(non_snake_case)]
pub fn LayoutComponent<'a>(cx: Scope<'a, LayoutComponentProps<'a>>) -> Element<'a> {
    match cx.props.layout {
        GenericLayout::Group(group) => cx.render(rsx! {
            GroupComponent {
                group: &group,
                tab_drop_offer: cx.props.tab_drop_offer.clone(),
                dragging_tab: cx.props.dragging_tab.clone(),
                bus: cx.props.bus.clone(),
                render_content: cx.props.render_content
            }
        }),
        GenericLayout::Split(split) => cx.render(rsx! {
            SplitComponent {
                split: split,
                tab_drop_offer: cx.props.tab_drop_offer.clone(),
                dragging_tab: cx.props.dragging_tab.clone(),
                bus: cx.props.bus.clone(),
                render_content: cx.props.render_content
            }
        }),
    }
}
