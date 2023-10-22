mod position;
mod texture;
mod visual;

use bytemuck::{Pod, Zeroable};
use std::sync::Arc;
use winit::window::Window;

pub use position::*;
pub use visual::*;

pub trait VertexBuffer: Pod + Zeroable {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub struct RenderContext {
    inner: Arc<ContextInner>,
}
impl RenderContext {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

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
                instance,
                adapter,
                device,
                queue,
            }),
        }
    }

    pub fn on_window(&self, window: &Window) -> RenderTarget {
        let surface = unsafe { self.inner.instance.create_surface(&window) }.unwrap();

        let surface_caps = surface.get_capabilities(&self.inner.adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let dimensions = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: dimensions.width,
            height: dimensions.height,
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

struct ContextInner {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

enum TargetInner {
    Surface(TargetSurface),
    Texture(wgpu::Texture),
}

struct TargetSurface {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
}

pub struct RenderTarget {
    context: Arc<ContextInner>,
    target: TargetInner,
}
impl RenderTarget {
    pub fn device(&self) -> &wgpu::Device {
        &self.context.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.context.queue
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        match &self.target {
            TargetInner::Surface(surface) => surface.config.format,
            TargetInner::Texture(texture) => texture.format(),
        }
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        if size.0 == 0 || size.1 == 0 {
            return;
        }

        match self.target {
            TargetInner::Surface(ref mut surface) => {
                surface.config.width = size.0;
                surface.config.height = size.1;
                surface
                    .surface
                    .configure(&self.context.device, &surface.config)
            }
            TargetInner::Texture(ref mut texture) => todo!(),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        match &self.target {
            TargetInner::Surface(surface) => (surface.config.width, surface.config.height),
            TargetInner::Texture(texture) => (texture.size().width, texture.size().height),
        }
    }

    pub fn aspect(&self) -> f64 {
        let (w, h) = self.size();
        w as f64 / h as f64
    }

    pub fn frame(&self) -> RenderFrame {
        match &self.target {
            TargetInner::Surface(surface) => {
                let surface_texture = surface.surface.get_current_texture().unwrap();
                let view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                RenderFrame::new(view, Some(surface_texture))
            }
            TargetInner::Texture(texture) => RenderFrame::new(
                texture.create_view(&wgpu::TextureViewDescriptor::default()),
                None,
            ),
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
