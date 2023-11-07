use dioxus::prelude::*;

use crate::command::CommandBus;

use super::GlobalCommand;

#[derive(PartialEq, Props)]
pub struct MenuBarProps {
    pub items: Vec<MenuItemKind>,
    pub bus: CommandBus<GlobalCommand>,
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
    pub action: Option<GlobalCommand>,
}

pub fn item<const N: usize>(
    name: &str,
    enabled: bool,
    children: [MenuItemKind; N],
    action: Option<GlobalCommand>,
) -> MenuItemKind {
    MenuItemKind::Item(MenuItem {
        name: name.to_string(),
        enabled,
        children: children.to_vec(),
        action,
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
                    MenuItemKind::Separator => rsx! {
                        div {
                            class: "menu-sep"
                        }
                    },
                    MenuItemKind::Item(item) => rsx! {
                        MenuItem {
                            key: "{i}",
                            item: item,
                            top_level: true,
                            bus: cx.props.bus.clone()
                        }
                    },
                }
            }
        }
    })
}

#[derive(Props)]
struct MenuItemProps<'a> {
    item: &'a MenuItem,
    top_level: bool,
    bus: CommandBus<GlobalCommand>,
}

#[allow(non_snake_case)]
fn MenuItem<'a>(cx: Scope<'a, MenuItemProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        a {
            class: "menu-item",
            onclick: move |_| {
                if let Some(ref command) = cx.props.item.action {
                    cx.props.bus.send_blocking(command.clone());
                }
            },
            div {
                class: "menu-item-name",
                "{cx.props.item.name}"
            }
            if !cx.props.top_level && cx.props.item.children.len() > 0 {
                rsx! {
                    div {
                        class: "menu-item-expandable",
                        "▸"
                    }
                }
            }
            if cx.props.item.children.len() > 0 {
                rsx!{
                    div {
                        class: "submenu",
                        for (i, item) in cx.props.item.children.iter().enumerate() {
                            match item {
                                MenuItemKind::Separator => rsx! {
                                    div {
                                        class: "menu-sep"
                                    }
                                },
                                MenuItemKind::Item(item) => rsx! {
                                    MenuItem {
                                        key: "{i}",
                                        item: item,
                                        top_level: false,
                                        bus: cx.props.bus.clone()
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    })
}
