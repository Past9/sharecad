use std::{
    cell::OnceCell,
    collections::HashMap,
    io::{BufReader, Cursor},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use space::{Point3, Vec2, Vec3};

use crate::{
    color::rgb,
    light::{AmbientLight, DirectionalLight},
    material::{Material, MaterialId, MaterialSpec, RgbSpec, Vec3Spec},
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
    err_material_id: OnceCell<MaterialId>,

    directional_lights: Vec<DirectionalLight>,
    ambient_lights: Vec<AmbientLight>,
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
            err_material_id: OnceCell::new(),

            directional_lights: vec![],
            ambient_lights: vec![],
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

    pub fn directional_lights(&self) -> &[DirectionalLight] {
        &self.directional_lights
    }

    pub fn ambient_lights(&self) -> &[AmbientLight] {
        &self.ambient_lights
    }

    /*
    pub fn light(&self) -> &Light {
        &self.light
    }

    pub fn set_light(&mut self, light: Light) {
        self.light = light;
    }
     */

    fn load_string(file_path: &str) -> String {
        std::fs::read_to_string(file_path).unwrap()
    }

    fn insert_rgb_texture(&mut self, image: image::DynamicImage) -> TextureId {
        let id = self
            .textures
            .iter()
            .filter_map(|(id, texture)| match &texture.image {
                TextureImage::Rgb(texture) => {
                    if *texture == image {
                        Some(id)
                    } else {
                        None
                    }
                }
                TextureImage::Vector(_) => None,
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

    fn insert_vector_map(&mut self, image: image::DynamicImage) -> TextureId {
        let id = self
            .textures
            .iter()
            .filter_map(|(id, texture)| match &texture.image {
                crate::texture::TextureImage::Vector(texture) => {
                    if *texture == image {
                        Some(id)
                    } else {
                        None
                    }
                }
                crate::texture::TextureImage::Rgb(_) => None,
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
        let diffuse_id = self.insert_rgb_texture(spec.diffuse.image());
        let normal_id = self.insert_vector_map(spec.normal.image());
        let emissive_id = self.insert_rgb_texture(spec.emissive.image());
        let roughness_id = self.insert_rgb_texture(spec.roughness.image());
        let metallic_id = self.insert_rgb_texture(spec.metallic.image());
        let ambient_id = self.insert_rgb_texture(spec.ambient.image());

        let id = self
            .materials
            .iter()
            .filter_map(|(id, material)| {
                if diffuse_id == material.diffuse
                    && normal_id == material.normal
                    && emissive_id == material.emissive
                    && roughness_id == material.roughness
                    && metallic_id == material.metallic
                    && ambient_id == material.ambient
                {
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
                self.materials.insert(
                    id,
                    Material::new(
                        id,
                        diffuse_id,
                        normal_id,
                        emissive_id,
                        roughness_id,
                        metallic_id,
                        ambient_id,
                    ),
                );
                id
            }
        }
    }

    pub fn directional_light(&mut self, light: DirectionalLight) {
        self.directional_lights.push(light);
    }

    pub fn ambient_light(&mut self, light: AmbientLight) {
        self.ambient_lights.push(light);
    }

    pub fn load_wavefront_obj_file<T: SceneObjectInstance>(
        &mut self,
        file_path: &str,
        instances: Vec<Vec<T>>,
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

        let missing_material_id =
            self.insert_material(MaterialSpec::default().diffuse_rgb(rgb(1.0, 0.0, 1.0)));

        let mut material_id_map: HashMap<usize, MaterialId> = HashMap::new();
        for (index, m) in obj_materials.unwrap().into_iter().enumerate() {
            let diffuse = {
                if m.diffuse_texture != "" {
                    let mut diffuse_pathbuf = parent_path.clone();
                    diffuse_pathbuf.push(m.diffuse_texture);
                    RgbSpec::from_file(&diffuse_pathbuf.into_os_string().into_string().unwrap())
                } else {
                    RgbSpec::Rgb(rgb(m.diffuse[0], m.diffuse[1], m.diffuse[2]))
                }
            };

            let normal = {
                if m.normal_texture != "" {
                    let mut normal_pathbuf = parent_path.clone();
                    normal_pathbuf.push(m.normal_texture);
                    Vec3Spec::from_file(&normal_pathbuf.into_os_string().into_string().unwrap())
                } else {
                    Vec3Spec::default_normal()
                }
            };

            let emissive = {
                if let Some(emissive) = m.unknown_param.get("map_Ke") {
                    if emissive != "" {
                        let mut emissive_pathbuf = parent_path.clone();
                        emissive_pathbuf.push(emissive);
                        RgbSpec::from_file(
                            &emissive_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        RgbSpec::default_emissive()
                    }
                } else {
                    RgbSpec::default_emissive()
                }
            };

            let roughness = {
                if let Some(roughness) = m.unknown_param.get("map_Pr") {
                    if roughness != "" {
                        let mut roughness_pathbuf = parent_path.clone();
                        roughness_pathbuf.push(roughness);
                        RgbSpec::from_file(
                            &roughness_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        RgbSpec::default_roughness()
                    }
                } else {
                    RgbSpec::default_roughness()
                }
            };

            let metallic = {
                if let Some(metallic) = m.unknown_param.get("map_Pm") {
                    if metallic != "" {
                        let mut metallic_pathbuf = parent_path.clone();
                        metallic_pathbuf.push(metallic);
                        RgbSpec::from_file(
                            &metallic_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        RgbSpec::default_metallic()
                    }
                } else {
                    RgbSpec::default_metallic()
                }
            };

            let ambient = {
                if let Some(ambient) = m.unknown_param.get("map_Po") {
                    if ambient != "" {
                        let mut ambient_pathbuf = parent_path.clone();
                        ambient_pathbuf.push(ambient);
                        RgbSpec::from_file(&ambient_pathbuf.into_os_string().into_string().unwrap())
                    } else {
                        RgbSpec::default_ambient()
                    }
                } else {
                    RgbSpec::default_ambient()
                }
            };

            let id = self.insert_material(
                MaterialSpec::default()
                    .diffuse(diffuse)
                    .normal(normal)
                    .emissive(emissive)
                    .roughness(roughness)
                    .metallic(metallic)
                    .ambient(ambient),
            );

            material_id_map.insert(index, id);
        }

        for m in models.into_iter() {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| MeshVertex {
                    position: [
                        -m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
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
                    (tangent + Vec3::from(vertices[c[0] as usize].tangent)).as_f32s();
                vertices[c[1] as usize].tangent =
                    (tangent + Vec3::from(vertices[c[1] as usize].tangent)).as_f32s();
                vertices[c[2] as usize].tangent =
                    (tangent + Vec3::from(vertices[c[2] as usize].tangent)).as_f32s();
                vertices[c[0] as usize].bitangent =
                    (bitangent + Vec3::from(vertices[c[0] as usize].bitangent)).as_f32s();
                vertices[c[1] as usize].bitangent =
                    (bitangent + Vec3::from(vertices[c[1] as usize].bitangent)).as_f32s();
                vertices[c[2] as usize].bitangent =
                    (bitangent + Vec3::from(vertices[c[2] as usize].bitangent)).as_f32s();

                // Used to average the tangents/bitangents
                triangles_included[c[0] as usize] += 1;
                triangles_included[c[1] as usize] += 1;
                triangles_included[c[2] as usize] += 1;
            }

            // Average the tangents/bitangents
            for (i, n) in triangles_included.into_iter().enumerate() {
                let denom = 1.0 / n as f64;
                let v = &mut vertices[i];
                v.tangent = (Vec3::from(v.tangent) * denom).as_f32s();
                v.bitangent = (Vec3::from(v.bitangent) * denom).as_f32s();
            }

            let mesh = Mesh::new(file_path.into(), vertices, m.mesh.indices);

            let material_id = match m.mesh.material_id {
                Some(index) => match material_id_map.get(&index) {
                    Some(id) => *id,
                    None => missing_material_id,
                },
                None => missing_material_id,
            };

            let object = MeshObject::new(mesh, instances[0].clone(), material_id);

            self.objects.push(Box::new(object));
        }
    }
}
