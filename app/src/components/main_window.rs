use dioxus::prelude::*;

use crate::components::MenuBar;

use super::{menu, MenuBarProps, MenuItem, MenuItemKind};

pub fn main_window(cx: Scope) -> Element {
    let name = use_state(cx, || "blah".to_string());

    let menu_items = vec![
        MenuItemKind::Item(MenuItem {
            name: "File".into(),
            enabled: true,
            children: vec![],
        }),
        MenuItemKind::Item(MenuItem {
            name: "Edit".into(),
            enabled: true,
            children: vec![],
        }),
        MenuItemKind::Item(MenuItem {
            name: "Help".into(),
            enabled: true,
            children: vec![],
        }),
    ];

    let menu_items = [
        menu::item(
            "File",
            true,
            [
                menu::item("New part", true, []),
                menu::item("New assembly", true, []),
                menu::sep(),
                menu::item("Save", false, []),
                menu::item("Save as...", false, []),
                menu::sep(),
                menu::item("Exit", true, []),
            ],
        ),
        menu::sep(),
        menu::item("Edit", true, []),
        menu::item(
            "Help",
            true,
            [
                menu::item("About", true, []),
                menu::item(
                    "Open source licenses",
                    true,
                    [
                        menu::item("Package 1", true, []),
                        menu::item("Package 2", true, []),
                        menu::item("Package 3", true, []),
                    ],
                ),
            ],
        ),
    ]
    .to_vec();

    cx.render(rsx! {
        MenuBar {
            items: menu_items
        }
        input {
            value: "{name}",
            oninput: move |evt| { name.set(evt.value.clone()) }
        }
        div {
            "FOO"
            "{name}"
        }
    })
}
