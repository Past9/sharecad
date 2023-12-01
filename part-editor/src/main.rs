mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::{self, RenderState, WgpuConfiguration},
    wgpu::{self, Features},
    Renderer,
};
use geometry::Curve3;
use render::{
    color::rgb,
    input::InputEvent,
    light::AmbientLight,
    render::{EguiTransfer, MsaaSamples},
    state::ViewState,
};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let get_device_descriptor = |_adapter: &wgpu::Adapter| -> wgpu::DeviceDescriptor<'static> {
        wgpu::DeviceDescriptor {
            features: Features::POLYGON_MODE_LINE,
            ..Default::default()
        }
    };

    let msaa_samples = MsaaSamples::Samples4;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 1024.0)),
        renderer: Renderer::Wgpu,
        wgpu_options: WgpuConfiguration {
            device_descriptor: Arc::new(get_device_descriptor),
            ..Default::default()
        },
        multisampling: 1, // msaa_samples.samples() as u16,
        ..Default::default()
    };

    let mut editor_state: Option<EditorState> = None;

    eframe::run_simple_native("Part Editor", options, move |ctx, frame| {
        let editor_state_left =
            editor_state.get_or_insert_with(|| EditorState::new(ctx, frame, msaa_samples));

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            if ui.button("Sketch").clicked() {
                println!("Sketch");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.editor(editor_state_left);
        });
    })
}

// BREP model
struct PartModel {
    edges: Vec<Curve3>,
}
impl PartModel {
    pub fn new(edges: Vec<Curve3>) -> Self {
        Self { edges }
    }
}
