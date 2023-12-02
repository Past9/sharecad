use super::{pad_u32, texture::TextureResources, MsaaSamples, RenderTarget, VertexBuffer};
use crate::{
    camera::{Camera, CameraRaw},
    model::{ModelInstanceRaw, SurfaceVertexRaw},
    scene::Scene,
};
use space::{vec3, Point3, Vec3};
use std::cell::OnceCell;
use wgpu::util::DeviceExt;

pub struct PositionRenderer {
    target: RenderTarget,

    depth_texture: OnceCell<TextureResources>,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    render_pipeline: wgpu::RenderPipeline,

    output_buffer: OnceCell<wgpu::Buffer>,
}
impl PositionRenderer {
    pub fn new(target: RenderTarget) -> Self {
        let device = target.device();

        let (camera_bind_group_layout, camera_bind_group, camera_buffer) = {
            let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[0u8; std::mem::size_of::<CameraRaw>()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let camera_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("camera_bind_group_layout"),
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

            let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("camera_bind_group"),
                layout: &camera_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }],
            });

            (camera_bind_group_layout, camera_bind_group, camera_buffer)
        };

        let render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Mesh Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Visual shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[SurfaceVertexRaw::desc(), ModelInstanceRaw::surface_desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
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

            render_pipeline
        };

        Self {
            target,

            depth_texture: OnceCell::new(),

            camera_bind_group,
            camera_buffer,

            render_pipeline,

            output_buffer: OnceCell::new(),
        }
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
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera.to_raw(self.aspect())]),
        );

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("visual-render-encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("visual-render-pass"),
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

            render_pass.set_pipeline(&self.render_pipeline);

            for model in scene.models().iter() {
                for surface in model.surfaces().iter() {
                    render_pass
                        .set_vertex_buffer(1, model.surface_instance_buffer(device).slice(..));
                    render_pass.set_vertex_buffer(0, surface.vertex_buffer(device).slice(..));
                    render_pass.set_index_buffer(
                        surface.index_buffer(device).slice(..),
                        wgpu::IndexFormat::Uint32,
                    );

                    {
                        // TODO: Move these out of the loop? Probably don't need to set these for
                        // every object since they don't change.
                        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
                    }
                    render_pass.draw_indexed(
                        0..surface.num_elements(),
                        0,
                        0..model.num_instances(),
                    );
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
            let bytes_per_row = pad_u32(
                self.target.format().block_size(None).unwrap() * self.target.size().0,
                256,
            );
            let buffer_size = (bytes_per_row * self.target.size().1) as wgpu::BufferAddress;

            let buffer_desc = wgpu::BufferDescriptor {
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                label: None,
                mapped_at_creation: false,
            };

            self.target.context.device.create_buffer(&buffer_desc)
        })
    }

    pub async fn visit_pixels<T>(&self, visitor: impl FnOnce(&[[f32; 4]]) -> T) -> T {
        let output: T;
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

            let (prefix, pixels, suffix) = unsafe { data.align_to::<[f32; 4]>() };

            if prefix.len() > 0 {
                panic!("data len = {}, prefix: {:?}", pixels.len(), prefix);
            }

            if suffix.len() > 0 {
                panic!("data len = {}, suffix: {:?}", pixels.len(), suffix);
            }

            output = visitor(pixels);
        }

        output_buffer.unmap();

        output
    }

    pub async fn get_avg_pos(&self) -> Point3 {
        let mut avg_pos = Vec3::ZERO;

        // We need to scope the mapping variables so that we can
        // unmap the buffer
        let output_buffer = self.output_buffer();

        {
            let buffer_slice = self.output_buffer().slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.target.context.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            let (prefix, pixels, suffix) = unsafe { data.align_to::<[f32; 4]>() };

            if prefix.len() > 0 {
                panic!("data len = {}, prefix: {:?}", pixels.len(), prefix);
            }

            if prefix.len() > 0 {
                panic!("data len = {}, suffix: {:?}", pixels.len(), suffix);
            }

            let mut total_weight: f64 = 0.0;
            for pixel in pixels.iter() {
                if pixel[3] == 0.0 {
                    continue;
                }

                let weight = 1.0; // TODO: Weighting function

                avg_pos += vec3(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64) * weight;
                total_weight += weight;
            }

            if total_weight > 0.0 {
                avg_pos = avg_pos / total_weight;
            }
        }

        output_buffer.unmap();

        avg_pos.into_point()
    }
}
