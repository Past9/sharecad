use dioxus::{html::input_data::MouseButton, prelude::*};

use crate::{
    components::{Command, DropTabOffer, HSplit, VSplit},
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};

use super::CommandBus;

#[derive(PartialEq, Clone)]
pub enum Split {
    VSplit(VSplit),
    HSplit(HSplit),
}

#[derive(Clone, Debug)]
struct SplitDragPosition {
    start_split: f64,
    start_mousepos: f64,
    current_mousepos: f64,
}
impl SplitDragPosition {
    pub fn dist(&self) -> f64 {
        self.current_mousepos - self.start_mousepos
    }

    pub fn with_current(self, current: f64) -> Self {
        Self {
            start_split: self.start_split,
            start_mousepos: self.start_mousepos,
            current_mousepos: current,
        }
    }

    pub fn split_dist(&self, size: f64) -> f64 {
        self.dist() / (size - 5.0)
    }

    pub fn adjust_split(&self, size: f64) -> f64 {
        (self.start_split + self.split_dist(size)).clamp(0.0, 1.0)
    }
}

#[derive(PartialEq, Props)]
pub struct SplitComponentProps {
    split: Split,
    #[props(!optional)]
    tab_drop_offer: Option<DropTabOffer>,
    #[props(!optional)]
    dragged_tab: Option<u32>,
    bus: CommandBus,
}

#[allow(non_snake_case)]
fn SplitComponent<'a>(cx: Scope<'a, SplitComponentProps>) -> Element<'a> {
    let size = use_ref(cx, ComponentSize::default);
    let drag_pos = use_state(cx, || -> Option<SplitDragPosition> { None });

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

    use_window_mouseup(cx, drag_pos, |drag_pos| {
        move |_| {
            drag_pos.set(None);
        }
    });

    use_window_mousemove(
        cx,
        (drag_pos, size, &cx.props.split, &cx.props.bus),
        |(drag_pos, size, split, bus)| {
            move |evt| {
                if let Some(ref pos) = *drag_pos.current() {
                    if evt.held_buttons().contains(MouseButton::Primary) {
                        let (position, space) = match split {
                            Split::VSplit(..) => (evt.client_coordinates().x, size.read().width),
                            Split::HSplit(..) => (evt.client_coordinates().y, size.read().height),
                        };

                        let new_drag_pos = pos.clone().with_current(position);
                        let new_location = new_drag_pos.adjust_split(space);
                        drag_pos.set(Some(new_drag_pos));

                        let command = match split {
                            Split::VSplit(VSplit { id, .. }) => {
                                Command::adjust_vsplit(id, 0, new_location)
                            }
                            Split::HSplit(HSplit { id, .. }) => {
                                Command::adjust_hsplit(id, 0, new_location)
                            }
                        };

                        bus.send_blocking(command);
                    } else {
                        drag_pos.set(None);
                    }
                }
            }
        },
    );

    let direction_class = match cx.props.split {
        Split::VSplit(..) => "vertical",
        Split::HSplit(..) => "horizontal",
    };

    let dragging_class = match drag_pos.is_some() {
        true => "dragging",
        false => "",
    };

    cx.render(rsx! {
        /*
        if drag_pos.is_some() {
            rsx! {
                div {
                    class: "split-drag-overlay {direction_class}"
                }
            }
        }
        div {
            class: "split {direction_class}",
            onmounted: move |evt| {
                on_resize.mount(evt);
            },
            div {
                class: "split-pane",
                flex: cx.props.split.location,
                TabLayoutComponent {
                    layout: split.a.as_ref(),
                    tab_drop_offer: tab_drop_offer.clone(),
                    dragged_tab: dragged_tab.clone(),
                    bus: bus.clone()
                }
            }
            div {
                class: "splitter {dragging_class}",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        let pos = match split.direction {
                            TabSplitDirection::Vertical => evt.client_coordinates().x,
                            TabSplitDirection::Horizontal => evt.client_coordinates().y,
                        };
                        drag_pos.set(Some(SplitDragPosition {
                            start_split: split.location,
                            start_mousepos: pos,
                            current_mousepos: pos
                        }));
                    }
                },
            }
            div {
                class: "split-pane",
                flex: 1.0 - split.location,
                TabLayoutComponent {
                    layout: split.b.as_ref(),
                    tab_drop_offer: tab_drop_offer.clone(),
                    dragged_tab: dragged_tab.clone(),
                    bus: bus.clone()
                }
            }
        }
         */
        div {
            "SPLIT"
        }
    })
}
