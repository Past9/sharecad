use super::{menu, tabs};
use crate::components::MenuBar;
use dioxus::prelude::*;
use tabs::TabArea;

#[allow(non_snake_case)]
pub fn MainWindow(cx: Scope) -> Element {
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

    let tab_layout = use_state(cx, || {
        tabs::vsplit(
            1,
            0.327,
            tabs::group(
                1,
                [
                    tabs::tab(1, "Some file", false),
                    tabs::tab(2, "Some file with a very long path.fileextension", true),
                ],
            ),
            tabs::vsplit(
                2,
                0.723,
                tabs::hsplit(
                    3,
                    0.4,
                    tabs::group(2, [tabs::tab(3, "A file", true)]),
                    tabs::group(3, [tabs::tab(4, "A file", true)]),
                ),
                tabs::group(
                    4,
                    [
                        tabs::tab(5, "A file", false),
                        tabs::tab(6, "Another file", true),
                    ],
                ),
            ),
        )
    });

    cx.render(rsx! {
        section {
            id: "page",

            section {
                id: "header",
                MenuBar {
                    items: menu_items
                }
            }

            section {
                id: "workspace",
                TabArea {
                    layout: tab_layout,
                    on_layout_changed: |layout| {
                        tab_layout.set(layout);
                    }
                }
            }

            section {
                id: "footer"
            }
        }


    })
}
