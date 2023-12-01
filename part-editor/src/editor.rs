use std::sync::{Arc, Mutex};

use eframe::{
    egui,
    egui_wgpu::{self, RenderState},
    wgpu,
};
use render::{
    input::InputEvent,
    render::{EguiTransfer, MsaaSamples},
    state::ViewState,
};

struct RenderResources {
    transfer: EguiTransfer,
}

#[derive(Clone)]
pub struct EditorState {
    inner: Arc<Mutex<StateInner>>,
}
impl EditorState {
    pub fn new(ctx: &egui::Context, frame: &eframe::Frame, msaa_samples: MsaaSamples) -> Self {
        let inner = Arc::new(Mutex::new(StateInner::new(ctx, frame, msaa_samples)));
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

struct StateInner {
    resized: bool,
    view_state: ViewState,
}
impl StateInner {
    pub fn new(ctx: &egui::Context, frame: &eframe::Frame, msaa_samples: MsaaSamples) -> Self {
        let render_state = frame.wgpu_render_state().unwrap();

        let view_state = ViewState::new_from_resources(
            render_state,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
            msaa_samples,
            ctx.pixels_per_point(),
        );

        init_transfer(
            render_state,
            view_state.visual_target().texture_view().unwrap(),
        );

        Self {
            resized: false,
            view_state,
        }
    }
}

pub trait EditorUi {
    fn editor(self, state: &mut EditorState);
}
impl EditorUi for &mut egui::Ui {
    fn editor(self, state: &mut EditorState) {
        let state = state.clone();
        egui::Frame::canvas(self.style()).show(self, move |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            ui.input(|input| {
                if input.events.len() > 0 {
                    let mut inner = state.inner.lock().unwrap();
                    for event in input.events.iter() {
                        inner.view_state.update();
                        inner.view_state.input(&InputEvent::from_egui_event(
                            event,
                            &rect,
                            ui.ctx().pixels_per_point(),
                        ));
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
