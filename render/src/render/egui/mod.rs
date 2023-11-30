use egui_wgpu::RenderState;
use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::vertex::Vertex2;

//const RENDER_LABEL: Option<&'static str> = Some("egui-transfer");

const QUAD_VERTS: [Vertex2; 6] = [
    Vertex2 {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex2 {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex2 {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex2 {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex2 {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex2 {
        position: [1.0, -1.0],
        tex_coords: [1.0, 1.0],
    },
];

pub struct EguiTransfer {
    device: Arc<wgpu::Device>,
    pipeline: Arc<wgpu::RenderPipeline>,
    quad_buffer: wgpu::Buffer,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_sampler: wgpu::Sampler,
    texture_bind_group: wgpu::BindGroup,
}
impl EguiTransfer {
    pub fn new(render_state: &RenderState, texture_view: &wgpu::TextureView) -> Self {
        let device = &render_state.device;

        let (texture_bind_group_layout, texture_sampler, texture_bind_group) = {
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("egui-transfer-texture-bind-group-layout"),
                entries: &[
                    // Texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    // Sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let bind_group = Self::make_texture_bind_group(device, &layout, &sampler, texture_view);

            (layout, sampler, bind_group)
        };

        let quad_buffer = {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("egui-transfer-quad-buffer"),
                contents: bytemuck::cast_slice(&QUAD_VERTS),
                usage: wgpu::BufferUsages::VERTEX,
            })
        };

        let pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("egui-transfer-pipeline-layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("egui-transfer-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
            });

            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("egui-transfer-render-pipeline"),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex2::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: render_state.target_format,
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
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

            pipeline
        };

        Self {
            device: device.clone(),
            pipeline: Arc::new(pipeline),
            quad_buffer,
            texture_bind_group_layout,
            texture_sampler,
            texture_bind_group,
        }
    }

    pub fn transfer<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.quad_buffer.slice(..));
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.draw(0..QUAD_VERTS.len() as u32, 0..1);
    }

    pub fn rebind_texture(&mut self, texture_view: &wgpu::TextureView) {
        self.texture_bind_group = Self::make_texture_bind_group(
            &self.device,
            &self.texture_bind_group_layout,
            &self.texture_sampler,
            texture_view,
        );
    }

    fn make_texture_bind_group(
        device: &Arc<wgpu::Device>,
        layout: &wgpu::BindGroupLayout,
        sampler: &wgpu::Sampler,
        texture_view: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("egui-transfer-texture-bind-group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        })
    }
}
