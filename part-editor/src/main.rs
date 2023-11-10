use eframe::{
    egui,
    egui_wgpu::{self, RenderState, WgpuConfiguration},
    wgpu::{self, Features},
    Renderer,
};
use render::{render::RenderContext, state::ViewState};
use std::sync::{Arc, Mutex};

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

    let mut editor_state = PartEditorState::new();

    let mut renderer_initialized = false;

    eframe::run_simple_native("Part Editor", options, move |ctx, frame| {
        if !renderer_initialized {
            let render_state = frame.wgpu_render_state().unwrap();
            let render_context = ViewState::new_from_resources(render_state, env!("OUT_DIR"));

            renderer_initialized = true;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Part Editor");
            ui.allocate_ui([800.0, 800.0].into(), |ui| {
                ui.part_editor(&mut editor_state);
                //
            });
        });
    })
}

#[derive(Clone)]
struct PartEditorStateInner {}
impl PartEditorStateInner {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
struct PartEditorState {
    inner: Arc<Mutex<PartEditorStateInner>>,
}
impl PartEditorState {
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(PartEditorStateInner::new()));
        Self { inner }
    }
}
impl egui_wgpu::CallbackTrait for PartEditorState {
    fn paint<'a>(
        &'a self,
        info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        callback_resources: &'a egui_wgpu::CallbackResources,
    ) {
        //todo!()
    }
}

trait PartEditorUi {
    fn part_editor(self, state: &mut PartEditorState);
}
impl PartEditorUi for &mut egui::Ui {
    fn part_editor(self, state: &mut PartEditorState) {
        egui::Frame::canvas(self.style()).show(self, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            ui.painter()
                .add(egui_wgpu::Callback::new_paint_callback(rect, state.clone()));
        });
    }
}

fn init_renderer(render_state: &RenderState) -> ViewState {
    todo!()
}
