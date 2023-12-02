use crate::{
    color::rgb,
    light::{AmbientLight, DirectionalLight},
    model::{
        MaterialLibrary, ModelInstance, SceneCurve, SceneModel, ScenePoints, SceneSurface,
        SurfaceId, SurfaceMaterialId, SurfaceMaterialSpec, SurfaceMesh, SurfaceRgbSpec,
        SurfaceVec3Spec, SurfaceVertex,
    },
};
use space::{Vec2, Vec3};
use std::{
    collections::HashMap,
    io::{BufReader, Cursor},
    marker::PhantomData,
    path::Path,
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
    models: Vec<SceneModel>,

    //surfaces: Vec<SceneSurface>,
    curves: Vec<SceneCurve>,
    points: Vec<ScenePoints>,

    materials: MaterialLibrary,

    directional_lights: Vec<DirectionalLight>,
    ambient_lights: Vec<AmbientLight>,
}
unsafe impl Send for Scene {}
impl Scene {
    pub fn new() -> Self {
        Self {
            models: vec![],

            //surfaces: vec![],
            curves: vec![],
            points: vec![],

            materials: MaterialLibrary::new(),

            directional_lights: vec![],
            ambient_lights: vec![],
        }
    }

    pub fn models(&self) -> &[SceneModel] {
        &self.models
    }

    /*
    pub fn surfaces(&self) -> &[SceneSurface] {
        &self.surfaces
    }
     */

    pub fn curves(&self) -> &[SceneCurve] {
        &self.curves
    }

    pub fn set_curves(&mut self, curves: Vec<SceneCurve>) {
        self.curves = curves;
    }

    pub fn points(&self) -> &[ScenePoints] {
        &self.points
    }

    pub fn set_points(&mut self, points: Vec<ScenePoints>) {
        self.points = points;
    }

    pub fn directional_lights(&self) -> &[DirectionalLight] {
        &self.directional_lights
    }

    pub fn ambient_lights(&self) -> &[AmbientLight] {
        &self.ambient_lights
    }

    fn load_string(file_path: &str) -> String {
        std::fs::read_to_string(file_path).unwrap()
    }

    pub fn materials(&self) -> &MaterialLibrary {
        &self.materials
    }

    pub fn materials_mut(&mut self) -> &mut MaterialLibrary {
        &mut self.materials
    }

    pub fn set_directional_lights(&mut self, lights: Vec<DirectionalLight>) {
        self.directional_lights = lights;
    }

    pub fn directional_light(&mut self, light: DirectionalLight) {
        self.directional_lights.push(light);
    }

    pub fn set_ambient_light(&mut self, light: AmbientLight) {
        self.ambient_lights.push(light);
    }

    pub fn load_wavefront_obj_file(&mut self, file_path: &str, instances: Vec<ModelInstance>) {
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

        let missing_material_id = self.materials.insert_surface_material(
            SurfaceMaterialSpec::default().diffuse_rgb(rgb(1.0, 0.0, 1.0)),
        );

        let mut material_id_map: HashMap<usize, SurfaceMaterialId> = HashMap::new();
        for (index, m) in obj_materials.unwrap().into_iter().enumerate() {
            let diffuse = {
                if m.diffuse_texture != "" {
                    let mut diffuse_pathbuf = parent_path.clone();
                    diffuse_pathbuf.push(m.diffuse_texture);
                    SurfaceRgbSpec::from_file(
                        &diffuse_pathbuf.into_os_string().into_string().unwrap(),
                    )
                } else {
                    SurfaceRgbSpec::Rgb(rgb(m.diffuse[0], m.diffuse[1], m.diffuse[2]))
                }
            };

            let normal = {
                if m.normal_texture != "" {
                    let mut normal_pathbuf = parent_path.clone();
                    normal_pathbuf.push(m.normal_texture);
                    SurfaceVec3Spec::from_file(
                        &normal_pathbuf.into_os_string().into_string().unwrap(),
                    )
                } else {
                    SurfaceVec3Spec::default_normal()
                }
            };

            let emissive = {
                if let Some(emissive) = m.unknown_param.get("map_Ke") {
                    if emissive != "" {
                        let mut emissive_pathbuf = parent_path.clone();
                        emissive_pathbuf.push(emissive);
                        SurfaceRgbSpec::from_file(
                            &emissive_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        SurfaceRgbSpec::default_emissive()
                    }
                } else {
                    SurfaceRgbSpec::default_emissive()
                }
            };

            let roughness = {
                if let Some(roughness) = m.unknown_param.get("map_Pr") {
                    if roughness != "" {
                        let mut roughness_pathbuf = parent_path.clone();
                        roughness_pathbuf.push(roughness);
                        SurfaceRgbSpec::from_file(
                            &roughness_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        SurfaceRgbSpec::default_roughness()
                    }
                } else {
                    SurfaceRgbSpec::default_roughness()
                }
            };

            let metallic = {
                if let Some(metallic) = m.unknown_param.get("map_Pm") {
                    if metallic != "" {
                        let mut metallic_pathbuf = parent_path.clone();
                        metallic_pathbuf.push(metallic);
                        SurfaceRgbSpec::from_file(
                            &metallic_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        SurfaceRgbSpec::default_metallic()
                    }
                } else {
                    SurfaceRgbSpec::default_metallic()
                }
            };

            let ambient = {
                if let Some(ambient) = m.unknown_param.get("map_Po") {
                    if ambient != "" {
                        let mut ambient_pathbuf = parent_path.clone();
                        ambient_pathbuf.push(ambient);
                        SurfaceRgbSpec::from_file(
                            &ambient_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        SurfaceRgbSpec::default_ambient()
                    }
                } else {
                    SurfaceRgbSpec::default_ambient()
                }
            };

            let transmit = {
                if let Some(transmit) = m.unknown_param.get("map_Kt") {
                    if transmit != "" {
                        let mut transmit_pathbuf = parent_path.clone();
                        transmit_pathbuf.push(transmit);
                        SurfaceRgbSpec::from_file(
                            &transmit_pathbuf.into_os_string().into_string().unwrap(),
                        )
                    } else {
                        SurfaceRgbSpec::default_transmit()
                    }
                } else {
                    SurfaceRgbSpec::default_transmit()
                }
            };

            let id = self.materials.insert_surface_material(
                SurfaceMaterialSpec::default()
                    .diffuse(diffuse)
                    .normal(normal)
                    .emissive(emissive)
                    .roughness(roughness)
                    .metallic(metallic)
                    .ambient(ambient)
                    .transmit(transmit),
            );

            material_id_map.insert(index, id);
        }

        let mut surfaces = vec![];

        for m in models.into_iter() {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| SurfaceVertex {
                    id: 1,
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

            let mesh = SurfaceMesh::new(vertices, m.mesh.indices);

            let material_id = match m.mesh.material_id {
                Some(index) => match material_id_map.get(&index) {
                    Some(id) => *id,
                    None => missing_material_id,
                },
                None => missing_material_id,
            };

            surfaces.push(SceneSurface::new(SurfaceId(1), mesh, material_id));
        }

        self.models
            .push(SceneModel::new(surfaces, vec![], vec![], instances));
    }
}
