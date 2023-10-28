use dioxus::prelude::*;

pub fn tab(id: u32) -> TabProps {
    TabProps { tab_id: id }
}

pub fn group<const N: usize>(tabs: [TabProps; N]) -> TabLayout {
    TabLayout::Group(tabs.to_vec())
}

pub fn vsplit(split: f32, left: TabLayout, right: TabLayout) -> TabLayout {
    TabLayout::VSplit {
        split,
        left: Box::new(left),
        right: Box::new(right),
    }
}

pub fn hsplit(split: f32, top: TabLayout, bottom: TabLayout) -> TabLayout {
    TabLayout::HSplit {
        split,
        top: Box::new(top),
        bottom: Box::new(bottom),
    }
}

#[derive(PartialEq)]
pub enum TabLayout {
    Group(Vec<TabProps>),
    VSplit {
        split: f32,
        left: Box<TabLayout>,
        right: Box<TabLayout>,
    },
    HSplit {
        split: f32,
        top: Box<TabLayout>,
        bottom: Box<TabLayout>,
    },
}

#[derive(PartialEq, Clone, Props)]
pub struct TabProps {
    tab_id: u32,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn TabArea<FGetInitialLayout: FnOnce() -> TabLayout>(
    cx: Scope,
    get_initial_layout: FGetInitialLayout,
) -> Element {
    cx.render(rsx! {
        "TabArea"
    })
}

#[allow(non_snake_case)]
#[inline_props]
fn TabLayout(cx: Scope, layout: TabLayout) -> Element {
    cx.render(rsx! {
        "TabLayout"
    })
}
