use super::{
    texture::TextureResources, GlobalsRaw, RenderContext, RenderTarget, VertexBuffer,
    MAX_AMBIENT_LIGHTS, MAX_DIRECTIONAL_LIGHTS,
};
use crate::{
    camera::Camera,
    light::{AmbientLightRaw, DirectionalLight, DirectionalLightRaw},
    model::{
        CurveMaterial, CurveMaterialId, CurveVertexRaw, ModelInstanceRaw, PointMaterial,
        PointMaterialId, PointVertexRaw, SurfaceMaterial, SurfaceMaterialId, SurfaceVertexRaw,
    },
    scene::Scene,
    texture::{Texture, TextureId},
    vertex::Vertex2,
};
use std::{cell::OnceCell, collections::HashMap};
use wgpu::util::DeviceExt;

const DIRECTIONAL_LIGHT_UNIFORM_SIZE: u32 =
    std::mem::size_of::<DirectionalLightRaw>() as u32 * MAX_DIRECTIONAL_LIGHTS;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MsaaSamples {
    Samples1,
    Samples2,
    Samples4,
    Samples8,
    Samples16,
}
impl MsaaSamples {
    pub fn max_from_flags(flags: wgpu::TextureFormatFeatureFlags) -> Self {
        if flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X16) {
            Self::Samples16
        } else if flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X8) {
            Self::Samples8
        } else if flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X4) {
            Self::Samples4
        } else if flags.contains(wgpu::TextureFormatFeatureFlags::MULTISAMPLE_X2) {
            Self::Samples2
        } else {
            Self::Samples1
        }
    }

    pub fn samples(&self) -> u32 {
        match self {
            MsaaSamples::Samples1 => 1,
            MsaaSamples::Samples2 => 2,
            MsaaSamples::Samples4 => 4,
            MsaaSamples::Samples8 => 8,
            MsaaSamples::Samples16 => 16,
        }
    }

    pub fn is_multisampled(&self) -> bool {
        match self {
            MsaaSamples::Samples1 => false,
            _ => true,
        }
    }
}
impl PartialOrd for MsaaSamples {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.samples().partial_cmp(&other.samples())
    }
}
impl Ord for MsaaSamples {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.samples().cmp(&other.samples())
    }
}

#[derive(Debug)]
pub struct VisualRenderer {
    pixels_per_point: f32,
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
    point_texture_bind_group_layout: wgpu::BindGroupLayout,
    depth_texture: OnceCell<TextureResources>,
    image_textures: HashMap<TextureId, TextureResources>,

    surface_material_bind_groups: HashMap<SurfaceMaterialId, wgpu::BindGroup>,
    curve_material_bind_groups: HashMap<CurveMaterialId, wgpu::BindGroup>,
    point_material_bind_groups: HashMap<PointMaterialId, wgpu::BindGroup>,

    globals_buffer: wgpu::Buffer,
    globals_bind_group: wgpu::BindGroup,

    light_bind_group_layout: wgpu::BindGroupLayout,
    light_bind_group: OnceCell<wgpu::BindGroup>,
    directional_light_buffer: wgpu::Buffer,
    ambient_light_buffer: wgpu::Buffer,

    opaque_surface_pipeline: wgpu::RenderPipeline,
    opaque_curve_pipeline: wgpu::RenderPipeline,
    opaque_point_pipeline: wgpu::RenderPipeline,
    translucent_surface_pipeline: wgpu::RenderPipeline,
    compositing_pipeline: wgpu::RenderPipeline,

    msaa_samples: MsaaSamples,
    msaa_target: Option<RenderTarget>,
}
impl VisualRenderer {
    pub fn new(
        context: &RenderContext,
        output_target: RenderTarget,
        msaa_samples: MsaaSamples,
        pixels_per_point: f32,
    ) -> Self {
        let device = output_target.device();

        let max_msaa_samples = MsaaSamples::max_from_flags(
            output_target
                .adapter()
                .get_texture_format_features(output_target.format())
                .flags,
        );

        let msaa_samples = msaa_samples.min(max_msaa_samples);

        let msaa_target = if msaa_samples.is_multisampled() {
            Some(context.render_into_memory(
                output_target.size(),
                output_target.format(),
                Some(wgpu::TextureUsages::TEXTURE_BINDING),
                msaa_samples,
            ))
        } else {
            None
        };

        let opaque_target = context.render_into_memory(
            output_target.size(),
            wgpu::TextureFormat::Rgb10a2Unorm,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
            msaa_samples,
        );

        let accum_target = context.render_into_memory(
            output_target.size(),
            wgpu::TextureFormat::Rgba16Float,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
            msaa_samples,
        );

        let transmit_target = context.render_into_memory(
            output_target.size(),
            wgpu::TextureFormat::R16Float,
            Some(wgpu::TextureUsages::TEXTURE_BINDING),
            msaa_samples,
        );

        let surface_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("surface-texture-bind-group-layout"),
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
                label: Some("curve-texture-bind-group-layout"),
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

        let point_texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("point-texture-bind-group-layout"),
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

        let (light_bind_group_layout, directional_light_buffer, ambient_light_buffer) = {
            let light_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("light-bind-group-layout"),
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
                });

            let directional_light_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("directional-light-buffer"),
                size: DIRECTIONAL_LIGHT_UNIFORM_SIZE as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let ambient_light_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("ambient-light-buffer"),
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
                label: Some("opaque-surface-pipeline-layout"),
                bind_group_layouts: &[
                    &surface_texture_bind_group_layout,
                    &globals_bind_group_layout,
                    &light_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("opaque-surface-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let opaque_surface_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("opaque-surface-pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_surface",
                        buffers: &[SurfaceVertexRaw::desc(), ModelInstanceRaw::surface_desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_opaque_surface",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: opaque_target.format(),
                            blend: Some(wgpu::BlendState::REPLACE),
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
                        count: msaa_samples.samples(),
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

            opaque_surface_pipeline
        };

        let opaque_curve_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("opaque-curve-pipeline-layout"),
                bind_group_layouts: &[&curve_texture_bind_group_layout, &globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("opaque-curve-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let opaque_curve_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("opaque-curve-pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_curve",
                        buffers: &[CurveVertexRaw::desc(), ModelInstanceRaw::curve_desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_opaque_curve",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: opaque_target.format(),
                            blend: Some(wgpu::BlendState::REPLACE),
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
                        count: msaa_samples.samples(),
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

            opaque_curve_pipeline
        };

        let opaque_point_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("opaque-point-pipeline-layout"),
                bind_group_layouts: &[&point_texture_bind_group_layout, &globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("opaque-point-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let opaque_point_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("opaque-point-pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_point",
                        buffers: &[PointVertexRaw::desc(), ModelInstanceRaw::point_desc()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_opaque_point",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: opaque_target.format(),
                            // Unlike the other opaque pipelines, this one uses
                            // alpha blending so we can feather the edges for cheap
                            // anti-aliasing.
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
                        count: msaa_samples.samples(),
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

            opaque_point_pipeline
        };

        let translucent_surface_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("translucent-surface-pipeline-layout"),
                bind_group_layouts: &[
                    &surface_texture_bind_group_layout,
                    &globals_bind_group_layout,
                    &light_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("translucent-surface-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let translucent_surface_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("translucent-surface-pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_surface",
                        buffers: &[SurfaceVertexRaw::desc(), ModelInstanceRaw::surface_desc()],
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
                        count: msaa_samples.samples(),
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
            let multisampled = msaa_samples.is_multisampled();
            let filterable = !multisampled;
            let sampler_ty = wgpu::BindingType::Sampler(match multisampled {
                true => wgpu::SamplerBindingType::NonFiltering,
                false => wgpu::SamplerBindingType::Filtering,
            });

            let compositing_bind_group_layout =
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("compositing-bind-group-layout"),
                    entries: &[
                        // Opaque texture
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled,
                                sample_type: wgpu::TextureSampleType::Float { filterable },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        // Opaque sampler
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: sampler_ty,
                            count: None,
                        },
                        // Accum texture
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled,
                                sample_type: wgpu::TextureSampleType::Float { filterable },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        // Accum sampler
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: sampler_ty,
                            count: None,
                        },
                        // Transmit texture
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled,
                                sample_type: wgpu::TextureSampleType::Float { filterable },
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        // Transmit sampler
                        wgpu::BindGroupLayoutEntry {
                            binding: 5,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: sampler_ty,
                            count: None,
                        },
                    ],
                });

            let opaque_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let accum_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let transmit_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            });

            let quad_buffer = {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("quad-buffer"),
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
                label: Some("compositing-pipeline-layout"),
                bind_group_layouts: &[&compositing_bind_group_layout, &globals_bind_group_layout],
                push_constant_ranges: &[],
            });

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("compositing-shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

            let format = match &msaa_target {
                Some(target) => target.format(),
                None => output_target.format(),
            };

            let compositing_pipeline =
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("compositing-pipeline"),
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
                            format,
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
                        count: msaa_samples.samples(),
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

            compositing_pipeline
        };

        Self {
            pixels_per_point,
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
            point_texture_bind_group_layout,
            image_textures: HashMap::new(),

            surface_material_bind_groups: HashMap::new(),
            curve_material_bind_groups: HashMap::new(),
            point_material_bind_groups: HashMap::new(),

            globals_bind_group,
            globals_buffer,

            light_bind_group_layout,
            light_bind_group: OnceCell::new(),
            directional_light_buffer,
            ambient_light_buffer,

            opaque_surface_pipeline,
            opaque_curve_pipeline,
            opaque_point_pipeline,
            translucent_surface_pipeline,
            compositing_pipeline,

            msaa_samples,
            msaa_target,
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
            self.output_target.resize(new_size, MsaaSamples::Samples1);
            self.opaque_target.resize(new_size, self.msaa_samples);
            self.accum_target.resize(new_size, self.msaa_samples);
            self.transmit_target.resize(new_size, self.msaa_samples);
            if let Some(ref mut msaa_target) = self.msaa_target {
                msaa_target.resize(new_size, self.msaa_samples);
            }
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
            //bytemuck::cast_slice(&[self.globals_raw(scene, camera)]),
            bytemuck::cast_slice(&[GlobalsRaw::build(
                scene,
                camera,
                self.aspect(),
                self.size(),
                self.pixels_per_point,
            )]),
        );

        let view_matrix = camera.view_rotation_matrix().transpose();

        for (i, (light, is_camera)) in scene
            .world_directional_lights()
            .iter()
            .map(|l| (l, false))
            .chain(scene.camera_directional_lights().iter().map(|l| (l, true)))
            .enumerate()
        {
            queue.write_buffer(
                &self.directional_light_buffer,
                (i * std::mem::size_of::<DirectionalLightRaw>()) as wgpu::BufferAddress,
                bytemuck::bytes_of(&match is_camera {
                    true => DirectionalLight {
                        direction: light
                            .direction
                            .into_point()
                            .transform(view_matrix)
                            .into_vec(),
                        color: light.color.clone(),
                    }
                    .to_raw(),
                    false => light.to_raw(),
                }),
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
            label: Some("visual-render-encoder"),
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
                    label: Some("opaque-render-pass"),
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

                    for (_model_id, model) in scene.models().iter() {
                        for (_surface_id, surface) in model.surfaces().iter() {
                            let material = scene
                                .materials()
                                .surface()
                                .get(&surface.material_id())
                                .unwrap();
                            if material.is_translucent {
                                continue;
                            }

                            render_pass
                                .set_vertex_buffer(0, surface.vertex_buffer(device).slice(..));
                            render_pass
                                .set_vertex_buffer(1, model.instance_buffer(device).slice(..));
                            render_pass.set_index_buffer(
                                surface.index_buffer(device).slice(..),
                                wgpu::IndexFormat::Uint32,
                            );

                            render_pass.set_bind_group(
                                0,
                                self.surface_material_bind_groups
                                    .get(&surface.material_id())
                                    .unwrap(),
                                &[],
                            );

                            render_pass.draw_indexed(
                                0..surface.num_elements(),
                                0,
                                0..model.num_instances(),
                            );
                        }
                    }
                }

                // Render opaque curves
                {
                    render_pass.set_pipeline(&self.opaque_curve_pipeline);

                    render_pass.set_bind_group(1, &self.globals_bind_group, &[]);

                    for (_model_id, model) in scene.models().iter() {
                        println!("curves.len() = {}", model.curves().len());
                        println!("model.num_instances() = {}", model.num_instances());
                        for (curve_id, curve) in model.curves().iter() {
                            render_pass.set_vertex_buffer(
                                0,
                                curve.vertex_buffer(curve_id, device).slice(..),
                            );
                            render_pass
                                .set_vertex_buffer(1, model.instance_buffer(device).slice(..));
                            render_pass.set_index_buffer(
                                curve.index_buffer(device).slice(..),
                                wgpu::IndexFormat::Uint32,
                            );

                            render_pass.set_bind_group(
                                0,
                                self.curve_material_bind_groups
                                    .get(&curve.material_id())
                                    .unwrap(),
                                &[],
                            );

                            render_pass.draw_indexed(
                                0..curve.num_elements(),
                                0,
                                0..model.num_instances(),
                            );
                        }
                    }
                }

                // Render opaque points
                {
                    render_pass.set_pipeline(&self.opaque_point_pipeline);

                    render_pass.set_bind_group(1, &self.globals_bind_group, &[]);

                    for (_model_id, model) in scene.models().iter() {
                        for (_point_id, point) in model.points().iter() {
                            render_pass.set_vertex_buffer(0, point.vertex_buffer(device).slice(..));
                            render_pass
                                .set_vertex_buffer(1, model.instance_buffer(device).slice(..));
                            render_pass.set_index_buffer(
                                point.index_buffer(device).slice(..),
                                wgpu::IndexFormat::Uint32,
                            );

                            render_pass.set_bind_group(
                                0,
                                self.point_material_bind_groups
                                    .get(&point.material_id())
                                    .unwrap(),
                                &[],
                            );

                            render_pass.draw_indexed(
                                0..point.num_elements(),
                                0,
                                0..model.num_instances(),
                            );
                        }
                    }
                }
            }

            // Render translucent geometry
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("translucent-render-pass"),
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

                    for (_model_id, model) in scene.models().iter() {
                        for (_surface_id, surface) in model.surfaces().iter() {
                            let material = scene
                                .materials()
                                .surface()
                                .get(&surface.material_id())
                                .unwrap();
                            if !material.is_translucent {
                                continue;
                            }

                            render_pass
                                .set_vertex_buffer(0, surface.vertex_buffer(device).slice(..));
                            render_pass
                                .set_vertex_buffer(1, model.instance_buffer(device).slice(..));
                            render_pass.set_index_buffer(
                                surface.index_buffer(device).slice(..),
                                wgpu::IndexFormat::Uint32,
                            );

                            render_pass.set_bind_group(
                                0,
                                self.surface_material_bind_groups
                                    .get(&surface.material_id())
                                    .unwrap(),
                                &[],
                            );

                            render_pass.draw_indexed(
                                0..surface.num_elements(),
                                0,
                                0..model.num_instances(),
                            );
                        }
                    }
                }
            }

            // Composite
            {
                let (view, resolve_target) = match &self.msaa_target {
                    Some(target) => (target.texture_view().unwrap(), Some(output_view)),
                    None => (output_view, None),
                };

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("compositing-render-pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target,
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
                render_pass.set_bind_group(1, &self.globals_bind_group, &[]);
                render_pass.draw(0..QUAD_VERTS.len() as u32, 0..1);
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
        output_frame.finish();

        Ok(())
    }

    fn depth_texture(&self) -> &TextureResources {
        self.depth_texture.get_or_init(|| {
            TextureResources::depth(
                self.output_target.device(),
                self.output_target.size(),
                self.msaa_samples,
            )
        })
    }

    fn build_image_texture_resources(&mut self, scene: &Scene) {
        for (_, texture) in scene.materials().textures() {
            self.create_image_texture_resources(texture);
        }
    }

    fn create_image_texture_resources(&mut self, texture: &Texture) {
        self.image_textures.entry(texture.id).or_insert_with(|| {
            TextureResources::image(
                &texture.image,
                self.output_target.device(),
                self.output_target.queue(),
                MsaaSamples::Samples1,
            )
        });
    }

    fn build_material_bind_groups(&mut self, scene: &Scene) {
        for material in scene.materials().surface().values() {
            self.create_surface_material_bind_groups(material);
        }

        for material in scene.materials().curve().values() {
            self.create_curve_material_bind_groups(material);
        }

        for material in scene.materials().point().values() {
            self.create_point_material_bind_groups(material);
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
                    label: Some("surface-material-bind-group"),
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
                    label: Some("curve-material-bind-group"),
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
                })
            });
    }

    fn create_point_material_bind_groups(&mut self, material: &PointMaterial) {
        self.point_material_bind_groups
            .entry(material.id)
            .or_insert_with(|| {
                let device = self.output_target.device();

                let color = self.image_textures.get(&material.color).unwrap();

                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("point-material-bind-group"),
                    layout: &self.point_texture_bind_group_layout,
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
                label: Some("compositing-bind-group"),
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
                label: Some("light-bind-group"),
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
