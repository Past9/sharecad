use super::CommandBus;
use crate::components::{DropTabOffer, GenericLayout, GroupComponent, SplitComponent, TabId};
use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct LayoutComponentProps<'a> {
    layout: &'a GenericLayout,
    #[props(!optional)]
    tab_drop_offer: Option<DropTabOffer>,
    #[props(!optional)]
    dragged_tab: Option<TabId>,
    bus: CommandBus,
}

#[allow(non_snake_case)]
pub fn LayoutComponent<'a>(cx: Scope<'a, LayoutComponentProps>) -> Element<'a> {
    match cx.props.layout {
        GenericLayout::Group(group) => cx.render(rsx! {
            GroupComponent {
                group: &group,
                tab_drop_offer: cx.props.tab_drop_offer.clone(),
                dragged_tab: cx.props.dragged_tab.clone(),
                bus: cx.props.bus.clone()
            }
        }),
        GenericLayout::Split(split) => cx.render(rsx! {
            SplitComponent {
                split: split,
                tab_drop_offer: cx.props.tab_drop_offer.clone(),
                dragged_tab: cx.props.dragged_tab.clone(),
                bus: cx.props.bus.clone()
            }
        }),
    }
}
