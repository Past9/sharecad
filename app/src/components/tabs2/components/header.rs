use super::CommandBus;
use crate::{
    components::{Command, GroupId, Tab, TabId},
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use dioxus::{html::input_data::MouseButton, prelude::*};
use web_sys::DragEventInit;

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
    dragged_tab: Option<TabId>,
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
        (drag_state, cx.props.tab, &cx.props.bus),
        |(drag_state, tab, bus)| {
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
                                    bus.send_blocking(Command::drag_tab(tab.id));
                                }
                            }
                            DragState::Dragging {
                                element_offset,
                                client_start_pos,
                                ..
                            } => {
                                {
                                    let win = web_sys::window().unwrap();
                                    let doc = win.document().unwrap();
                                    let elements = doc.elements_from_point(
                                        client_current_pos.0 as f32,
                                        client_current_pos.1 as f32,
                                    );

                                    let mut found_overlay = false;
                                    let mut element_under_cursor = None;
                                    for el in elements.iter() {
                                        let el = match web_sys::Element::try_from(el) {
                                            Ok(el) => el,
                                            Err(_) => {
                                                continue;
                                            }
                                        };

                                        let classes = el
                                            .class_name()
                                            .split_whitespace()
                                            .into_iter()
                                            .map(|c| c.to_string())
                                            .collect::<Vec<_>>();

                                        if !found_overlay {
                                            if classes.iter().any(|c| c == "tab-drag-overlay") {
                                                found_overlay = true;
                                            }
                                            continue;
                                        }

                                        if classes.iter().any(|c| c == "tab-header") {
                                            element_under_cursor = Some(el);
                                        }

                                        break;
                                    }

                                    if let Some(element_under_cursor) = element_under_cursor {
                                        let event = web_sys::DragEvent::new_with_event_init_dict(
                                            "dragover",
                                            DragEventInit::new()
                                                .bubbles(true)
                                                .cancelable(true)
                                                .view(Some(&win))
                                                .client_x(evt.client_coordinates().x as i32)
                                                .client_y(evt.client_coordinates().y as i32)
                                                .screen_x(evt.screen_coordinates().x as i32)
                                                .screen_y(evt.screen_coordinates().y as i32)
                                                .buttons(1),
                                        )
                                        .unwrap();

                                        element_under_cursor.dispatch_event(&event).unwrap();
                                    }
                                }

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
            ondragover: move |evt: Event<DragData>| {
                if let Some(dragged_tab) = cx.props.dragged_tab {
                    if dragged_tab != cx.props.tab.id {
                        let drop_index = if evt.mouse.element_coordinates().x < size.read().width / 2.0 {
                            cx.props.index
                        } else {
                            cx.props.index + 1
                        };

                        cx.props.bus.send_blocking(Command::offer_drop_tab_in_group(cx.props.group_id, drop_index));
                    }
                }
            },
            onmounted: move |evt| {
                on_resize.mount(evt);
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
                            ondragover: |_| {},
                            onmounted: |_| {},
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
    ondragover: EventHandler<'a, Event<DragData>>,
    onmounted: EventHandler<'a, Event<MountedData>>,
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
            class: "tab-header {active_in_group_class}",
            onmousedown: |evt| { onmousedown.call(evt) },
            ondragover: |evt| { ondragover.call(evt) },
            onmounted: |evt| { onmounted.call(evt) },
            position: position_attr,
            left: "{left_attr}px",
            top: "{top_attr}px",
            opacity: "{opacity}",
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
    });

    div
}
