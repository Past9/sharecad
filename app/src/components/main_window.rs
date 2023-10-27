use dioxus::prelude::*;

use crate::components::{MenuBar, Tabs};

use super::{menu, MenuItem, MenuItemKind};

pub fn main_window(cx: Scope) -> Element {
    let menu_items = [
        menu::item(
            "File",
            true,
            [
                menu::item("New part", true, []),
                menu::item("New assembly", true, []),
                menu::sep(),
                menu::item("Open file", true, []),
                menu::item(
                    "Open some recent file",
                    true,
                    [
                        menu::item("some/file/part1.prt", true, []),
                        menu::item("some/file/part2.prt", true, []),
                        menu::item("some/file/assembly1.asm", true, []),
                        menu::item("some/file/assembly2.asm", true, []),
                    ],
                ),
                menu::sep(),
                menu::item("Save", false, []),
                menu::item("Save as...", false, []),
                menu::sep(),
                menu::item(
                    "Settings",
                    true,
                    [
                        menu::item("Option 1", false, []),
                        menu::item("Option 2", false, []),
                    ],
                ),
                menu::item("Exit", true, []),
            ],
        ),
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
                        menu::item(
                            "Package 2",
                            true,
                            [
                                menu::item("Version 1", true, []),
                                menu::item("Version 2", true, []),
                            ],
                        ),
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

        Tabs {

        }

    })
}
