use dioxus::{
    html::input_data::{MouseButton, MouseButtonSet},
    prelude::*,
};
use gloo::events::EventListener;
use wasm_bindgen::{prelude::Closure, JsCast};

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

    log::debug!("is_dragging {}", is_dragging);

    use_effect(cx, is_dragging, |is_dragging| async move {
        let win = web_sys::window().unwrap();

        let mousemove_listener = EventListener::new(&win, "mousemove", move |_| {
            log::debug!("move");
        })
        .forget();

        let mouseup_listener = EventListener::new(&win, "mouseup", move |_| {
            log::debug!("up");
            is_dragging.set(false);
        })
        .forget();

        /*
        let on_mouse_up = Closure::wrap(Box::new(move || {
            log::debug!("up");
            is_dragging.set(false);
        }) as Box<dyn FnMut()>);

        let on_mouse_move = Closure::wrap(Box::new(move || {
            log::debug!("move");
        }) as Box<dyn FnMut()>);

        log::debug!("add handlers");

        {
            win.add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())
                .unwrap();

            win.add_event_listener_with_callback(
                "mousemove",
                on_mouse_move.as_ref().unchecked_ref(),
            )
            .unwrap();
        }

        on_mouse_up.forget();
        on_mouse_move.forget();
         */

        move || {
            log::debug!("remove handlers");

            //std::mem::drop(mousemove_listener);
            //std::mem::drop(mouseup_listener);

            /*
            win.remove_event_listener_with_callback(
                "mouseup",
                on_mouse_up.as_ref().unchecked_ref(),
            )
            .unwrap();

            win.remove_event_listener_with_callback(
                "mousemove",
                on_mouse_move.as_ref().unchecked_ref(),
            )
            .unwrap();
             */
        }
    });

    /*
    if **is_dragging {
        log::debug!("dragging");
    }
     */

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
                    //if evt.held_buttons().contains(MouseButton::Primary) {
                        log::debug!("onmousedown {:?}", evt);
                        is_dragging.set(true);
                    }
                },
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
