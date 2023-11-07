use std::collections::HashMap;

use super::{menu, Tab, TabId, TabsCommand, WorkspaceView, WorkspaceViews};
use crate::{
    command::{Command, CommandBus},
    components::{LayoutBuilder, MenuBar, PartView, Tabs, WorkspaceTabContent},
};
use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GlobalCommand {
    NewPart,
    NewAssembly,
}
impl Command for GlobalCommand {
    const TYPE_NAME: &'static str = "GlobalCommand";
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
    let bus = cx.use_hook(|| CommandBus::<GlobalCommand>::new());

    let menu_items = [
        menu::item(
            "File",
            true,
            [
                menu::item("New part", true, [], Some(GlobalCommand::NewPart)),
                menu::item("New assembly", true, [], Some(GlobalCommand::NewAssembly)),
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
            .focus_tab(TabId::new(1))
            .as_new_config()
    });

    use_shared_state_provider(cx, || WorkspaceViews {
        views: HashMap::from([(TabId::new(1), WorkspaceView::Welcome)]),
    });

    let workspace_views = use_shared_state::<WorkspaceViews>(cx).unwrap();

    bus.listen(cx, |cmd| {
        match cmd {
            GlobalCommand::NewPart => {
                tab_config.modify(|cfg| {
                    let mut cfg = cfg.clone();
                    let (group_id, index) = match cfg.layout.find_focused_tab() {
                        Some(tab_id) => match cfg.layout.find_tab_group(tab_id) {
                            Some(group_id) => match cfg.layout.find_tab_index(tab_id) {
                                Some(index) => (group_id, index + 1),
                                None => return cfg,
                            },
                            None => return cfg,
                        },
                        None => return cfg,
                    };

                    let mut new_layout = cfg
                        .layout
                        .create_new_tab(group_id, index, "Untitled Part")
                        .clean();

                    let tab_id = new_layout.highest_tab_id();
                    workspace_views.write().views.insert(
                        tab_id,
                        WorkspaceView::Part(PartView {
                            text: "This is a part".to_string(),
                        }),
                    );

                    new_layout = new_layout.focus_tab(tab_id).activate_one_tab_per_group();

                    cfg.layout = new_layout;

                    cfg
                });
            }
            GlobalCommand::NewAssembly => {
                //
            }
        }
    });

    log::debug!("layout {:#?}", tab_config.layout);

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
