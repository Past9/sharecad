use super::CommandBus;
use crate::{
    components::{Command, DraggingTab, GroupId, Tab},
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use dioxus::{html::input_data::MouseButton, prelude::*};

#[derive(Debug)]
pub enum DragState {
    Standby {
        element_offset: (f64, f64),
        client_start_pos: (f64, f64),
    },
    Dragging {
        element_offset: (f64, f64),
        client_start_pos: (f64, f64),
        client_current_pos: (f64, f64),
    },
}

#[derive(PartialEq, Props)]
pub struct HeaderComponentProps<'a> {
    group_id: GroupId,
    index: usize,
    tab: &'a Tab,
    #[props(!optional)]
    dragging_tab: Option<DraggingTab>,
    bus: CommandBus,
}

#[allow(non_snake_case)]
pub fn HeaderComponent<'a>(cx: Scope<'a, HeaderComponentProps<'a>>) -> Element<'a> {
    const DRAG_TRIGGER_DIST: f64 = 5.0;

    let size = use_ref(cx, ComponentSize::default);
    let drag_state = use_state(cx, || -> Option<DragState> { None });

    let on_resize = use_state(cx, || {
        to_owned![size];
        OnResize::new(move |new_size: ComponentSize| size.set(new_size))
    });

    {
        to_owned![on_resize];
        use_on_unmount(cx, move || {
            on_resize.unmount();
        });
    }

    use_window_mouseup(cx, (drag_state, &cx.props.bus), |(drag_state, bus)| {
        move |_| {
            drag_state.set(None);
            bus.send_blocking(Command::drop_tab());
        }
    });

    use_window_mousemove(
        cx,
        (
            drag_state,
            &cx.props.group_id,
            &cx.props.index,
            cx.props.tab,
            &cx.props.bus,
        ),
        |(drag_state, group_id, index, tab, bus)| {
            move |evt| {
                if let Some(ref state) = *drag_state.current() {
                    if evt.held_buttons().contains(MouseButton::Primary) {
                        let client_current_pos = evt.client_coordinates().to_tuple();
                        match state {
                            DragState::Standby {
                                element_offset,
                                client_start_pos,
                            } => {
                                // Distance mouse has traveled from mousedown by Pythagorean theorem
                                let dist = ((client_current_pos.0 - client_start_pos.0).powi(2)
                                    + (client_current_pos.1 - client_start_pos.1).powi(2))
                                .sqrt();

                                if dist > DRAG_TRIGGER_DIST {
                                    drag_state.set(Some(DragState::Dragging {
                                        element_offset: *element_offset,
                                        client_start_pos: client_current_pos,
                                        client_current_pos: client_current_pos,
                                    }));
                                    bus.send_blocking(Command::drag_tab(group_id, index, tab.id));
                                }
                            }
                            DragState::Dragging {
                                element_offset,
                                client_start_pos,
                                ..
                            } => {
                                drag_state.set(Some(DragState::Dragging {
                                    element_offset: *element_offset,
                                    client_start_pos: *client_start_pos,
                                    client_current_pos,
                                }));
                            }
                        }
                    } else {
                        drag_state.set(None);
                        bus.send_blocking(Command::drop_tab());
                    }
                }
            }
        },
    );

    let (is_dragging, pos) = match *drag_state.current() {
        Some(ref drag_state) => match drag_state {
            DragState::Dragging {
                element_offset,
                client_current_pos,
                ..
            } => (
                true,
                Some((
                    client_current_pos.0 - element_offset.0,
                    client_current_pos.1 - element_offset.1,
                )),
            ),
            _ => (false, None),
        },
        None => (false, None),
    };

    cx.render(rsx! {
        HeaderComponentInner {
            title: cx.props.tab.title.clone(),
            active_in_group: cx.props.tab.active_in_group,
            absolute_pos: None,
            opacity: 1.0,
            onmousedown: move |evt: Event<MouseData>| {
                let client_coords = evt.client_coordinates().to_tuple();
                let element_coords = evt.element_coordinates().to_tuple();
                drag_state.set(Some(DragState::Standby {
                    element_offset: element_coords,
                    client_start_pos: client_coords
                }));
                cx.props.bus.send_blocking(Command::set_active_tab_in_group(
                    cx.props.group_id,
                    cx.props.tab.id
                ));
            },
            onmounted: move |evt| {
                on_resize.mount(evt);
            },
            lr_mouse_targets: cx.props.dragging_tab.is_some(),
            onmouseover_drop_left: |_| {
                cx.props.bus.send_blocking(Command::offer_drop_tab_in_group(cx.props.group_id, cx.props.index));
            },
            onmouseover_drop_right: |_| {
                let index = match cx.props.dragging_tab {
                    Some(ref dragging_tab) => if dragging_tab.tab_id == cx.props.tab.id {
                        cx.props.index
                    } else {
                        cx.props.index + 1
                    }
                    None => cx.props.index + 1,
                };

                cx.props.bus.send_blocking(Command::offer_drop_tab_in_group(cx.props.group_id, index));
            },
            onmouseout_drop_left: |_| {
                cx.props.bus.send_blocking(Command::cancel_offer_drop_tab());
            },
            onmouseout_drop_right: |_| {
                cx.props.bus.send_blocking(Command::cancel_offer_drop_tab());
            },
            on_request_close: move |_| {
                cx.props.bus.send_blocking(Command::close_tab(cx.props.tab.id));
            },
            if is_dragging {
                rsx! {
                    div {
                        class: "tab-drag-overlay",
                        HeaderComponentInner {
                            title: cx.props.tab.title.clone(),
                            active_in_group: true,
                            absolute_pos: pos,
                            opacity: 0.8,
                            onmousedown: |_| {},
                            onmounted: |_| {},
                            lr_mouse_targets: false,
                            onmouseover_drop_left: |_| {},
                            onmouseover_drop_right: |_| {},
                            onmouseout_drop_left: |_| {},
                            onmouseout_drop_right: |_| {},
                            on_request_close: |_| {}
                        }
                    }
                }
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn HeaderComponentInner<'a>(
    cx: Scoped,
    title: String,
    active_in_group: bool,
    #[props(!optional)] absolute_pos: Option<(f64, f64)>,
    opacity: f64,
    onmousedown: EventHandler<'a, Event<MouseData>>,
    onmounted: EventHandler<'a, Event<MountedData>>,
    lr_mouse_targets: bool,
    onmouseover_drop_left: EventHandler<'a, Event<MouseData>>,
    onmouseover_drop_right: EventHandler<'a, Event<MouseData>>,
    onmouseout_drop_left: EventHandler<'a, Event<MouseData>>,
    onmouseout_drop_right: EventHandler<'a, Event<MouseData>>,
    on_request_close: EventHandler<'a, ()>,
    children: Element<'a>,
) -> Element {
    let active_in_group_class = match active_in_group {
        true => "active-in-group",
        false => "",
    };

    let (position_attr, left_attr, top_attr) = match absolute_pos {
        Some(pos) => ("absolute", pos.0, pos.1),
        None => ("static", 0.0, 0.0),
    };

    let div = cx.render(rsx! {
        div {
            class: "tab-header",
            onmousedown: |evt| { onmousedown.call(evt) },
            onmounted: |evt| { onmounted.call(evt) },
            position: position_attr,
            left: "{left_attr}px",
            top: "{top_attr}px",
            opacity: "{opacity}",

            div {
                class: "inner",

                if *lr_mouse_targets {
                    rsx! {
                        div {
                            class: "mouse-target left",
                            onmouseover: move |evt| {
                                onmouseover_drop_left.call(evt);
                            },
                            onmouseout: move |evt| {
                                onmouseout_drop_left.call(evt);
                            },
                        }
                        div {
                            class: "mouse-target right",
                            onmouseover: move |evt| {
                                onmouseover_drop_right.call(evt);
                            },
                            onmouseout: move |evt| {
                                onmouseout_drop_right.call(evt);
                            },
                        }
                    }
                }

                div {
                    class: "padding {active_in_group_class}",
                    div {
                        class: "tab-icon",
                        "Ϣ"
                    }
                    div {
                        class: "tab-title",
                        "{title}"
                    }
                    div {
                        class: "tab-close",
                        onclick: move |evt| {
                            on_request_close.call(());
                            evt.stop_propagation();
                        },
                        onmousedown: move |evt| {
                            evt.stop_propagation();
                        },
                        "✕"
                    }
                    children
                }
            }
        }
    });

    div
}
