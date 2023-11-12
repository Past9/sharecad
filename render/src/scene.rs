use std::{
    collections::HashMap,
    io::{BufReader, Cursor},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use space::{Point3, Vec2, Vec3};

use crate::{
    light::Light,
    material::{rgba, Material, MaterialId, MaterialSpec},
    model::{Mesh, MeshObject, MeshVertex, SceneObject, SceneObjectInstance},
    texture::{ImageTextureKind, Texture, TextureId, TextureImage},
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
unsafe impl Send for Scene {}
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

    fn load_string(file_path: &str) -> String {
        std::fs::read_to_string(file_path).unwrap()
    }

    fn load_binary(file_path: &str) -> Vec<u8> {
        std::fs::read(file_path).unwrap()
    }

    fn load_texture(&self, id: TextureId, file_path: &str, kind: ImageTextureKind) -> Texture {
        let data = Self::load_binary(file_path);
        Texture::from_bytes(id, &data, kind)
    }

    fn insert_diffuse_texture(&mut self, image: image::DynamicImage) -> TextureId {
        let id = self
            .textures
            .iter()
            .filter_map(|(id, texture)| match &texture.image {
                TextureImage::Diffuse(texture) => {
                    if *texture == image {
                        Some(id)
                    } else {
                        None
                    }
                }
                TextureImage::NormalMap(_) => None,
            })
            .next();

        match id {
            Some(id) => *id,
            None => {
                let id = self.texture_ids.next();
                self.textures.insert(
                    id,
                    Texture::from_image(id, image, ImageTextureKind::Diffuse),
                );
                id
            }
        }
    }

    fn insert_normal_map(&mut self, image: image::DynamicImage) -> TextureId {
        let id = self
            .textures
            .iter()
            .filter_map(|(id, texture)| match &texture.image {
                crate::texture::TextureImage::NormalMap(texture) => {
                    if *texture == image {
                        Some(id)
                    } else {
                        None
                    }
                }
                crate::texture::TextureImage::Diffuse(_) => None,
            })
            .next();

        match id {
            Some(id) => *id,
            None => {
                let id = self.texture_ids.next();
                self.textures.insert(
                    id,
                    Texture::from_image(id, image, ImageTextureKind::NormalMap),
                );
                id
            }
        }
    }

    fn insert_material(&mut self, spec: MaterialSpec) -> MaterialId {
        let diffuse_id = self.insert_diffuse_texture(spec.diffuse.image());
        let normal_id = self.insert_normal_map(spec.normal.image());

        let id = self
            .materials
            .iter()
            .filter_map(|(id, material)| {
                if diffuse_id == material.diffuse && normal_id == material.normal {
                    Some(id)
                } else {
                    None
                }
            })
            .next();

        match id {
            Some(id) => *id,
            None => {
                let id = self.material_ids.next();
                self.materials
                    .insert(id, Material::new(id, diffuse_id, normal_id));
                id
            }
        }
    }

    pub fn load_model_file<T: SceneObjectInstance>(
        &mut self,
        file_path: &str,
        mut instances: Vec<Vec<T>>,
    ) {
        let parent_path = Path::new(file_path).parent().unwrap().to_path_buf();

        let obj_text = Self::load_string(file_path);
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf(
            &mut obj_reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| {
                let mut material_pathbuf = parent_path.clone();
                material_pathbuf.push(p);

                let mat_text =
                    Self::load_string(&material_pathbuf.into_os_string().into_string().unwrap());
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            },
        )
        .unwrap();

        for m in obj_materials.unwrap().into_iter() {
            let mut diffuse_pathbuf = parent_path.clone();
            diffuse_pathbuf.push(m.diffuse_texture);

            let mut normal_pathbuf = parent_path.clone();
            normal_pathbuf.push(m.normal_texture);

            self.insert_material(
                MaterialSpec::default()
                    .diffuse_from_file(&diffuse_pathbuf.into_os_string().into_string().unwrap()), //.normal_from_file(&normal_pathbuf.into_os_string().into_string().unwrap()),
            );
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
