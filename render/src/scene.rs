use std::{
    collections::HashMap,
    io::{BufReader, Cursor},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use space::{Point3, Vec2, Vec3};

use crate::{
    light::Light,
    material::{Material, MaterialId},
    model::{Mesh, MeshObject, MeshVertex, SceneObject, SceneObjectInstance},
    texture::{ImageTextureKind, Texture, TextureId},
};

#[derive(Debug)]
pub(crate) struct IdSeries<T: From<u32>> {
    last_id: u32,
    _t: PhantomData<T>,
}
impl<T: From<u32>> IdSeries<T> {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            _t: PhantomData,
        }
    }

    pub fn next(&mut self) -> T {
        self.last_id += 1;
        self.last_id.into()
    }
}

#[derive(Debug)]
pub struct Scene {
    objects: Vec<Box<dyn SceneObject>>,

    texture_ids: IdSeries<TextureId>,
    textures: HashMap<TextureId, Texture>,

    material_ids: IdSeries<MaterialId>,
    materials: HashMap<MaterialId, Material>,

    light: Light,
}
impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![],

            texture_ids: IdSeries::new(),
            textures: HashMap::new(),

            material_ids: IdSeries::new(),
            materials: HashMap::new(),

            light: Light::new(Point3::ZERO, [0.0; 3]),
        }
    }

    pub fn objects(&self) -> &[Box<dyn SceneObject>] {
        &self.objects
    }

    pub fn materials(&self) -> &HashMap<MaterialId, Material> {
        &self.materials
    }

    pub fn textures(&self) -> &HashMap<TextureId, Texture> {
        &self.textures
    }

    pub fn light(&self) -> &Light {
        &self.light
    }

    pub fn set_light(&mut self, light: Light) {
        self.light = light;
    }

    async fn load_string(file_path: &str) -> String {
        println!("load_string file_path = {}", file_path);
        std::fs::read_to_string(file_path).unwrap()
    }

    async fn load_binary(file_path: &str) -> Vec<u8> {
        println!("load_binary file_path = {}", file_path);
        std::fs::read(file_path).unwrap()
    }

    async fn load_texture(
        &self,
        id: TextureId,
        file_path: &str,
        kind: ImageTextureKind,
    ) -> Texture {
        println!("load_texture file_path = {}", file_path);
        let data = Self::load_binary(file_path).await;
        Texture::from_bytes(id, &data, file_path, kind)
    }

    pub async fn load_model_file<T: SceneObjectInstance>(
        &mut self,
        file_path: &str,
        mut instances: Vec<Vec<T>>,
    ) {
        println!("load_model_file file_path = {}", file_path);
        let mut parent_path = Path::new(file_path).parent().unwrap();

        let obj_text = Self::load_string(file_path).await;
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
                let mut material_pathbuf = PathBuf::from(parent_path);
                material_pathbuf.push(p);

                let mat_text =
                    Self::load_string(&material_pathbuf.into_os_string().into_string().unwrap())
                        .await;
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )
        .await
        .unwrap();

        for m in obj_materials.unwrap().into_iter() {
            let diffuse_tex_id = self.texture_ids.next();
            let normal_tex_id = self.texture_ids.next();
            let material_id = self.material_ids.next();

            let mut diffuse_pathbuf = PathBuf::from(parent_path);
            diffuse_pathbuf.push(m.diffuse_texture);

            self.textures.insert(
                diffuse_tex_id,
                self.load_texture(
                    diffuse_tex_id,
                    &diffuse_pathbuf.into_os_string().into_string().unwrap(),
                    ImageTextureKind::Diffuse,
                )
                .await,
            );

            let mut normal_pathbuf = PathBuf::from(parent_path);
            normal_pathbuf.push(m.normal_texture);

            self.textures.insert(
                normal_tex_id,
                self.load_texture(
                    normal_tex_id,
                    &normal_pathbuf.into_os_string().into_string().unwrap(),
                    ImageTextureKind::NormalMap,
                )
                .await,
            );

            let material = Material::new(material_id, &m.name, diffuse_tex_id, normal_tex_id);

            self.materials.insert(material.id, material);
        }

        for m in models.into_iter() {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| MeshVertex {
                    position: [
                        -m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        -m.mesh.normals[i * 3],
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

                let pos0: Vec3 = v0.position.into();
                let pos1: Vec3 = v1.position.into();
                let pos2: Vec3 = v2.position.into();

                let uv0: Vec2 = v0.tex_coords.into();
                let uv1: Vec2 = v1.tex_coords.into();
                let uv2: Vec2 = v2.tex_coords.into();

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
                    (tangent + Vec3::from(vertices[c[0] as usize].tangent)).to_f32s();
                vertices[c[1] as usize].tangent =
                    (tangent + Vec3::from(vertices[c[1] as usize].tangent)).to_f32s();
                vertices[c[2] as usize].tangent =
                    (tangent + Vec3::from(vertices[c[2] as usize].tangent)).to_f32s();
                vertices[c[0] as usize].bitangent =
                    (bitangent + Vec3::from(vertices[c[0] as usize].bitangent)).to_f32s();
                vertices[c[1] as usize].bitangent =
                    (bitangent + Vec3::from(vertices[c[1] as usize].bitangent)).to_f32s();
                vertices[c[2] as usize].bitangent =
                    (bitangent + Vec3::from(vertices[c[2] as usize].bitangent)).to_f32s();

                // Used to average the tangents/bitangents
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }

            // Average the tangents/bitangents
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denom = 1.0 / n as f64;
                let v = &mut vertices[i];
                v.tangent = (Vec3::from(v.tangent) * denom).to_f32s();
                v.bitangent = (Vec3::from(v.bitangent) * denom).to_f32s();
            }

            let mesh = Mesh::new(file_path.into(), vertices, m.mesh.indices);

            let object = MeshObject::new(
                mesh,
                instances.remove(0),
                MaterialId(self.materials.len() as u32 + m.mesh.material_id.unwrap() as u32),
            );

            self.objects.push(Box::new(object));
        }
    }
}
