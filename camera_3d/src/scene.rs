use crate::{
    instance::{CubeInstance, InstanceRaw, VertexBuffer},
    texture::Texture,
};
use bytemuck::{Pod, Zeroable};
use std::{
    cell::OnceCell,
    collections::HashMap,
    io::{BufReader, Cursor},
    sync::Arc,
};
use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);

#[derive(Copy, Clone, Debug)]
pub struct InstanceId(pub u32);

pub trait Object: std::fmt::Debug {
    fn mesh(&self) -> &Mesh;
    fn instance_buffer(&self) -> &wgpu::Buffer;
    fn material_id(&self) -> MaterialId;
    fn num_instances(&self) -> u32;
}

pub trait Instance: std::fmt::Debug + 'static {
    type RawBuffer: VertexBuffer;

    fn id(&self) -> InstanceId;
    fn to_raw(&self) -> Self::RawBuffer;
}

pub trait DrawVisualScene<'a> {
    fn draw_visual(
        &mut self,
        scene: &'a Scene,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    );
}
impl<'a> DrawVisualScene<'a> for wgpu::RenderPass<'a> {
    fn draw_visual(
        &mut self,
        scene: &'a Scene,
        camera_bind_group: &'a wgpu::BindGroup,
        light_bind_group: &'a wgpu::BindGroup,
    ) {
        for object in scene.objects.iter() {
            let mesh = object.mesh();
            let material = scene.materials.get(&object.material_id()).expect(&format!(
                "Could not find material {:?}",
                object.material_id()
            ));
            self.set_vertex_buffer(1, object.instance_buffer().slice(..));
            self.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
            self.set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
            self.set_bind_group(0, &material.bind_group, &[]);
            {
                // TODO: Move these out of the loop? Probably don't need to set these for
                // every object since they don't change.
                self.set_bind_group(1, &camera_bind_group, &[]);
                self.set_bind_group(2, &light_bind_group, &[]);
            }
            self.draw_indexed(0..mesh.num_elements(), 0, 0..object.num_instances());
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    device: Arc<wgpu::Device>,
    objects: Vec<Box<dyn Object>>,
    materials: HashMap<MaterialId, Material>,
}
impl Scene {
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            objects: vec![],
            materials: HashMap::new(),
        }
    }

    pub fn materials(&self) -> &HashMap<MaterialId, Material> {
        &self.materials
    }

    async fn load_string(file_name: &str) -> String {
        let path = std::path::Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_name);
        std::fs::read_to_string(path).unwrap()
    }

    async fn load_binary(file_name: &str) -> Vec<u8> {
        let path = std::path::Path::new(env!("OUT_DIR"))
            .join("res")
            .join(file_name);
        std::fs::read(path).unwrap()
    }

    async fn load_texture(
        &self,
        file_name: &str,
        is_normal_map: bool,
        queue: &wgpu::Queue,
    ) -> Texture {
        let data = Self::load_binary(file_name).await;
        Texture::from_bytes(&self.device, queue, &data, file_name, is_normal_map).unwrap()
    }

    pub async fn load_model_file<T: Instance>(
        &mut self,
        file_name: &str,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        mut instances: Vec<Vec<T>>,
    ) {
        let obj_text = Self::load_string(file_name).await;
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| async move {
                let mat_text = Self::load_string(&p).await;
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )
        .await
        .unwrap();

        for (id, m) in obj_materials.unwrap().into_iter().enumerate() {
            let diffuse_texture = self.load_texture(&m.diffuse_texture, false, queue).await;
            let normal_texture = self.load_texture(&m.normal_texture, true, queue).await;

            let material = Material::new(
                &self.device,
                &m.name,
                diffuse_texture,
                normal_texture,
                layout,
            );

            self.materials.insert(MaterialId(id as u32 + 1), material);
        }

        for m in models.into_iter() {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| MeshVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                    tangent: [0.0; 3],
                    bitangent: [0.0; 3],
                    param_coords: [0.0, 0.0],
                })
                .collect::<Vec<_>>();

            let indices = &m.mesh.indices;
            let mut triangles_included = vec![0; vertices.len()];

            // Calculate tangents and bitangets. We're going to
            // use the triangles, so we need to loop through the
            // indices in chunks of 3
            for c in indices.chunks(3) {
                let v0 = vertices[c[0] as usize];
                let v1 = vertices[c[1] as usize];
                let v2 = vertices[c[2] as usize];

                let pos0: cgmath::Vector3<_> = v0.position.into();
                let pos1: cgmath::Vector3<_> = v1.position.into();
                let pos2: cgmath::Vector3<_> = v2.position.into();

                let uv0: cgmath::Vector2<_> = v0.tex_coords.into();
                let uv1: cgmath::Vector2<_> = v1.tex_coords.into();
                let uv2: cgmath::Vector2<_> = v2.tex_coords.into();

                // Calculate the edges of the triangle
                let delta_pos1 = pos1 - pos0;
                let delta_pos2 = pos2 - pos0;

                // This will give us a direction to calculate the
                // tangent and bitangent
                let delta_uv1 = uv1 - uv0;
                let delta_uv2 = uv2 - uv0;

                // Solving the following system of equations will
                // give us the tangent and bitangent.
                //     delta_pos1 = delta_uv1.x * T + delta_u.y * B
                //     delta_pos2 = delta_uv2.x * T + delta_uv2.y * B
                // Luckily, the place I found this equation provided
                // the solution!
                let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                // We flip the bitangent to enable right-handed normal
                // maps with wgpu texture coordinate system
                let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

                // We'll use the same tangent/bitangent for each vertex in the triangle
                vertices[c[0] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[0] as usize].tangent)).into();
                vertices[c[1] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[1] as usize].tangent)).into();
                vertices[c[2] as usize].tangent =
                    (tangent + cgmath::Vector3::from(vertices[c[2] as usize].tangent)).into();
                vertices[c[0] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[0] as usize].bitangent)).into();
                vertices[c[1] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[1] as usize].bitangent)).into();
                vertices[c[2] as usize].bitangent =
                    (bitangent + cgmath::Vector3::from(vertices[c[2] as usize].bitangent)).into();

                // Used to average the tangents/bitangents
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }

            // Average the tangents/bitangents
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denom = 1.0 / n as f32;
                let mut v = &mut vertices[i];
                v.tangent = (cgmath::Vector3::from(v.tangent) * denom).into();
                v.bitangent = (cgmath::Vector3::from(v.bitangent) * denom).into();
            }

            let mesh = Mesh::new(
                self.device.clone(),
                file_name.into(),
                vertices,
                m.mesh.indices,
            );

            let object: MeshObject<T> = MeshObject {
                device: self.device.clone(),
                mesh,
                instances: instances.remove(0),
                instance_buffer: OnceCell::new(),
                material_id: MaterialId(
                    self.materials.len() as u32 + m.mesh.material_id.unwrap() as u32,
                ),
            };

            self.objects.push(Box::new(object));
        }
    }
}

#[derive(Debug)]
pub struct MeshObject<T: Instance> {
    device: Arc<wgpu::Device>,
    mesh: Mesh,
    instances: Vec<T>,
    instance_buffer: OnceCell<wgpu::Buffer>,
    material_id: MaterialId,
}
impl<T: Instance> Object for MeshObject<T> {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    fn instance_buffer(&self) -> &wgpu::Buffer {
        self.instance_buffer.get_or_init(|| {
            let instance_data = self
                .instances
                .iter()
                .map(|inst| inst.to_raw())
                .collect::<Vec<_>>();
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance buffer"),
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        })
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }

    fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug)]
pub struct Mesh {
    device: Arc<wgpu::Device>,
    name: String,
    vertices: Vec<MeshVertex>,
    indices: Vec<u32>,
    vertex_buffer: OnceCell<wgpu::Buffer>,
    index_buffer: OnceCell<wgpu::Buffer>,
}
impl Mesh {
    pub fn new(
        device: Arc<wgpu::Device>,
        name: &str,
        vertices: Vec<MeshVertex>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            device,
            name: name.into(),
            vertices,
            indices,
            vertex_buffer: OnceCell::new(),
            index_buffer: OnceCell::new(),
        }
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        self.vertex_buffer.get_or_init(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{} vertex buffer", self.name)),
                    contents: bytemuck::cast_slice(&self.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        })
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        self.index_buffer.get_or_init(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{} index buffer", self.name)),
                    contents: bytemuck::cast_slice(&self.indices),
                    usage: wgpu::BufferUsages::INDEX,
                })
        })
    }

    pub fn num_elements(&self) -> u32 {
        self.indices.len() as u32
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct MeshVertex {
    /// Position in world space
    position: [f32; 3],

    /// Texture UV coordinates
    tex_coords: [f32; 2],

    /// Normal vector
    normal: [f32; 3],

    /// Tangent vector
    tangent: [f32; 3],

    /// Bitangent vector
    bitangent: [f32; 3],

    /// Parameteric surface UV coordinates
    param_coords: [f32; 2],
}
impl MeshVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x2,
        2 => Float32x3,
        3 => Float32x3,
        4 => Float32x3,
        5 => Float32x2
    ];
}
impl VertexBuffer for MeshVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
    pub normal_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}
impl Material {
    pub fn new(
        device: &wgpu::Device,
        name: &str,
        diffuse_texture: Texture,
        normal_texture: Texture,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&normal_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&normal_texture.sampler),
                },
            ],
            label: Some(name),
        });

        Self {
            name: String::from(name),
            diffuse_texture,
            normal_texture,
            bind_group: bind_group,
        }
    }
}
