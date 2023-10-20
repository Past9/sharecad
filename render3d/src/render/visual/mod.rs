use std::{collections::HashMap, sync::Arc};

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{
    camera::{Camera, CameraUniform},
    light::LightUniform,
    model::{InstanceRaw, MeshVertex},
    scene::Scene,
    texture::Texture,
};

use super::VertexBuffer;

pub struct VisualRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),

    texture_bind_group_layout: wgpu::BindGroupLayout,
    depth_texture: Texture,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,

    render_pipeline: wgpu::RenderPipeline,
}
impl VisualRenderer {
    pub async fn new(window: &Window) -> Self {
        let size = {
            let size = window.inner_size();
            (size.width, size.height)
        };

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    // Diffuse texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Diffuse sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Normal texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    // Normal sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let (camera_bind_group_layout, camera_bind_group, camera_buffer) = {
            let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[0u8; std::mem::size_of::<CameraUniform>()]),
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

        let (light_buffer, light_bind_group_layout, light_bind_group) = {
            let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[0u8; std::mem::size_of::<LightUniform>()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let light_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    label: None,
                });

            let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &light_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_buffer.as_entire_binding(),
                }],
                label: None,
            });

            (light_buffer, light_bind_group_layout, light_bind_group)
        };

        let depth_texture = Texture::depth("Depth texture");

        let render_pipeline = {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &texture_bind_group_layout,
                        &camera_bind_group_layout,
                        &light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Visual shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[MeshVertex::desc(), InstanceRaw::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
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
                    format: Texture::DEPTH_FORMAT,
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
            device,
            queue,
            surface,
            config,
            size,

            texture_bind_group_layout,
            depth_texture,

            camera_bind_group,
            camera_buffer,

            light_bind_group,
            light_buffer,

            render_pipeline,
        }
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 || new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = Texture::depth("Depth texture");
        }
    }

    pub fn aspect(&self) -> f64 {
        self.size.0 as f64 / self.size.1 as f64
    }

    pub fn render(&self, scene: &Scene, camera: &Camera) -> Result<(), wgpu::SurfaceError> {
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera.to_raw(self.aspect())]),
        );

        self.queue.write_buffer(
            &self.light_buffer,
            0,
            bytemuck::cast_slice(&[scene.light().to_raw()]),
        );

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Visual render encoder"),
            });

        let material_bind_groups = {
            let mut material_bind_groups = HashMap::new();

            for object in scene.objects().iter() {
                let material = scene.material(&object.material_id()).expect(&format!(
                    "Could not find material {:?}",
                    object.material_id()
                ));
                let material_bind_group = material.bind_group(
                    &self.device,
                    &self.queue,
                    &self.config,
                    &self.texture_bind_group_layout,
                );
                material_bind_groups.insert(object.material_id(), material_bind_group);
            }

            material_bind_groups
        };

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self
                        .depth_texture
                        .resources(&self.device, &self.queue, &self.config)
                        .view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);

            for object in scene.objects().iter() {
                let mesh = object.mesh();
                render_pass.set_vertex_buffer(1, object.instance_buffer(&self.device).slice(..));
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer(&self.device).slice(..));
                render_pass.set_index_buffer(
                    mesh.index_buffer(&self.device).slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.set_bind_group(
                    0,
                    &material_bind_groups.get(&object.material_id()).unwrap(),
                    &[],
                );
                {
                    // TODO: Move these out of the loop? Probably don't need to set these for
                    // every object since they don't change.
                    render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
                    render_pass.set_bind_group(2, &self.light_bind_group, &[]);
                }
                render_pass.draw_indexed(0..mesh.num_elements(), 0, 0..object.num_instances());
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
