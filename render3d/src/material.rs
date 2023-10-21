use std::{cell::OnceCell, sync::Arc};

use wgpu::BindGroup;

use crate::texture::Texture;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);
impl From<u32> for MaterialId {
    fn from(id: u32) -> Self {
        MaterialId(id)
    }
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub diffuse: Texture,
    pub normal: Texture,
    bind_group: OnceCell<Arc<wgpu::BindGroup>>,
}
impl Material {
    pub fn new(name: &str, diffuse: Texture, normal: Texture) -> Self {
        Self {
            name: name.into(),
            diffuse,
            normal,
            bind_group: OnceCell::new(),
        }
    }

    pub fn bind_group(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        layout: &wgpu::BindGroupLayout,
    ) -> Arc<BindGroup> {
        self.bind_group
            .get_or_init(|| {
                let diffuse = self.diffuse.resources(device, queue, size);
                let normal = self.normal.resources(device, queue, size);

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
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
                    label: Some(&self.name),
                });

                Arc::new(bind_group)
            })
            .clone()
    }
}
