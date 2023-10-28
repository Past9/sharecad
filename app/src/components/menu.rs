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
                    MenuItemKind::Separator => rsx! { div { class: "menu-sep" } },
                    MenuItemKind::Item(item) => rsx! { menu_item { key: "{i}", item: item, top_level: true } },
                }
            }
        }
    })
}

#[inline_props]
fn menu_item<'a>(cx: Scope, item: &'a MenuItem, top_level: bool) -> Element<'a> {
    cx.render(rsx! {
        a {
            class: "menu-item",
            div {
                class: "menu-item-name",
                "{item.name}"
            }
            if !top_level && item.children.len() > 0 {
                rsx! { 
                    div {
                        class: "menu-item-expandable",
                        "▸"
                    }
                }
            }
            if item.children.len() > 0 {
                rsx!{
                    div {
                        class: "submenu",
                        for (i, item) in item.children.iter().enumerate() {
                            match item {
                                MenuItemKind::Separator => rsx! { div { class: "menu-sep" } },
                                MenuItemKind::Item(item) => rsx! { menu_item { key: "{i}", item: item, top_level: false } },
                            }
                        }
                    }
                }
            }
        }
    })
}
