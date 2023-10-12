use bytemuck::{Pod, Zeroable};
use eframe::{
    egui::{self},
    egui_wgpu::{self, RenderState, WgpuConfiguration},
    wgpu::{self, util::DeviceExt, BufferBinding, CommandBuffer, Features},
    Renderer,
};
use space::{deg, point2, Angle, Point2, Vec2};
use std::{num::NonZeroU64, sync::Arc};

const RENDER_LABEL: Option<&'static str> = Some("Sketch");

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    pos: [f32; 2],
    dir: [f32; 2],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

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

    let mut sketch_pipeline_initialized = false;
    let mut sketches = vec![SketchState::new(0)];

    eframe::run_simple_native("View 2D", options, move |ctx, frame| {
        if !sketch_pipeline_initialized {
            init_sketch_pipeline(frame.wgpu_render_state().unwrap());
        }
        sketch_pipeline_initialized = true;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Some app");
            ui.horizontal(|ui| {
                for sketch in sketches.iter_mut() {
                    ui.allocate_ui([800.0, 800.0].into(), |ui| {
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
    vertex_buffers: Vec<wgpu::Buffer>,
    index_buffers: Vec<wgpu::Buffer>,
}

#[derive(Clone)]
struct SketchState {
    buffer_index: usize,
    angle: f32,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}
impl SketchState {
    fn new(buffer_index: usize) -> Self {
        /*
        let points = vec![
            point2(0.0, -0.7), //
            point2(0.0, 0.7),  //
            point2(0.5, 0.7),  //
        ];
        */

        let points = vec![
            point2(0.0, 0.0),   //
            point2(0.2, 0.0),   //
            point2(0.2, 0.2),   //
            point2(0.0, 0.2),   //
            point2(-0.2, -0.2), //
            point2(0.0, -0.1),  //
            point2(0.2, -0.4),  //
            point2(0.2, -0.6),  //
            point2(-0.2, -0.6), //
            point2(0.6, -0.5),  //
            point2(0.6, 0.5),   //
            point2(0.6, 0.8),   //
            point2(0.6, 0.3),   //
        ];

        let mut vertices = Vec::with_capacity((points.len() - 1) * 4);

        for i in 1..points.len() {
            let p0 = points[i - 1];
            let p1 = points[i];
            let line_dir = (p1 - p0).to_f32s();
            let p0_pos = p0.to_f32s();
            let p1_pos = p1.to_f32s();
            vertices.extend([
                Vertex {
                    pos: p0_pos,
                    dir: line_dir,
                },
                Vertex {
                    pos: p0_pos,
                    dir: line_dir,
                },
                Vertex {
                    pos: p1_pos,
                    dir: line_dir,
                },
                Vertex {
                    pos: p1_pos,
                    dir: line_dir,
                },
            ]);
        }

        println!("vertices = {:#?}", vertices);
        println!("vertices.len() = {}", vertices.len());

        let indices = (1..points.len())
            .flat_map(|i| {
                let i = (i as u32 - 1) * 4;
                [
                    // First triangle
                    i + 2,
                    i,
                    i + 1,
                    // Second triangle
                    i + 2,
                    i + 1,
                    i + 3,
                ]
            })
            .collect::<Vec<_>>();

        println!("indices = {:?}", indices);
        println!("indices.len() = {}", indices.len());

        //let (vertices, indices) = line.to_verts(&Stroke::new(0.02, deg(15.0)));

        Self {
            buffer_index,
            angle: 0.0,
            vertices,
            indices,
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

            resources.vertex_buffers.push(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: RENDER_LABEL,
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
                },
            ));

            resources.index_buffers.push(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: RENDER_LABEL,
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::INDEX,
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
        render_pass.set_vertex_buffer(0, resources.vertex_buffers[self.buffer_index].slice(..));
        render_pass.set_index_buffer(
            resources.index_buffers[self.buffer_index].slice(..),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.set_bind_group(0, &resources.bind_groups[self.buffer_index], &[]);

        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
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
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            //targets: &[Some(render_state.target_format.into())],
            targets: &[Some(wgpu::ColorTargetState {
                format: render_state.target_format.into(),

                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: wgpu::BlendComponent::OVER,
                }),

                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        },
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
            vertex_buffers: vec![],
            index_buffers: vec![],
        });
}
