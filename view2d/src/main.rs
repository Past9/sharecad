use eframe::{
    egui::{self, Sense},
    egui_wgpu::{self, RenderState},
    epaint::Vec2,
    wgpu::{self, util::DeviceExt, BufferBinding, CommandBuffer, DynamicOffset},
    Renderer,
};
use std::{num::NonZeroU64, process::Command};

const RENDER_LABEL: Option<&'static str> = Some("View2D");

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1600.0, 900.0)),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };

    let mut sketch_pipeline_initialized = false;

    let mut name = "Ross".to_owned();

    //let mut sketch1 = SketchState::new(0);
    //let mut sketch2 = SketchState::new(1);

    let mut sketches = Vec::new();

    eframe::run_simple_native("View 2D", options, move |ctx, frame| {
        if !sketch_pipeline_initialized {
            init_sketch_pipeline(frame.wgpu_render_state().unwrap());
        }
        sketch_pipeline_initialized = true;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Some app");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });

            ui.horizontal(|ui| {
                /*
                ui.allocate_ui([300.0, 300.0].into(), |ui| {
                    ui.sketch(&mut sketch1);
                });

                ui.allocate_ui([300.0, 300.0].into(), |ui| {
                    ui.sketch(&mut sketch2);
                });
                 */
                for sketch in sketches.iter_mut() {
                    ui.allocate_ui([300.0, 300.0].into(), |ui| {
                        ui.sketch(sketch);
                    });
                }
            });

            if ui.button("Add").clicked() {
                println!("Adding sketch");
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
    renderer_initialized: bool,
    angle: f32,
}
impl SketchState {
    fn new(buffer_index: usize) -> Self {
        Self {
            buffer_index,
            renderer_initialized: false,
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
            let (buffers, bind_groups) =
                build_buffers(device, &resources.bind_group_layout, control_count);

            resources.uniform_buffers = buffers;
            resources.bind_groups = bind_groups;
        }

        queue.write_buffer(
            &resources.uniform_buffers[self.buffer_index],
            0,
            &bytemuck::cast_slice(&[self.angle, 0.0, 0.0, 0.0]),
        );

        Vec::new()
    }

    fn finish_prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        egui_encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &mut RenderResources = resources.get_mut().unwrap();

        vec![]
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

fn build_buffers(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    count: usize,
) -> (Vec<wgpu::Buffer>, Vec<wgpu::BindGroup>) {
    let buffers = (0..count)
        .map(|_| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: RENDER_LABEL,
                contents: bytemuck::cast_slice(&[0.0_f32; 4]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            })
        })
        .collect::<Vec<_>>();

    let bind_groups = buffers
        .iter()
        .map(|buffer| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: RENDER_LABEL,
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(BufferBinding {
                        buffer: buffer,
                        offset: 0,
                        size: NonZeroU64::new(16),
                    }),
                }],
            })
        })
        .collect::<Vec<_>>();

    (buffers, bind_groups)
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

    /*
    let uniform_buffers: Vec<wgpu::Buffer> = (0..2)
        .map(|_| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: RENDER_LABEL,
                contents: bytemuck::cast_slice(&[0.0_f32; 4]),
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            })
        })
        .collect();

    let bind_groups = uniform_buffers
        .iter()
        .enumerate()
        .map(|(i, uniform_buffer)| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: RENDER_LABEL,
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(BufferBinding {
                        buffer: uniform_buffer,
                        offset: 0,
                        size: NonZeroU64::new(16),
                    }),
                }],
            })
        })
        .collect::<Vec<_>>();
     */

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
