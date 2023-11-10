use eframe::{
    egui,
    egui_wgpu::{self, RenderState, WgpuConfiguration},
    wgpu::{self, Features},
    Renderer,
};
use render::{
    render::{EguiTransfer, RenderContext},
    state::ViewState,
};
use std::{
    cell::OnceCell,
    iter::Once,
    sync::{Arc, Mutex},
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let get_device_descriptor = |_adapter: &wgpu::Adapter| -> wgpu::DeviceDescriptor<'static> {
        wgpu::DeviceDescriptor {
            features: Features::POLYGON_MODE_LINE,
            ..Default::default()
        }
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 900.0)),
        renderer: Renderer::Wgpu,
        wgpu_options: WgpuConfiguration {
            device_descriptor: Arc::new(get_device_descriptor),
            ..Default::default()
        },
        ..Default::default()
    };

    let mut editor_state: Option<EditorState> = None;

    eframe::run_simple_native("Part Editor", options, move |ctx, frame| {
        let editor_state =
            editor_state.get_or_insert_with(|| EditorState::new(frame, PartModel::new()));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Part Editor");
            ui.allocate_ui([800.0, 800.0].into(), |ui| {
                ui.part_editor(editor_state);
                //
            });
        });
    })
}

struct PartModel {}
impl PartModel {
    pub fn new() -> Self {
        Self {}
    }
}

struct EditorStateInner {
    view_state: ViewState,
    //transfer: EguiTransfer,
    model: PartModel,
}
impl EditorStateInner {
    pub fn new(frame: &eframe::Frame, model: PartModel) -> Self {
        println!("EditorStateInner::new");
        let render_state = frame.wgpu_render_state().unwrap();

        let view_state = ViewState::new_from_resources(
            render_state,
            env!("OUT_DIR"),
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
        );

        init_transfer(
            render_state,
            view_state.visual_target().texture_view().unwrap(),
        );

        /*
        let transfer = EguiTransfer::new(
            render_state,
            view_state.visual_target().texture_view().unwrap(),
        );
         */

        Self {
            view_state,
            //transfer,
            model,
        }
    }
}

#[derive(Clone)]
struct EditorState {
    inner: Arc<Mutex<EditorStateInner>>,
}
impl EditorState {
    pub fn new(frame: &eframe::Frame, model: PartModel) -> Self {
        println!("EditorState::new");
        let inner = Arc::new(Mutex::new(EditorStateInner::new(frame, model)));
        Self { inner }
    }
}
impl egui_wgpu::CallbackTrait for EditorState {
    fn paint<'p>(
        &'p self,
        info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'p>,
        callback_resources: &'p egui_wgpu::CallbackResources,
    ) {
        let mut state = self.inner.lock().unwrap();

        state.view_state.resize((
            info.viewport_in_pixels().width_px as u32,
            info.viewport_in_pixels().height_px as u32,
        ));
        state.view_state.render().unwrap();

        let transfer: &EguiTransfer = callback_resources.get().unwrap();
        transfer.transfer(render_pass);
    }
}

trait PartEditorUi {
    fn part_editor(self, state: &mut EditorState);
}
impl PartEditorUi for &mut egui::Ui {
    fn part_editor(self, state: &mut EditorState) {
        let state = state.clone();
        egui::Frame::canvas(self.style()).show(self, move |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());
            ui.painter()
                .add(egui_wgpu::Callback::new_paint_callback(rect, state));
        });
    }
}

fn init_transfer(render_state: &RenderState, texture_view: &wgpu::TextureView) {
    let transfer = EguiTransfer::new(render_state, texture_view);
    render_state
        .renderer
        .write()
        .callback_resources
        .insert(transfer);
}
