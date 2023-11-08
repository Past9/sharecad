use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Welcome<'a>(cx: Scope<'a, ()>) -> Element {
    cx.render(rsx! {
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
    })
}
