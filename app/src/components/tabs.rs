use dioxus::prelude::*;

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

#[derive(Clone, PartialEq)]
pub enum TabLayout {
    Group(TabGroup),
    VSplit(TabVSplit),
    HSplit(TabHSplit),
}

#[derive(Clone, PartialEq, Props)]
pub struct TabGroup {
    tabs: Vec<TabProps>,
}

#[derive(Clone, PartialEq, Props)]
pub struct TabVSplit {
    split: f64,
    left: Box<TabLayout>,
    right: Box<TabLayout>,
}

#[derive(Clone, PartialEq, Props)]
pub struct TabHSplit {
    split: f64,
    top: Box<TabLayout>,
    bottom: Box<TabLayout>,
}

#[derive(PartialEq, Clone, Props)]
pub struct TabProps {
    tab_id: u32,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn TabArea(cx: Scope, layout: TabLayout) -> Element {
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
                class: "splitter"
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
