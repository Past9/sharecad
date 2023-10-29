use std::time::{Duration, Instant};

use dioxus::{
    html::input_data::{MouseButton, MouseButtonSet},
    prelude::*,
};
use futures::StreamExt;
use gloo::events::EventListener;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{use_window_event, WindowEvents};

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
    layout: TabLayout,
    on_layout_changed: EventHandler<'a, TabLayout>,
) -> Element {
    let layout = layout.clone();

    cx.render(rsx! {
        TabLayoutComponent {
            layout: layout
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabLayoutComponent(cx: Scoped, layout: TabLayout) -> Element {
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

#[allow(non_snake_case)]
#[inline_props]
fn TabVSplitComponent(cx: Scoped, vsplit: TabVSplit) -> Element<'a> {
    let is_dragging = use_state(cx, || false);

    {
        to_owned![is_dragging];
        use_window_event(cx, "mouseup", move |evt| {
            is_dragging.set(false);
            log::debug!("got mousemove {:?}", evt)
        });
    }

    use_window_event(cx, "mousemove", move |evt| {
        log::debug!("got mousemove {:?}", evt)
    });

    cx.render(rsx! {
        div {
            class: "vsplit",
            div {
                class: "vsplit-pane vsplit-left",
                flex: vsplit.split,
                TabLayoutComponent {
                    layout: *vsplit.left.clone()
                }
            }
            div {
                class: "splitter",
                onmousedown: move |evt| {
                    if let Some(MouseButton::Primary) = evt.trigger_button() {
                        log::debug!("onmousedown {:?}", evt);
                        is_dragging.set(true);
                    }
                },
                onclick: move |_| {
                    is_dragging.set(!is_dragging);
                }
            }
            div {
                class: "vsplit-pane vsplit-right",
                flex: 1.0 - vsplit.split,
                TabLayoutComponent {
                    layout: *vsplit.right.clone()
                }
            }
        }
    })
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
                    layout: *hsplit.top.clone()
                }
            }
            div {
                class: "splitter"
            }
            div {
                class: "hsplit-pane hsplit-bottom",
                flex: 1.0 - hsplit.split,
                TabLayoutComponent {
                    layout: *hsplit.bottom.clone()
                }
            }
        }
    })
}
