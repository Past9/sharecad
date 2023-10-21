use std::{cell::OnceCell, collections::HashMap, sync::Arc};
use wgpu::util::DeviceExt;

use crate::{
    camera::{Camera, CameraUniform},
    light::LightUniform,
    material::{Material, MaterialId},
    model::{InstanceRaw, MeshVertex},
    scene::Scene,
    texture::Texture,
};

use super::{RenderTarget, VertexBuffer};

#[derive(Debug)]
struct TextureResources {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
impl TextureResources {
    const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    fn depth(device: &wgpu::Device, size: (u32, u32)) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}

pub struct VisualRenderer {
    target: RenderTarget,

    texture_bind_group_layout: wgpu::BindGroupLayout,
    depth_texture: OnceCell<TextureResources>,

    material_bind_groups: HashMap<MaterialId, wgpu::BindGroup>,

    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,

    mesh_render_pipeline: wgpu::RenderPipeline,
}
impl VisualRenderer {
    pub async fn new(target: RenderTarget) -> Self {
        let device = target.device();

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

        let mesh_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Mesh Render Pipeline Layout"),
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

            let mesh_render_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[MeshVertex::desc(), InstanceRaw::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: target.format(),
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

            mesh_render_pipeline
        };

        Self {
            target,

            texture_bind_group_layout,
            depth_texture: OnceCell::new(),

            material_bind_groups: HashMap::new(),

            camera_bind_group,
            camera_buffer,

            light_bind_group,
            light_buffer,

            mesh_render_pipeline,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.target.size()
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 || new_size.1 > 0 {
            self.target.resize(new_size);
            self.depth_texture = OnceCell::new()
        }
    }

    pub fn aspect(&self) -> f64 {
        self.target.aspect()
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), wgpu::SurfaceError> {
        self.build_material_bind_groups(scene);

        let device = self.target.device();
        let queue = self.target.queue();
        let frame = self.target.frame();
        let view = frame.view();

        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera.to_raw(self.aspect())]),
        );

        queue.write_buffer(
            &self.light_buffer,
            0,
            bytemuck::cast_slice(&[scene.light().to_raw()]),
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
                    view: &self.depth_texture().view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.mesh_render_pipeline);

            for object in scene.objects().iter() {
                let mesh = object.mesh();
                render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                render_pass.set_index_buffer(
                    mesh.index_buffer(device).slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                let mbg = self
                    .material_bind_groups
                    .get(&object.material_id())
                    .unwrap();

                render_pass.set_bind_group(
                    0,
                    mbg,
                    //&material_bind_groups.get(&object.material_id()).unwrap(),
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

        queue.submit(std::iter::once(encoder.finish()));
        frame.finish();

        Ok(())
    }

    fn depth_texture(&self) -> &TextureResources {
        self.depth_texture
            .get_or_init(|| TextureResources::depth(self.target.device(), self.target.size()))
    }

    fn build_material_bind_groups(&mut self, scene: &Scene) {
        for (_, material) in scene.materials() {
            self.create_material_bind_group(material)
        }
    }

    fn create_material_bind_group(&mut self, material: &Material) {
        self.material_bind_groups
            .entry(material.id)
            .or_insert_with(|| {
                let device = self.target.device();
                let queue = self.target.queue();

                let diffuse = material.diffuse.resources(device, queue);
                let normal = material.normal.resources(device, queue);

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&normal.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::Sampler(&normal.sampler),
                        },
                    ],
                    label: None,
                })
            });
    }
}
