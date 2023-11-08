use std::collections::HashMap;

use dioxus::prelude::*;

use crate::components::{PartEditor, Welcome};

use super::{TabContentProps, TabId};

pub enum WorkspaceView {
    Welcome,
    Part(PartView),
    Assembly(AssemblyView),
}

pub struct PartView {
    pub text: String,
}

pub struct AssemblyView {}

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
                Welcome { }
            }),
            WorkspaceView::Part(part) => cx.render(rsx! {
                PartEditor {

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
