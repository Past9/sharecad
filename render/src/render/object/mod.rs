use std::cell::OnceCell;

use crate::{
    camera::Camera,
    model::{
        CurveInstanceRaw, CurveVertex, PointInstanceRaw, PointVertex, SurfaceInstanceRaw,
        SurfaceVertex,
    },
    scene::Scene,
};

use super::{
    pad_u32, texture::TextureResources, GlobalsRaw, MsaaSamples, RenderTarget, VertexBuffer,
};
use wgpu::util::DeviceExt;

const PIXEL_BYTES: u32 = 4;

pub struct ObjectRenderer {
    target: RenderTarget,
    depth_texture: OnceCell<TextureResources>,
    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,
    surface_pipeline: wgpu::RenderPipeline,
    curve_pipeline: wgpu::RenderPipeline,
    point_pipeline: wgpu::RenderPipeline,
    output_buffer: OnceCell<wgpu::Buffer>,
}
impl ObjectRenderer {
    pub fn new(target: RenderTarget) -> Self {
        let device = target.device();

        let (globals_bind_group_layout, globals_bind_group, globals_buffer) = {
            let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("globals-buffer"),
                contents: bytemuck::cast_slice(&[0u8; std::mem::size_of::<GlobalsRaw>()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let globals_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("globals-bind-group-layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

            let globals_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("globals-bind-group"),
                layout: &globals_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: globals_buffer.as_entire_binding(),
                }],
            });

            (
                globals_bind_group_layout,
                globals_bind_group,
                globals_buffer,
            )
        };

        let surface_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("object-surface-pipeline-layout"),
                bind_group_layouts: &[&globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("object-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let surface_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("object-surface-pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_surface",
                    buffers: &[SurfaceVertex::desc(), SurfaceInstanceRaw::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_surface",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target.format(),
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: TextureResources::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            surface_pipeline
        };

        let curve_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("object-curve-pipeline-layout"),
                bind_group_layouts: &[&globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("object-curve-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let curve_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("object-curve-pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_curve",
                    buffers: &[CurveVertex::desc(), CurveInstanceRaw::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_curve",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: target.format(),
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: TextureResources::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            curve_pipeline
        };

        let point_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("object-point-pipeline-layout"),
                bind_group_layouts: &[&globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("object-point-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let opaque_point_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("object-point-pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_point",
                        buffers: &[PointVertex::desc(), PointInstanceRaw::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_point",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: target.format(),
                            // Unlike the other opaque pipelines, this one uses
                            // alpha blending so we can feather the edges for cheap
                            // anti-aliasing.
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: TextureResources::DEPTH_FORMAT,
                        depth_write_enabled: false,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

            opaque_point_pipeline
        };

        Self {
            target,
            depth_texture: OnceCell::new(),
            globals_buffer,
            globals_bind_group,
            surface_pipeline,
            curve_pipeline,
            point_pipeline,
            output_buffer: OnceCell::new(),
        }
    }

    pub fn target(&self) -> &RenderTarget {
        &self.target
    }

    pub fn size(&self) -> (u32, u32) {
        self.target.size()
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 || new_size.1 > 0 {
            self.target.resize(new_size, MsaaSamples::Samples1);
            self.depth_texture = OnceCell::new();
            self.output_buffer = OnceCell::new();
        }
    }

    pub fn aspect(&self) -> f64 {
        self.target.aspect()
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), wgpu::SurfaceError> {
        let device = self.target.device();
        let queue = self.target.queue();
        let frame = self.target.frame();
        let view = frame.view();

        queue.write_buffer(
            &self.globals_buffer,
            0,
            //bytemuck::cast_slice(&[camera.to_raw(self.aspect())]),
            bytemuck::cast_slice(&[GlobalsRaw::build(scene, camera, self.aspect(), self.size())]),
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Visual render encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture().view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            // Render surfaces
            {
                render_pass.set_pipeline(&self.surface_pipeline);

                for object in scene.surfaces().iter() {
                    let mesh = object.mesh();
                    render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                    render_pass.set_index_buffer(
                        mesh.index_buffer(device).slice(..),
                        wgpu::IndexFormat::Uint32,
                    );

                    {
                        // TODO: Move these out of the loop? Probably don't need to set these for
                        // every object since they don't change.
                        render_pass.set_bind_group(0, &self.globals_bind_group, &[]);
                    }
                    render_pass.draw_indexed(0..mesh.num_elements(), 0, 0..object.num_instances());
                }
            }

            // Render curves
            {
                render_pass.set_pipeline(&self.curve_pipeline);

                render_pass.set_bind_group(1, &self.globals_bind_group, &[]);

                for object in scene.curves().iter() {
                    let mesh = object.mesh();

                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                    render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                    render_pass.set_index_buffer(
                        mesh.index_buffer(device).slice(..),
                        wgpu::IndexFormat::Uint32,
                    );

                    render_pass.draw_indexed(0..mesh.num_elements(), 0, 0..object.num_instances());
                }
            }

            // Render points
            {
                render_pass.set_pipeline(&self.point_pipeline);

                render_pass.set_bind_group(1, &self.globals_bind_group, &[]);

                for object in scene.points().iter() {
                    let mesh = object.mesh();

                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                    render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                    render_pass.set_index_buffer(
                        mesh.index_buffer(device).slice(..),
                        wgpu::IndexFormat::Uint32,
                    );

                    render_pass.draw_indexed(0..mesh.num_elements(), 0, 0..object.num_instances());
                }
            }
        }

        self.target
            .copy_to_buffer(&mut encoder, self.output_buffer());

        queue.submit(std::iter::once(encoder.finish()));
        frame.finish();

        Ok(())
    }

    fn depth_texture(&self) -> &TextureResources {
        self.depth_texture.get_or_init(|| {
            TextureResources::depth(
                self.target.device(),
                self.target.size(),
                MsaaSamples::Samples1,
            )
        })
    }

    fn output_buffer(&self) -> &wgpu::Buffer {
        self.output_buffer.get_or_init(|| {
            let buffer_size = (self.bytes_per_row() * self.target.size().1) as wgpu::BufferAddress;

            let buffer_desc = wgpu::BufferDescriptor {
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                label: None,
                mapped_at_creation: false,
            };

            self.target.context.device.create_buffer(&buffer_desc)
        })
    }

    fn bytes_per_row(&self) -> u32 {
        pad_u32(self.target.size().0 * PIXEL_BYTES, 256)
    }

    pub async fn get_id_at(&self, coords: (u32, u32)) -> u32 {
        let (x, y) = coords;

        if x >= self.target.size().0 {
            return 0;
        }

        if y >= self.target.size().1 {
            return 0;
        }

        let output: u32;
        let output_buffer = self.output_buffer();

        {
            let buffer_slice = output_buffer.slice(..);

            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.target.context.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let (prefix, pixels, suffix) = unsafe { data.align_to::<u32>() };

            if prefix.len() > 0 {
                panic!("data len = {}, prefix: {:?}", pixels.len(), prefix);
            }

            if suffix.len() > 0 {
                panic!("data len = {}, suffix: {:?}", pixels.len(), suffix);
            }

            let (x, y) = coords;
            let index = y * (self.bytes_per_row() / PIXEL_BYTES) + x;

            output = pixels[index as usize];
        }

        output_buffer.unmap();

        output
    }
}
