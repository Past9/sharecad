use std::collections::HashMap;

use dioxus::prelude::*;

use super::{TabContentProps, TabId};

pub enum WorkspaceView {
    Welcome,
    Part(Part),
    Assembly(Assembly),
}

pub struct Part {
    pub text: String,
}

pub struct Assembly {}

pub struct WorkspaceViews {
    pub views: HashMap<TabId, WorkspaceView>,
}

#[allow(non_snake_case)]
pub fn WorkspaceTabContent<'a>(cx: Scope<'a, TabContentProps>) -> Element {
    let views = use_shared_state::<WorkspaceViews>(cx).unwrap();

    let views = views.read();
    let view = views.views.get(&cx.props.tab_id);

    if let Some(view) = view {
        match view {
            WorkspaceView::Welcome => cx.render(rsx! {
                div {
                    class: "welcome",
                    div {
                        class: "content",
                        div {
                            class: "title-container",
                            div {
                                class: "logo",
                                "Ϣ"
                            }
                            div {
                                class: "text",
                                div {
                                    class: "title",
                                    "ShareCAD"
                                }
                                div {
                                    class: "subtitle",
                                    "CAD for everyone"
                                }
                            }
                        }
                        div {
                            class: "quickstart",
                            div {
                                class: "title",
                                "Start"
                            }
                            div {
                                class: "action",
                                "New Part"
                            }
                            div {
                                class: "action",
                                "New Assemby"
                            }
                            div {
                                class: "action",
                                "Open File..."
                            }
                            div {
                                class: "action",
                                "Open Folder..."
                            }
                        }
                    }
                }
            }),
            WorkspaceView::Part(part) => cx.render(rsx! {
                div {
                    "Some part: {part.text}"
                }
            }),
            WorkspaceView::Assembly(assembly) => cx.render(rsx! {
                div {
                    "Some assembly"
                }
            }),
        }
    } else {
        cx.render(rsx! {
            div {
                "Could not load content for tab {cx.props.tab_id.num()}"
            }
        })
    }
}
