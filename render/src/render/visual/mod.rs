use super::{texture::TextureResources, RenderContext, RenderTarget, VertexBuffer};
use crate::{
    camera::{Camera, CameraRaw},
    light::{AmbientLightRaw, DirectionalLightRaw},
    model::{
        CurveMaterial, CurveMaterialId, CurveVertex, SurfaceMaterial, SurfaceMaterialId,
        SurfaceVertex, TransformedCurveInstanceRaw, TransformedSurfaceInstanceRaw,
    },
    scene::Scene,
    texture::{Texture, TextureId},
    vertex::Vertex2,
};
use bytemuck::{Pod, Zeroable};
use std::{cell::OnceCell, cmp::min, collections::HashMap};
use wgpu::util::DeviceExt;

const MAX_DIRECTIONAL_LIGHTS: u32 = 32;
const DIRECTIONAL_LIGHT_UNIFORM_SIZE: u32 =
    std::mem::size_of::<DirectionalLightRaw>() as u32 * MAX_DIRECTIONAL_LIGHTS;

const MAX_AMBIENT_LIGHTS: u32 = 32;
const AMBIENT_LIGHT_UNIFORM_SIZE: u32 =
    std::mem::size_of::<AmbientLightRaw>() as u32 * MAX_AMBIENT_LIGHTS;

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

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct GlobalsRaw {
    num_directional_lights: u32,
    _padding: [u32; 3],
    num_ambient_lights: u32,
    _padding2: [u32; 3],
    camera: CameraRaw,
}

#[derive(Debug)]
pub struct VisualRenderer {
    output_target: RenderTarget,

    opaque_target: RenderTarget,
    accum_target: RenderTarget,
    transmit_target: RenderTarget,

    compositing_bind_group_layout: wgpu::BindGroupLayout,
    compositing_bind_group: OnceCell<wgpu::BindGroup>,
    quad_buffer: wgpu::Buffer,
    opaque_sampler: wgpu::Sampler,
    accum_sampler: wgpu::Sampler,
    transmit_sampler: wgpu::Sampler,

    surface_texture_bind_group_layout: wgpu::BindGroupLayout,
    curve_texture_bind_group_layout: wgpu::BindGroupLayout,
    depth_texture: OnceCell<TextureResources>,
    image_textures: HashMap<TextureId, TextureResources>,

    surface_material_bind_groups: HashMap<SurfaceMaterialId, wgpu::BindGroup>,
    curve_material_bind_groups: HashMap<CurveMaterialId, wgpu::BindGroup>,

    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,

    light_bind_group_layout: wgpu::BindGroupLayout,
    light_bind_group: OnceCell<wgpu::BindGroup>,
    directional_light_buffer: wgpu::Buffer,
    ambient_light_buffer: wgpu::Buffer,

    opaque_surface_pipeline: wgpu::RenderPipeline,
    opaque_curve_pipeline: wgpu::RenderPipeline,
    translucent_surface_pipeline: wgpu::RenderPipeline,
    compositing_pipeline: wgpu::RenderPipeline,
}
impl VisualRenderer {
    pub fn new(context: &RenderContext, output_target: RenderTarget) -> Self {
        let device = output_target.device();

        let opaque_target = context.render_into_memory(
            output_target.size(),
            wgpu::TextureFormat::Rgb10a2Unorm,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
        );
        let accum_target = context.render_into_memory(
            output_target.size(),
            wgpu::TextureFormat::Rgba16Float,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
        );
        let transmit_target = context.render_into_memory(
            output_target.size(),
            wgpu::TextureFormat::R16Float,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
        );

        let surface_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
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
                    // Emissive texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Emissive sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 5,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Roughness texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 6,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Roughness sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 7,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Metallic texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 8,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Metallic sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 9,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Ambient occlusion texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 10,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Ambient occlusion sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 11,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    // Transmission texture
                    wgpu::BindGroupLayoutEntry {
                        binding: 12,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    // Transmission sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 13,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let curve_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    // Color texture
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
                    // Color sampler
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let (globals_bind_group_layout, globals_bind_group, globals_buffer) = {
            let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[0u8; std::mem::size_of::<GlobalsRaw>()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let globals_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
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
                label: None,
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

        let (light_bind_group_layout, directional_light_buffer, ambient_light_buffer) = {
            let light_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(
                                    DIRECTIONAL_LIGHT_UNIFORM_SIZE as u64,
                                ),
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: wgpu::BufferSize::new(
                                    AMBIENT_LIGHT_UNIFORM_SIZE as u64,
                                ),
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

            let directional_light_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: DIRECTIONAL_LIGHT_UNIFORM_SIZE as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let ambient_light_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: AMBIENT_LIGHT_UNIFORM_SIZE as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            (
                light_bind_group_layout,
                directional_light_buffer,
                ambient_light_buffer,
            )
        };

        let opaque_surface_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &surface_texture_bind_group_layout,
                    &globals_bind_group_layout,
                    &light_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Opaque surface shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let opaque_surface_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Opaque surface pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_surface",
                        buffers: &[SurfaceVertex::desc(), TransformedSurfaceInstanceRaw::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_opaque_surface",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: opaque_target.format(),
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
                        //cull_mode: Some(wgpu::Face::Back),
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

            opaque_surface_pipeline
        };

        let opaque_curve_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&curve_texture_bind_group_layout, &globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Opaque curve shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let opaque_curve_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Opaque curve pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_curve",
                        buffers: &[CurveVertex::desc(), TransformedCurveInstanceRaw::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_opaque_curve",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: opaque_target.format(),
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

            opaque_curve_pipeline
        };

        let translucent_surface_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &surface_texture_bind_group_layout,
                    &globals_bind_group_layout,
                    &light_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Truanslucent surface shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let translucent_surface_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Translucent surface pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_surface",
                        buffers: &[SurfaceVertex::desc(), TransformedSurfaceInstanceRaw::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_translucent_surface",
                        targets: &[
                            Some(wgpu::ColorTargetState {
                                format: accum_target.format(),
                                blend: Some(wgpu::BlendState {
                                    color: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::One,
                                        dst_factor: wgpu::BlendFactor::One,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                    alpha: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::One,
                                        dst_factor: wgpu::BlendFactor::One,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                }),
                                write_mask: wgpu::ColorWrites::all(),
                            }),
                            Some(wgpu::ColorTargetState {
                                format: transmit_target.format(),
                                blend: Some(wgpu::BlendState {
                                    color: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::Zero,
                                        dst_factor: wgpu::BlendFactor::OneMinusSrc,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                    alpha: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::Zero,
                                        dst_factor: wgpu::BlendFactor::Zero,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                }),
                                write_mask: wgpu::ColorWrites::all(),
                            }),
                            Some(wgpu::ColorTargetState {
                                format: opaque_target.format(),
                                blend: Some(wgpu::BlendState {
                                    color: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::Zero,
                                        dst_factor: wgpu::BlendFactor::OneMinusSrc,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                    alpha: wgpu::BlendComponent {
                                        src_factor: wgpu::BlendFactor::Zero,
                                        dst_factor: wgpu::BlendFactor::Zero,
                                        operation: wgpu::BlendOperation::Add,
                                    },
                                }),
                                write_mask: wgpu::ColorWrites::all(),
                            }),
                        ],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        //cull_mode: Some(wgpu::Face::Back),
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

            translucent_surface_pipeline
        };

        let (
            compositing_bind_group_layout,
            opaque_sampler,
            accum_sampler,
            transmit_sampler,
            quad_buffer,
        ) = {
            let compositing_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        // Opaque texture
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
                        // Opaque sampler
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        // Accum texture
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
                        // Accum sampler
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        // Transmit texture
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        // Transmit sampler
                        wgpu::BindGroupLayoutEntry {
                            binding: 5,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

            let opaque_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let accum_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let transmit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let quad_buffer = {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&QUAD_VERTS),
                    usage: wgpu::BufferUsages::VERTEX,
                })
            };

            (
                compositing_bind_group_layout,
                opaque_sampler,
                accum_sampler,
                transmit_sampler,
                quad_buffer,
            )
        };

        let compositing_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Compositing Pipeline Layout"),
                bind_group_layouts: &[&compositing_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Compositing shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let compositing_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Compositing pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_composite",
                        buffers: &[Vertex2::desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_composite",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: output_target.format(),
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
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

            compositing_pipeline
        };

        Self {
            output_target,

            opaque_target,
            accum_target,
            transmit_target,

            compositing_bind_group_layout,
            compositing_bind_group: OnceCell::new(),
            quad_buffer,
            opaque_sampler,
            accum_sampler,
            transmit_sampler,

            depth_texture: OnceCell::new(),

            surface_texture_bind_group_layout,
            curve_texture_bind_group_layout,
            image_textures: HashMap::new(),

            surface_material_bind_groups: HashMap::new(),
            curve_material_bind_groups: HashMap::new(),

            globals_bind_group,
            globals_buffer,

            light_bind_group_layout,
            light_bind_group: OnceCell::new(),
            directional_light_buffer,
            ambient_light_buffer,

            opaque_surface_pipeline,
            opaque_curve_pipeline,
            translucent_surface_pipeline,
            compositing_pipeline,
        }
    }

    fn globals_raw(&self, scene: &Scene, camera: &Camera) -> GlobalsRaw {
        GlobalsRaw {
            num_directional_lights: min(
                scene.directional_lights().len() as u32,
                MAX_DIRECTIONAL_LIGHTS,
            ),
            _padding: [0; 3],
            num_ambient_lights: min(scene.ambient_lights().len() as u32, MAX_AMBIENT_LIGHTS),
            _padding2: [0; 3],
            camera: camera.to_raw(self.aspect()),
        }
    }

    pub fn target(&self) -> &RenderTarget {
        &self.output_target
    }

    pub fn size(&self) -> (u32, u32) {
        self.output_target.size()
    }

    pub fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 || new_size.1 > 0 {
            self.output_target.resize(new_size);
            self.opaque_target.resize(new_size);
            self.accum_target.resize(new_size);
            self.transmit_target.resize(new_size);
            self.compositing_bind_group = OnceCell::new();
            self.depth_texture = OnceCell::new()
        }
    }

    pub fn aspect(&self) -> f64 {
        self.output_target.aspect()
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), wgpu::SurfaceError> {
        self.build_image_texture_resources(scene);
        self.build_material_bind_groups(scene);

        let device = self.output_target.device();
        let queue = self.output_target.queue();

        queue.write_buffer(
            &self.globals_buffer,
            0,
            bytemuck::cast_slice(&[self.globals_raw(scene, camera)]),
        );

        for (i, light) in scene.directional_lights().iter().enumerate() {
            queue.write_buffer(
                &self.directional_light_buffer,
                (i * std::mem::size_of::<DirectionalLightRaw>()) as wgpu::BufferAddress,
                bytemuck::bytes_of(&light.to_raw()),
            )
        }

        for (i, light) in scene.ambient_lights().iter().enumerate() {
            queue.write_buffer(
                &self.ambient_light_buffer,
                (i * std::mem::size_of::<AmbientLightRaw>()) as wgpu::BufferAddress,
                bytemuck::bytes_of(&light.to_raw()),
            )
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Visual render encoder"),
        });

        let accum_frame = self.accum_target.frame();
        let accum_view = accum_frame.view();

        let transmit_frame = self.transmit_target.frame();
        let transmit_view = transmit_frame.view();

        let opaque_frame = self.opaque_target.frame();
        let opaque_view = opaque_frame.view();

        let output_frame = self.output_target.frame();
        let output_view = output_frame.view();

        // Render and composite scene
        {
            // Render opaque geometry
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Opaque Render Pass"),
                    color_attachments: &[
                        // Opaque
                        Some(wgpu::RenderPassColorAttachment {
                            view: &opaque_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.3,
                                    g: 0.4,
                                    b: 0.5,
                                    a: 1.0,
                                }),
                                store: true,
                            },
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture().view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                });

                // Render opaque surfaces
                {
                    render_pass.set_pipeline(&self.opaque_surface_pipeline);

                    render_pass.set_bind_group(1, &self.globals_bind_group, &[]);
                    render_pass.set_bind_group(2, &self.light_bind_group(), &[]);

                    for object in scene.surfaces().iter() {
                        let material = scene
                            .surface_materials()
                            .get(&object.material_id())
                            .unwrap();
                        if material.is_translucent {
                            continue;
                        }

                        let mesh = object.mesh();

                        render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                        render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                        render_pass.set_index_buffer(
                            mesh.index_buffer(device).slice(..),
                            wgpu::IndexFormat::Uint32,
                        );

                        render_pass.set_bind_group(
                            0,
                            self.surface_material_bind_groups
                                .get(&object.material_id())
                                .unwrap(),
                            &[],
                        );

                        render_pass.draw_indexed(
                            0..mesh.num_elements(),
                            0,
                            0..object.num_instances(),
                        );
                    }
                }

                // Render opaque curves
                {
                    render_pass.set_pipeline(&self.opaque_curve_pipeline);

                    render_pass.set_bind_group(1, &self.globals_bind_group, &[]);

                    for object in scene.curves().iter() {
                        let mesh = object.mesh();

                        render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                        render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                        render_pass.set_index_buffer(
                            mesh.index_buffer(device).slice(..),
                            wgpu::IndexFormat::Uint32,
                        );

                        render_pass.set_bind_group(
                            0,
                            self.curve_material_bind_groups
                                .get(&object.material_id())
                                .unwrap(),
                            &[],
                        );

                        render_pass.draw_indexed(
                            0..mesh.num_elements(),
                            0,
                            0..object.num_instances(),
                        );
                    }
                }
            }

            // Render translucent geometry
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Translucent Render Pass"),
                    color_attachments: &[
                        // Accum
                        Some(wgpu::RenderPassColorAttachment {
                            view: &accum_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.0,
                                }),
                                store: true,
                            },
                        }),
                        // Transmit
                        Some(wgpu::RenderPassColorAttachment {
                            view: &transmit_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 1.0,
                                    g: 0.0,
                                    b: 0.0,
                                    a: 0.0,
                                }),
                                store: true,
                            },
                        }),
                        // Modulate the background (opaque) target
                        Some(wgpu::RenderPassColorAttachment {
                            view: &opaque_view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: true,
                            },
                        }),
                    ],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture().view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: false,
                        }),
                        stencil_ops: None,
                    }),
                });

                // Render translucent surfaces
                {
                    render_pass.set_pipeline(&self.translucent_surface_pipeline);

                    render_pass.set_bind_group(1, &self.globals_bind_group, &[]);
                    render_pass.set_bind_group(2, &self.light_bind_group(), &[]);

                    for object in scene.surfaces().iter() {
                        let material = scene
                            .surface_materials()
                            .get(&object.material_id())
                            .unwrap();
                        if !material.is_translucent {
                            continue;
                        }

                        let mesh = object.mesh();

                        render_pass.set_vertex_buffer(0, mesh.vertex_buffer(device).slice(..));
                        render_pass.set_vertex_buffer(1, object.instance_buffer(device).slice(..));
                        render_pass.set_index_buffer(
                            mesh.index_buffer(device).slice(..),
                            wgpu::IndexFormat::Uint32,
                        );

                        render_pass.set_bind_group(
                            0,
                            self.surface_material_bind_groups
                                .get(&object.material_id())
                                .unwrap(),
                            &[],
                        );

                        render_pass.draw_indexed(
                            0..mesh.num_elements(),
                            0,
                            0..object.num_instances(),
                        );
                    }
                }
            }

            // Composite
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Compositing Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &output_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });

                render_pass.set_pipeline(&self.compositing_pipeline);
                render_pass.set_vertex_buffer(0, self.quad_buffer.slice(..));
                render_pass.set_bind_group(
                    0,
                    &self.compositing_bind_group(opaque_view, accum_view, transmit_view),
                    &[],
                );
                render_pass.draw(0..QUAD_VERTS.len() as u32, 0..1);
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        output_frame.finish();

        Ok(())
    }

    fn depth_texture(&self) -> &TextureResources {
        self.depth_texture.get_or_init(|| {
            TextureResources::depth(self.output_target.device(), self.output_target.size())
        })
    }

    fn build_image_texture_resources(&mut self, scene: &Scene) {
        for (_, texture) in scene.textures() {
            self.create_image_texture_resources(texture);
        }
    }

    fn create_image_texture_resources(&mut self, texture: &Texture) {
        self.image_textures.entry(texture.id).or_insert_with(|| {
            TextureResources::image(
                &texture.image,
                self.output_target.device(),
                self.output_target.queue(),
            )
        });
    }

    fn build_material_bind_groups(&mut self, scene: &Scene) {
        for material in scene.surface_materials().values() {
            self.create_surface_material_bind_groups(material);
        }

        for material in scene.curve_materials().values() {
            self.create_curve_material_bind_groups(material);
        }
    }

    fn create_surface_material_bind_groups(&mut self, material: &SurfaceMaterial) {
        self.surface_material_bind_groups
            .entry(material.id)
            .or_insert_with(|| {
                let device = self.output_target.device();

                let diffuse = self.image_textures.get(&material.diffuse).unwrap();
                let normal = self.image_textures.get(&material.normal).unwrap();
                let emissive = self.image_textures.get(&material.emissive).unwrap();
                let roughness = self.image_textures.get(&material.roughness).unwrap();
                let metallic = self.image_textures.get(&material.metallic).unwrap();
                let ambient = self.image_textures.get(&material.ambient).unwrap();
                let transmit = self.image_textures.get(&material.transmit).unwrap();

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.surface_texture_bind_group_layout,
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
                        wgpu::BindGroupEntry {
                            binding: 4,
                            resource: wgpu::BindingResource::TextureView(&emissive.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 5,
                            resource: wgpu::BindingResource::Sampler(&emissive.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 6,
                            resource: wgpu::BindingResource::TextureView(&roughness.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 7,
                            resource: wgpu::BindingResource::Sampler(&roughness.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 8,
                            resource: wgpu::BindingResource::TextureView(&metallic.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 9,
                            resource: wgpu::BindingResource::Sampler(&metallic.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 10,
                            resource: wgpu::BindingResource::TextureView(&ambient.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 11,
                            resource: wgpu::BindingResource::Sampler(&ambient.sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 12,
                            resource: wgpu::BindingResource::TextureView(&transmit.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 13,
                            resource: wgpu::BindingResource::Sampler(&transmit.sampler),
                        },
                    ],
                    label: None,
                })
            });
    }

    fn create_curve_material_bind_groups(&mut self, material: &CurveMaterial) {
        self.curve_material_bind_groups
            .entry(material.id)
            .or_insert_with(|| {
                let device = self.output_target.device();

                let color = self.image_textures.get(&material.color).unwrap();

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &self.curve_texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&color.view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&color.sampler),
                        },
                    ],
                    label: None,
                })
            });
    }

    fn compositing_bind_group(
        &self,
        opaque_view: &wgpu::TextureView,
        accum_view: &wgpu::TextureView,
        transmit_view: &wgpu::TextureView,
    ) -> &wgpu::BindGroup {
        self.compositing_bind_group.get_or_init(|| {
            let device = self.output_target.device();

            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.compositing_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(opaque_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.opaque_sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(accum_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Sampler(&self.accum_sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(transmit_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: wgpu::BindingResource::Sampler(&self.transmit_sampler),
                    },
                ],
            })
        })
    }

    fn light_bind_group(&self) -> &wgpu::BindGroup {
        self.light_bind_group.get_or_init(|| {
            let device = self.output_target.device();

            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.light_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.directional_light_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.ambient_light_buffer.as_entire_binding(),
                    },
                ],
            })
        })
    }
}
