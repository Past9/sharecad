use std::collections::HashMap;

use super::{menu, TabId, WorkspaceView, WorkspaceViews};
use crate::{
    command::CommandBus,
    components::{LayoutBuilder, MenuBar, Tabs, WorkspaceTabContent},
};
use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GlobalCommand {
    NewPart,
    NewAssembly,
}

pub struct AppState {
    //
}
impl AppState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn modify(&mut self, command: &GlobalCommand) {
        todo!()
    }
}

#[allow(non_snake_case)]
pub fn MainWindow(cx: Scope) -> Element {
    let bus = cx
        .use_hook(|| CommandBus::<GlobalCommand>::new())
        .listen(cx, |cmd| {
            log::debug!("cmd {:?}", cmd);
        });

    let menu_items = [
        menu::item(
            "File",
            true,
            [
                menu::item("New part", true, [], None),
                menu::item("New assembly", true, [], None),
                menu::sep(),
                menu::item("Open file", true, [], None),
                menu::item(
                    "Open some recent file",
                    true,
                    [
                        menu::item("some/file/part1.prt", true, [], None),
                        menu::item("some/file/part2.prt", true, [], None),
                        menu::item("some/file/assembly1.asm", true, [], None),
                        menu::item("some/file/assembly2.asm", true, [], None),
                    ],
                    None,
                ),
                menu::sep(),
                menu::item("Save", false, [], None),
                menu::item("Save as...", false, [], None),
                menu::sep(),
                menu::item(
                    "Settings",
                    true,
                    [
                        menu::item("Option 1", false, [], None),
                        menu::item("Option 2", false, [], None),
                    ],
                    None,
                ),
                menu::item("Exit", true, [], None),
            ],
            None,
        ),
        menu::item("Edit", true, [], None),
        menu::item(
            "Help",
            true,
            [
                menu::item("About", true, [], None),
                menu::item(
                    "Open source licenses",
                    true,
                    [
                        menu::item("Package 1", true, [], None),
                        menu::item(
                            "Package 2",
                            true,
                            [
                                menu::item("Version 1", true, [], None),
                                menu::item("Version 2", true, [], None),
                            ],
                            None,
                        ),
                        menu::item("Package 3", true, [], None),
                    ],
                    None,
                ),
            ],
            None,
        ),
    ]
    .to_vec();

    let tab_config = use_state(cx, || {
        LayoutBuilder::new()
            .group(|cx| {
                cx.tab("Welcome");
                cx.tab("Something");
            })
            .as_new_config()
    });

    use_shared_state_provider(cx, || WorkspaceViews {
        views: HashMap::from([(TabId::new(1), WorkspaceView::Welcome)]),
    });

    cx.render(rsx! {
        section {
            id: "page",

            section {
                id: "header",
                MenuBar {
                    items: menu_items,
                    bus: bus.clone()
                }
            }

            section {
                id: "workspace",
                Tabs {
                    config: tab_config,
                    on_config_changed: |config| {
                        tab_config.set(config);
                    },
                    render_content: WorkspaceTabContent
                }
            }

            section {
                id: "footer"
            }
        }


    })
}
