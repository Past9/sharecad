use super::{menu, tabs};
use crate::components::{LayoutBuilder, MenuBar, Tabs};
use dioxus::prelude::*;

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

    let tab_config = use_state(cx, || {
        LayoutBuilder::new()
            .vsplit(|cx| {
                cx.group(|cx| {
                    cx.tab("File1");
                });
                cx.group(|cx| {
                    cx.tab("File2");
                });
                cx.hsplit(|cx| {
                    cx.group(|cx| {
                        cx.tab("File3");
                    });
                    cx.group(|cx| {
                        cx.tab("File4-1");
                        cx.tab("File4-2");
                    });
                    cx.group(|cx| {
                        cx.tab("File5");
                    });
                    cx.group(|cx| {
                        cx.tab("File6");
                    });
                });
                cx.group(|cx| {
                    cx.tab("File7");
                });
                cx.group(|cx| {
                    cx.tab("File8");
                });
            })
            .as_new_config()
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
                Tabs {
                    config: tab_config,
                    on_config_changed: |config| {
                        tab_config.set(config);
                    }
                }
            }

            section {
                id: "footer"
            }
        }


    })
}
