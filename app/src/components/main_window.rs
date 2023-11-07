use std::collections::HashMap;

use super::{menu, TabId, WorkspaceView, WorkspaceViews};
use crate::{
    command::CommandBus,
    components::{LayoutBuilder, MenuBar, Tabs, WorkspaceTabContent},
};
use dioxus::prelude::*;

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
    /*
    let (sender, receiver) = cx.use_hook(|| async_channel::unbounded::<GlobalCommand>());
    let bus = CommandBus::new(sender.clone());

    let next_command = use_state::<Option<GlobalCommand>>(cx, || None);

    let _cr = {
        to_owned![next_command, receiver];
        use_coroutine(cx, |_rx: UnboundedReceiver<()>| async move {
            loop {
                if let Ok(command) = receiver.recv().await {
                    next_command.set(Some(command));
                }
            }
        })
    };

    if let Some(command) = &**next_command {
        next_command.set(None);

        let new_config = cx.props.state.modify(command);
    }
     */

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
                    items: menu_items
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
