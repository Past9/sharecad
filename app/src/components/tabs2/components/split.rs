use super::CommandBus;
use crate::{
    components::{
        Command, DraggingTab, DropTabOffer, GenericSplit, LayoutComponent, SplitOrientation,
    },
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use dioxus::{html::input_data::MouseButton, prelude::*};

#[derive(Clone, Debug)]
struct DragState {
    splitter_index: usize,
    start_split: f64,
    start_mousepos: f64,
    current_mousepos: f64,
}
impl DragState {
    pub fn dist(&self) -> f64 {
        self.current_mousepos - self.start_mousepos
    }

    pub fn with_current(self, current: f64) -> Self {
        Self {
            splitter_index: self.splitter_index,
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
pub struct SplitComponentProps<'a> {
    split: &'a GenericSplit,
    #[props(!optional)]
    tab_drop_offer: Option<DropTabOffer>,
    #[props(!optional)]
    dragging_tab: Option<DraggingTab>,
    bus: CommandBus,
}

#[allow(non_snake_case)]
pub fn SplitComponent<'a>(cx: Scope<'a, SplitComponentProps>) -> Element<'a> {
    let size = use_ref(cx, ComponentSize::default);
    let drag = use_state(cx, || -> Option<DragState> { None });

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

    use_window_mouseup(cx, drag, |drag_pos| {
        move |_| {
            drag_pos.set(None);
        }
    });

    use_window_mousemove(
        cx,
        (drag, size, cx.props.split, &cx.props.bus),
        |(drag, size, split, bus)| {
            move |evt| {
                if let Some(ref pos) = *drag.current() {
                    if evt.held_buttons().contains(MouseButton::Primary) {
                        let (position, space) = match split.orientation {
                            SplitOrientation::Vertical => {
                                (evt.client_coordinates().x, size.read().width)
                            }
                            SplitOrientation::Horizontal => {
                                (evt.client_coordinates().y, size.read().height)
                            }
                        };

                        let new_drag_pos = pos.clone().with_current(position);
                        let new_location = new_drag_pos.adjust_split(space);
                        drag.set(Some(new_drag_pos));

                        let command = match split.orientation {
                            SplitOrientation::Vertical => Command::adjust_vsplit(
                                split.id.as_vsplit_id(),
                                pos.splitter_index,
                                new_location,
                            ),
                            SplitOrientation::Horizontal => Command::adjust_hsplit(
                                split.id.as_hsplit_id(),
                                pos.splitter_index,
                                new_location,
                            ),
                        };

                        bus.send_blocking(command);
                    } else {
                        drag.set(None);
                    }
                }
            }
        },
    );

    let direction_class = match cx.props.split.orientation {
        SplitOrientation::Vertical => "vertical",
        SplitOrientation::Horizontal => "horizontal",
    };

    let total_widths = use_memo(cx, &cx.props.split.children, |children| {
        let mut total_width = 0f64;
        let mut total_widths = vec![];
        for child in children.iter() {
            total_width += child.width;
            total_widths.push(total_width);
        }
        total_widths
    });

    cx.render(rsx! {
        if drag.is_some() {
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

            for (i, child) in cx.props.split.children.iter().enumerate() {
                rsx! {
                    div {
                        class: "split-pane",
                        flex: child.width,
                        LayoutComponent {
                            layout: (&child.child).into(),
                            tab_drop_offer: cx.props.tab_drop_offer.clone(),
                            dragging_tab: cx.props.dragging_tab.clone(),
                            bus: cx.props.bus.clone()
                        }
                    }
                }

                if i < cx.props.split.children.len() - 1 {
                    let dragging_class = match drag.current().as_ref() {
                        Some(drag) => match drag.splitter_index == i {
                            true => "dragging",
                            false => ""
                        },
                        None => "",
                    };

                    rsx! {
                        div {
                            class: "splitter {dragging_class}",
                            onmousedown: move |evt| {
                                if let Some(MouseButton::Primary) = evt.trigger_button() {
                                    let pos = match cx.props.split.orientation {
                                        SplitOrientation::Vertical => evt.client_coordinates().x,
                                        SplitOrientation::Horizontal => evt.client_coordinates().y,
                                    };
                                    drag.set(Some(DragState {
                                        splitter_index: i,
                                        start_split: total_widths[i],
                                        start_mousepos: pos,
                                        current_mousepos: pos
                                    }));
                                }
                            },
                        }
                    }
                }
            }
        }
    })
}
