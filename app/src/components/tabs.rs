use crate::{
    on_resize::{ComponentSize, OnResize},
    window_events::{use_window_mousemove, use_window_mouseup},
};
use dioxus::{html::input_data::MouseButton, prelude::*};

//const FLEX_RESOLUTION: f32 = 10000.0;

pub fn tab(id: u32) -> TabProps {
    TabProps { tab_id: id }
}

pub fn group<const N: usize>(tabs: [TabProps; N]) -> TabLayout {
    TabLayout::Group(TabGroup {
        tabs: tabs.to_vec(),
    })
}

pub fn vsplit(split: f64, left: TabLayout, right: TabLayout) -> TabLayout {
    TabLayout::VSplit(TabVSplit {
        split,
        left: Box::new(left),
        right: Box::new(right),
    })
}

pub fn hsplit(split: f64, top: TabLayout, bottom: TabLayout) -> TabLayout {
    TabLayout::HSplit(TabHSplit {
        split,
        top: Box::new(top),
        bottom: Box::new(bottom),
    })
}

#[derive(Clone, PartialEq, Debug)]
pub enum TabLayout {
    Group(TabGroup),
    VSplit(TabVSplit),
    HSplit(TabHSplit),
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabGroup {
    tabs: Vec<TabProps>,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabVSplit {
    split: f64,
    left: Box<TabLayout>,
    right: Box<TabLayout>,
}

#[derive(Clone, Debug, PartialEq, Props)]
pub struct TabHSplit {
    split: f64,
    top: Box<TabLayout>,
    bottom: Box<TabLayout>,
}

#[derive(PartialEq, Debug, Clone, Props)]
pub struct TabProps {
    tab_id: u32,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn TabArea<'a>(
    cx: Scope,
    layout: &'a TabLayout,
    on_layout_changed: EventHandler<'a, TabLayout>,
) -> Element {
    //let layout = *layout.to_owned();

    cx.render(rsx! {
        TabLayoutComponent {
            layout: layout
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabLayoutComponent<'a>(cx: Scoped, layout: &'a TabLayout) -> Element {
    match layout {
        TabLayout::Group(group) => {
            //
            cx.render(rsx! {
                TabGroupComponent { group: group.clone() }
            })
        }
        TabLayout::VSplit(vsplit) => cx.render(rsx! {
            TabVSplitComponent { vsplit: vsplit.clone() }
        }),
        TabLayout::HSplit(hsplit) => cx.render(rsx! {
            TabHSplitComponent { hsplit: hsplit.clone() }
        }),
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn TabComponent(cx: Scoped, tab: TabProps) -> Element {
    cx.render(rsx! {
        div {
            "Tab "
            "{tab.tab_id}"
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabGroupComponent(cx: Scoped, group: TabGroup) -> Element<'a> {
    cx.render(rsx! {
        for tab in group.tabs.iter() {
            rsx! {
                TabComponent {
                    tab: tab.clone()
                }
            }
        }
    })
}

#[derive(Clone)]
struct DragPosition {
    start: f64,
    current: f64,
}
impl DragPosition {
    pub fn dist(&self) -> f64 {
        self.current - self.start
    }

    pub fn with_current(self, current: f64) -> Self {
        Self {
            start: self.start,
            current,
        }
    }

    pub fn split_dist(&self, size: f64) -> f64 {
        self.dist() / size
    }

    pub fn adjust_split(&self, size: f64, current_split: f64) -> f64 {
        (current_split + self.split_dist(size)).clamp(0.0, 1.0)
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn TabVSplitComponent(cx: Scoped, vsplit: TabVSplit) -> Element<'a> {
    let size = use_ref(cx, ComponentSize::default);
    let drag_dist = use_state(cx, || -> Option<DragPosition> { None });

    let on_resize = use_state(cx, || {
        to_owned![size];
        OnResize::new(move |new_size: ComponentSize| size.set(new_size))
    });

    {
        to_owned![drag_dist];
        use_window_mouseup(cx, move |evt| {
            drag_dist.set(None);
            if evt.trigger_button() == Some(MouseButton::Primary) {
                drag_dist.set(None);
            }
            log::debug!("stop drag");
        });
    }

    {
        to_owned![drag_dist, size, vsplit];
        use_window_mousemove(cx, move |evt| {
            if evt.held_buttons().contains(MouseButton::Primary) {
                if let Some(ref dist) = *drag_dist.current() {
                    //log::debug!("drag {:#?}", evt);
                    drag_dist.set(Some(dist.clone().with_current(evt.client_coordinates().x)));
                    log::debug!(
                        "new split {}",
                        dist.adjust_split(size.read().width, vsplit.split)
                    );
                }
            } else {
                drag_dist.set(None);
            }
        });
    }

    let element = cx.render(rsx! {
        div {
            class: "vsplit",
            onmounted: move |evt| {
                //log::debug!("onmounted {:#?}", evt.data.get_client_rect().await);
                on_resize.mount(evt);
            },
            div {
                class: "vsplit-pane vsplit-left",
                flex: vsplit.split,
                span {
                    "width: {size.read().width}, height: {size.read().height}"
                }
                TabLayoutComponent {
                    layout: vsplit.left.as_ref()
                }
            }
            div {
                class: "splitter",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        let pos = evt.client_coordinates().x;
                        drag_dist.set(Some(DragPosition {
                            start: pos,
                            current: pos
                        }));
                        //is_dragging.set(true);
                        log::debug!("start drag");
                    }
                },
            }
            div {
                class: "vsplit-pane vsplit-right",
                flex: 1.0 - vsplit.split,
                TabLayoutComponent {
                    layout: vsplit.right.as_ref()
                }
            }
        }
    });

    element
}

#[allow(non_snake_case)]
#[inline_props]
fn TabHSplitComponent(cx: Scoped, hsplit: TabHSplit) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "hsplit",
            div {
                class: "hsplit-pane hsplit-top",
                flex: hsplit.split,
                TabLayoutComponent {
                    layout: hsplit.top.as_ref()
                }
            }
            div {
                class: "splitter"
            }
            div {
                class: "hsplit-pane hsplit-bottom",
                flex: 1.0 - hsplit.split,
                TabLayoutComponent {
                    layout: hsplit.bottom.as_ref()
                }
            }
        }
    })
}
