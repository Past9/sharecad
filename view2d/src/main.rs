use eframe::{
    egui::{self},
    egui_wgpu::{self, RenderState},
    wgpu::{self, util::DeviceExt, BufferBinding, CommandBuffer},
    Renderer,
};
use std::num::NonZeroU64;

const RENDER_LABEL: Option<&'static str> = Some("View2D");

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 900.0)),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    let mut sketch_pipeline_initialized = false;
    let mut sketches = Vec::new();

    eframe::run_simple_native("View 2D", options, move |ctx, frame| {
        if !sketch_pipeline_initialized {
            init_sketch_pipeline(frame.wgpu_render_state().unwrap());
        }
        sketch_pipeline_initialized = true;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Some app");
            ui.horizontal(|ui| {
                for sketch in sketches.iter_mut() {
                    ui.allocate_ui([300.0, 300.0].into(), |ui| {
                        ui.sketch(sketch);
                    });
                }
            });

            if ui.button("Add").clicked() {
                sketches.push(SketchState::new(sketches.len()));
            }
        });
    })
}

struct RenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
    uniform_buffers: Vec<wgpu::Buffer>,
}

#[derive(Clone)]
struct SketchState {
    buffer_index: usize,
    angle: f32,
}
impl SketchState {
    fn new(buffer_index: usize) -> Self {
        Self {
            buffer_index,
            angle: 0.0,
        }
    }
}
impl egui_wgpu::CallbackTrait for SketchState {
    fn prepare(
        &self,
        device: &eframe::wgpu::Device,
        queue: &eframe::wgpu::Queue,
        _encoder: &mut eframe::wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<CommandBuffer> {
        let resources: &mut RenderResources = resources.get_mut().unwrap();

        let control_count = self.buffer_index + 1;
        if resources.uniform_buffers.len() < control_count {
            resources.uniform_buffers.push(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: RENDER_LABEL,
                    contents: bytemuck::cast_slice(&[0.0_f32; 4]),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                },
            ));

            resources
                .bind_groups
                .push(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: RENDER_LABEL,
                    layout: &resources.bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(BufferBinding {
                            buffer: &resources.uniform_buffers[self.buffer_index],
                            offset: 0,
                            size: NonZeroU64::new(16),
                        }),
                    }],
                }));
        }

        queue.write_buffer(
            &resources.uniform_buffers[self.buffer_index],
            0,
            &bytemuck::cast_slice(&[self.angle, 0.0, 0.0, 0.0]),
        );

        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: eframe::epaint::PaintCallbackInfo,
        render_pass: &mut eframe::wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources: &RenderResources = resources.get().unwrap();
        render_pass.set_pipeline(&resources.pipeline);

        render_pass.set_bind_group(0, &resources.bind_groups[self.buffer_index], &[]);

        render_pass.draw(0..3, 0..1);
    }
}

trait SketchUi {
    fn sketch(self, state: &mut SketchState);
}
impl SketchUi for &mut egui::Ui {
    fn sketch(self, state: &mut SketchState) {
        egui::Frame::canvas(self.style()).show(self, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            state.angle += response.drag_delta().x * 0.01;

            ui.painter()
                .add(egui_wgpu::Callback::new_paint_callback(rect, state.clone()));
        });
    }
}

fn init_sketch_pipeline(render_state: &RenderState) {
    let device = &render_state.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: RENDER_LABEL,
        source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: RENDER_LABEL,
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(16),
            },
            count: None,
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: RENDER_LABEL,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: RENDER_LABEL,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(render_state.target_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    render_state
        .renderer
        .write()
        .callback_resources
        .insert(RenderResources {
            pipeline,
            bind_group_layout,
            bind_groups: vec![],
            uniform_buffers: vec![],
        });
}
