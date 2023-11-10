use dioxus::prelude::*;
use render::state::ViewState as RenderState;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

#[derive(Props, PartialEq)]
pub struct PartEditorProps {}

#[allow(non_snake_case)]
pub fn PartEditor<'a>(cx: Scope<'a, PartEditorProps>) -> Element {
    let canvas = use_state::<Option<HtmlCanvasElement>>(cx, || None);

    log::debug!("canvas {:?}", canvas);

    use_future(cx, (canvas), |(canvas)| async move {
        if let Some(canvas) = canvas.current().as_ref() {
            // let mut render_state = RenderState::new_on_canvas(canvas.clone(), "").await;
            // render_state.render().unwrap();
        }
    });

    cx.render(rsx! {
        div {
            class: "part-editor",
            canvas {
                class: "part-view",
                onmounted: move |evt| {
                    let element = evt
                        .get_raw_element()
                        .expect("Could not get canvas")
                        .downcast_ref::<web_sys::Element>()
                        .expect("Could not cast to element")
                        .clone()
                        .dyn_into::<HtmlCanvasElement>()
                        .expect("Could not cast to canvas");

                    canvas.set(Some(element));

                }
            }
        }
    })
}
