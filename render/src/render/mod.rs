mod object;
mod position;
mod texture;
mod visual;

#[cfg(feature = "egui")]
mod egui;

use bytemuck::{Pod, Zeroable};
use std::{cmp::min, sync::Arc};
use wgpu::Surface;

pub use object::*;
pub use position::*;
pub use visual::*;

#[cfg(feature = "egui")]
pub use egui::*;

use crate::{
    camera::{Camera, CameraRaw},
    scene::Scene,
};

const MAX_DIRECTIONAL_LIGHTS: u32 = 32;
const MAX_AMBIENT_LIGHTS: u32 = 32;

pub trait VertexBuffer: Pod + Zeroable {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub fn pad_u32(num: u32, pad: u32) -> u32 {
    num + (pad - num % pad)
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct GlobalsRaw {
    num_directional_lights: u32,
    _padding1: [u32; 3],
    num_ambient_lights: u32,
    _padding2: [u32; 3],
    viewport_dims: [f32; 2],
    _padding3: [u32; 2],
    pixels_per_point: f32,
    _padding4: [u32; 3],
    camera: CameraRaw,
}
impl GlobalsRaw {
    fn build(
        scene: &Scene,
        camera: &Camera,
        aspect: f64,
        size: (u32, u32),
        pixels_per_point: f32,
    ) -> GlobalsRaw {
        GlobalsRaw {
            num_directional_lights: min(
                (scene.world_directional_lights().len() + scene.camera_directional_lights().len())
                    as u32,
                MAX_DIRECTIONAL_LIGHTS,
            ),
            _padding1: [0; 3],
            num_ambient_lights: min(scene.ambient_lights().len() as u32, MAX_AMBIENT_LIGHTS),
            _padding2: [0; 3],
            viewport_dims: [size.0 as f32, size.1 as f32],
            _padding3: [0; 2],
            pixels_per_point,
            _padding4: [0; 3],
            camera: camera.to_raw(aspect),
        }
    }
}

pub struct RenderContext {
    inner: Arc<ContextInner>,

    #[cfg(feature = "winit")]
    instance: wgpu::Instance,
}
impl RenderContext {
    #[cfg(not(feature = "winit"))]
    pub fn from_resources(
        adapter: Arc<wgpu::Adapter>,
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
    ) -> Self {
        Self {
            inner: Arc::new(ContextInner {
                adapter,
                device,
                queue,
            }),
        }
    }

    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL, //wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });

        log::debug!("instance {:#?}", instance);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
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

        Self {
            inner: Arc::new(ContextInner {
                adapter: Arc::new(adapter),
                device: Arc::new(device),
                queue: Arc::new(queue),
            }),

            #[cfg(feature = "winit")]
            instance,
        }
    }

    pub fn render_into_memory(
        &self,
        size: (u32, u32),
        format: wgpu::TextureFormat,
        usage: Option<wgpu::TextureUsages>,
        samples: MsaaSamples,
    ) -> RenderTarget {
        RenderTarget {
            context: self.inner.clone(),
            target: TargetInner::Texture(TargetTexture::new(
                &self.inner.device,
                size,
                format,
                usage,
                samples,
            )),
        }
    }

    #[cfg(feature = "winit")]
    pub fn render_on_window(&self, window: &winit::window::Window) -> RenderTarget {
        let surface = unsafe { self.instance.create_surface(&window) }.unwrap();
        let dimensions = window.inner_size();
        self.render_on_surface(surface, (dimensions.width, dimensions.height))
    }

    pub fn render_on_surface(&self, surface: Surface, size: (u32, u32)) -> RenderTarget {
        let surface_caps = surface.get_capabilities(&self.inner.adapter);

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

        surface.configure(&self.inner.device, &config);

        RenderTarget {
            context: self.inner.clone(),
            target: TargetInner::Surface(TargetSurface { surface, config }),
        }
    }
}

#[derive(Debug)]
struct ContextInner {
    //instance: wgpu::Instance,
    adapter: Arc<wgpu::Adapter>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

#[derive(Debug)]
enum TargetInner {
    Surface(TargetSurface),
    Texture(TargetTexture),
}

#[derive(Debug)]
struct TargetSurface {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
}

#[derive(Debug)]
struct TargetTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
}
impl TargetTexture {
    pub fn new(
        device: &wgpu::Device,
        size: (u32, u32),
        format: wgpu::TextureFormat,
        usage: Option<wgpu::TextureUsages>,
        msaa_samples: MsaaSamples,
    ) -> Self {
        let usage = match usage {
            Some(addl_usage) => {
                wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT | addl_usage
            }
            None => wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        };

        let desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: msaa_samples.samples(),
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            label: None,
            view_formats: &[],
        };

        let texture = device.create_texture(&desc);
        let view = texture.create_view(&Default::default());

        TargetTexture { texture, view }
    }

    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, buffer: &wgpu::Buffer) {
        let bytes_per_row = pad_u32(
            self.texture.format().block_size(None).unwrap() * self.texture.size().width,
            256,
        );

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(self.texture.size().height),
                },
            },
            self.texture.size(),
        );
    }
}

#[derive(Debug)]
pub struct RenderTarget {
    context: Arc<ContextInner>,
    target: TargetInner,
}
impl RenderTarget {
    pub fn device(&self) -> &wgpu::Device {
        &self.context.device
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.context.adapter
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.context.queue
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        match &self.target {
            TargetInner::Surface(target) => target.config.format,
            TargetInner::Texture(target) => target.texture.format(),
        }
    }

    pub fn texture_view(&self) -> Option<&wgpu::TextureView> {
        match self.target {
            TargetInner::Surface(_) => None,
            TargetInner::Texture(ref target) => Some(&target.view),
        }
    }

    pub fn resize(&mut self, size: (u32, u32), samples: MsaaSamples) {
        if size.0 == 0 || size.1 == 0 {
            return;
        }

        if let TargetInner::Surface(ref mut target) = self.target {
            target.config.width = size.0;
            target.config.height = size.1;
            target
                .surface
                .configure(&self.context.device, &target.config);
        } else if let TargetInner::Texture(target) = &self.target {
            self.target = TargetInner::Texture(TargetTexture::new(
                self.device(),
                size,
                target.texture.format(),
                Some(target.texture.usage()),
                samples,
            ));
        } else {
            todo!("Resize not yet implemented for target type");
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match &self.target {
            TargetInner::Surface(target) => (target.config.width, target.config.height),
            TargetInner::Texture(target) => {
                (target.texture.size().width, target.texture.size().height)
            }
        }
    }

    pub fn aspect(&self) -> f64 {
        let (w, h) = self.size();
        w as f64 / h as f64
    }

    pub fn frame(&self) -> RenderFrame {
        match &self.target {
            TargetInner::Surface(target) => {
                let surface_texture = target.surface.get_current_texture().unwrap();
                let view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                RenderFrame::new(view, Some(surface_texture))
            }
            TargetInner::Texture(target) => RenderFrame::new(
                target
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
                None,
            ),
        }
    }

    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, buffer: &wgpu::Buffer) {
        match &self.target {
            TargetInner::Surface(_) => todo!(),
            TargetInner::Texture(target) => {
                target.copy_to_buffer(encoder, buffer);
            }
        }
    }
}

pub struct RenderFrame {
    view: wgpu::TextureView,
    surface_texture: Option<wgpu::SurfaceTexture>,
}
impl RenderFrame {
    fn new(view: wgpu::TextureView, surface_texture: Option<wgpu::SurfaceTexture>) -> Self {
        Self {
            view,
            surface_texture,
        }
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn finish(self) {
        if let Some(surface_texture) = self.surface_texture {
            surface_texture.present();
        }
    }
}
