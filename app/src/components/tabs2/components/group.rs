use dioxus::prelude::*;

use crate::{
    components::{
        tabs2::SplitDirection, Command, DraggingTab, DropTabOffer, Group, HeaderComponent,
    },
    on_resize::{ComponentSize, OnResize},
};

use super::CommandBus;

#[derive(PartialEq, Props)]
pub struct GroupComponentProps<'a> {
    group: &'a Group,
    #[props(!optional)]
    tab_drop_offer: Option<DropTabOffer>,
    #[props(!optional)]
    dragging_tab: Option<DraggingTab>,
    bus: CommandBus,
}

#[allow(non_snake_case)]
pub fn GroupComponent<'a>(cx: Scope<'a, GroupComponentProps<'a>>) -> Element<'a> {
    let body_drop_size = use_ref(cx, ComponentSize::default);
    let on_body_drop_resize = use_state(cx, || {
        to_owned![body_drop_size];
        OnResize::new(move |new_size: ComponentSize| body_drop_size.set(new_size))
    });

    {
        to_owned![on_body_drop_resize];
        use_on_unmount(cx, move || {
            on_body_drop_resize.unmount();
        });
    }

    let body_drop_target_class: &Option<&'static str> = use_memo(
        cx,
        (&cx.props.tab_drop_offer, cx.props.group),
        |(tab_drop_offer, group)| match tab_drop_offer {
            Some(offer) => {
                if offer.group_id() == group.id {
                    Some(match offer {
                        DropTabOffer::InGroup { .. } => "in-group",
                        DropTabOffer::Split { direction, .. } => match direction {
                            SplitDirection::Left => "split-left",
                            SplitDirection::Right => "split-right",
                            SplitDirection::Up => "split-up",
                            SplitDirection::Down => "split-down",
                        },
                    })
                } else {
                    None
                }
            }
            None => None,
        },
    );

    cx.render(rsx! {
        div {
            class: "group",
            div {
                class: "group-header",
                for (index, tab) in cx.props.group.tabs.iter().enumerate() {
                    rsx! {
                        if let Some(DropTabOffer::InGroup {
                            group_id: offer_group_id,
                            index: offer_index
                        }) = cx.props.tab_drop_offer {
                            if offer_group_id == cx.props.group.id && offer_index == index {
                                rsx! {
                                    div {
                                        class: "tab-drop-target"
                                    }
                                }
                            } else {
                                rsx! { "" }
                            }
                        }
                        HeaderComponent {
                            key: "{tab.tab_id}",
                            group_id: cx.props.group.id,
                            index: index,
                            tab: tab,
                            dragging_tab: cx.props.dragging_tab.clone(),
                            bus: cx.props.bus.clone()
                        }
                    }
                }
                if let Some(DropTabOffer::InGroup {
                    group_id: offer_group_id,
                    index: offer_index
                }) = cx.props.tab_drop_offer {
                    if offer_group_id == cx.props.group.id && offer_index >= cx.props.group.tabs.len() {
                        rsx! {
                            div {
                                class: "tab-drop-target"
                            }
                        }
                    } else {
                        rsx! { "" }
                    }
                }
            }
            div {
                class: "tab-content",
                if cx.props.dragging_tab.is_some() {
                    rsx! {
                        div {
                            class: "body-drop-overlay",
                            onmounted: move |evt| {
                                on_body_drop_resize.mount(evt);
                            },
                            onmouseout: move |evt| {
                                log::debug!("cancel 1");
                                cx.props.bus.send_blocking(Command::cancel_offer_drop_tab());
                            },
                            onmousemove: move |evt| {
                                let (x, y) = evt.element_coordinates().to_tuple();
                                let body_drop_size = body_drop_size.read();
                                let width = body_drop_size.width;
                                let height = body_drop_size.height;

                                let h_third = match x / width {
                                    t if t < 0.3333 => 0,
                                    t if t > 0.6666 => 2,
                                    _ => 1
                                };

                                let v_third = match y / height {
                                    t if t < 0.3333 => 0,
                                    t if t > 0.6666 => 2,
                                    _ => 1
                                };

                                let left_dist = x.abs();
                                let right_dist = (width - x).abs();
                                let top_dist = y.abs();
                                let bottom_dist = (height - y).abs();

                                let offer = match (h_third, v_third) {
                                    // Top left, target is top or left depending on closest edge
                                    (0, 0) => match left_dist < top_dist {
                                        true => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Left },
                                        false => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Up },
                                    }
                                    // Top right, target is top or right depending on closest edge
                                    (2, 0) => match right_dist < top_dist {
                                        true => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Right },
                                        false => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Up },
                                    }
                                    // Bottom left, target is bottom or left depending on closest edge
                                    (0, 2) => match left_dist < bottom_dist {
                                        true => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Left },
                                        false => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Down },
                                    }
                                    // Bottom right, target is bottom or right depending on closest edge
                                    (2, 2) => match right_dist < bottom_dist {
                                        true => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Right },
                                        false => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Down },
                                    }
                                    // Left center, target is left
                                    (0, 1) => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Left },
                                    // Right center, target is right
                                    (2, 1) => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Right },
                                    // Top center, target is top
                                    (1, 0) => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Up },
                                    // Bottom center, target is bottom
                                    (1, 2) => DropTabOffer::Split { group_id: cx.props.group.id, direction: SplitDirection::Down },
                                    // Center, target is in group
                                    _ => DropTabOffer::InGroup {
                                        group_id: cx.props.group.id,
                                        index: cx.props.group.tabs.len()
                                    }
                                };

                                cx.props.bus.send_blocking(Command::offer_drop_tab(offer));
                            },
                            if let Some(body_drop_target_class) = body_drop_target_class {
                                rsx! {
                                    div {
                                        class: "body-drop-target {body_drop_target_class}"
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(tab) = cx.props.group.tabs.iter().find(|tab| tab.active_in_group) {
                    rsx! {
                        div {
                            class: "active-content",
                            "ACTIVE: {tab.title}"
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "no-active-tab",
                            p {
                                "Click a tab to open it"
                            }
                        }
                    }
                }
            }
        }
    })
}
