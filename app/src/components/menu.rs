use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct MenuBarProps {
    pub items: Vec<MenuItemKind>,
}

#[derive(PartialEq, Clone)]
pub enum MenuItemKind {
    Separator,
    Item(MenuItem),
}

#[derive(PartialEq, Props, Clone)]
pub struct MenuItem {
    pub name: String,
    pub enabled: bool,
    pub children: Vec<MenuItemKind>,
}

pub fn item<const N: usize>(
    name: &str,
    enabled: bool,
    children: [MenuItemKind; N],
) -> MenuItemKind {
    MenuItemKind::Item(MenuItem {
        name: name.to_string(),
        enabled,
        children: children.to_vec(),
    })
}

pub fn sep() -> MenuItemKind {
    MenuItemKind::Separator
}

#[allow(non_snake_case)]
pub fn MenuBar(cx: Scope<MenuBarProps>) -> Element {
    cx.render(rsx! {
        nav {
            class: "menu-bar",
            for (i, item) in cx.props.items.iter().enumerate() {
                match item {
                    MenuItemKind::Separator => rsx! { div { "seps" } },
                    MenuItemKind::Item(item) => rsx! { top_level_menu_item { key: "{i}", item: item } },
                }
            }
        }
    })
}

#[inline_props]
fn top_level_menu_item<'a>(cx: Scope, item: &'a MenuItem) -> Element<'a> {
    cx.render(rsx! {
        a {
            "{item.name}"
        }
        /*
        for (i, item) in item.children.iter().enumerate() {
            match item {
                MenuItemKind::Separator => rsx! { div { "> sep" } },
                MenuItemKind::Item(item) => rsx! { child_item { key: "{i}", item: item } },
            }
        }
         */
    })
}

#[inline_props]
fn child_item<'a>(cx: Scope, item: &'a MenuItem) -> Element<'a> {
    cx.render(rsx! {
        a {
            "> {item.name}"
        }
        for (i, item) in item.children.iter().enumerate() {
            match item {
                MenuItemKind::Separator => rsx! { div { "> sep" } },
                MenuItemKind::Item(item) => rsx! { child_item { key: "{i}", item: item } },
            }
        }
    })
}

/*
pub fn top_level_menu_item<'a>(cx: Scope<'a, &'a MenuItem>) -> Element {
    cx.render(rsx! {
        div {
            "{cx.props.name}"
        }
    })
}
 */

/*
pub fn top_menu_item(cx: Scope<MenuItem>) -> Element {
    cx.render(rsx! {
        div {
            "top_menu_item"
        }
    })
}

pub fn menu_child(cx: Scope<MenuItem>) -> Element {
    cx.render(rsx! {
        div {
            "menu_child"
        }
    })
}

 */
