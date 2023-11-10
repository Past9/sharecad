use eframe::{
    egui,
    egui_wgpu::{self, RenderState, WgpuConfiguration},
    epaint::Rect,
    wgpu::{self, Features},
    Renderer,
};
use render::{
    input::InputEvent,
    render::{EguiTransfer, RenderContext},
    state::ViewState,
};
use space::point2;
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
        let editor_state_left =
            editor_state.get_or_insert_with(|| EditorState::new(frame, PartModel::new()));

        egui::SidePanel::left("history-panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("History");
                ui.separator();
            });

        egui::SidePanel::right("config-panel")
            .min_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Configuration");
                ui.separator();
            });

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            if ui.button("Sketch").clicked() {
                println!("Sketch");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.part_editor(editor_state_left);
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
    resized: bool,
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
            resized: false,
            view_state,
            //transfer,
            model,
        }
    }
}

struct RenderResources {
    transfer: EguiTransfer,
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
    fn prepare(
        &self,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        _egui_encoder: &mut wgpu::CommandEncoder,
        callback_resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let mut state = self.inner.lock().unwrap();

        if state.resized {
            let res: &mut RenderResources = callback_resources.get_mut().unwrap();
            res.transfer
                .rebind_texture(state.view_state.visual_target().texture_view().unwrap());
        }

        state.resized = false;

        vec![]
    }

    fn paint<'p>(
        &'p self,
        info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'p>,
        callback_resources: &'p egui_wgpu::CallbackResources,
    ) {
        let mut state = self.inner.lock().unwrap();
        let res: &RenderResources = callback_resources.get().unwrap();

        state.resized = state.view_state.resize((
            info.viewport_in_pixels().width_px as u32,
            info.viewport_in_pixels().height_px as u32,
        ));

        state.view_state.render().unwrap();

        res.transfer.transfer(render_pass);
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

            // Mousemove events
            if let Some(hover_pos) = response.hover_pos() {
                let pos = point2(
                    (hover_pos.x - rect.left()) as f64,
                    (hover_pos.y - rect.top()) as f64,
                );

                if pos.x >= 0.0
                    && pos.y >= 0.0
                    && pos.x <= rect.width() as f64
                    && pos.y <= rect.height() as f64
                {
                    println!("mousemove {}", pos);
                }
            }

            ui.input(|input| {
                if input.events.len() > 0 {
                    let mut inner = state.inner.lock().unwrap();
                    for event in input.events.iter() {
                        inner.view_state.input(&InputEvent::from(event.to_owned()));
                    }
                }
            });

            ui.painter()
                .add(egui_wgpu::Callback::new_paint_callback(rect, state));
        });
    }
}

fn init_transfer(render_state: &RenderState, texture_view: &wgpu::TextureView) {
    let transfer = EguiTransfer::new(render_state, texture_view);

    let res = RenderResources { transfer };

    render_state.renderer.write().callback_resources.insert(res);
}
