use super::CommandBus;
use crate::components::{DraggingTab, DropTabOffer, GenericLayout, GroupComponent, SplitComponent};
use dioxus::prelude::*;

#[derive(Props)]
pub struct LayoutComponentProps<'a> {
    layout: &'a GenericLayout,
    #[props(!optional)]
    tab_drop_offer: Option<DropTabOffer>,
    #[props(!optional)]
    dragging_tab: Option<DraggingTab>,
    bus: CommandBus,
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
            }
        }),
        GenericLayout::Split(split) => cx.render(rsx! {
            SplitComponent {
                split: split,
                tab_drop_offer: cx.props.tab_drop_offer.clone(),
                dragging_tab: cx.props.dragging_tab.clone(),
                bus: cx.props.bus.clone()
            }
        }),
    }
}
