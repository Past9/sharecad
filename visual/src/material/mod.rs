mod curve;
mod point;
mod surface;

use std::collections::HashMap;

use common::IdSeries;
pub use curve::*;
pub use point::*;
pub use surface::*;

use crate::texture::{ImageTextureKind, Texture, TextureId, TextureImage};

#[derive(Debug)]
pub struct MaterialLibrary {
    texture_ids: IdSeries<TextureId>,
    textures: HashMap<TextureId, Texture>,

    surface_material_ids: IdSeries<SurfaceMaterialId>,
    surface_materials: HashMap<SurfaceMaterialId, SurfaceMaterial>,

    curve_material_ids: IdSeries<CurveMaterialId>,
    curve_materials: HashMap<CurveMaterialId, CurveMaterial>,

    point_material_ids: IdSeries<PointMaterialId>,
    point_materials: HashMap<PointMaterialId, PointMaterial>,

    default_surface_material_id: SurfaceMaterialId,
    default_curve_material_id: CurveMaterialId,
    default_point_material_id: PointMaterialId,
}
impl MaterialLibrary {
    pub fn new() -> Self {
        let mut lib = Self {
            texture_ids: IdSeries::new(),
            textures: HashMap::new(),

            surface_material_ids: IdSeries::new(),
            surface_materials: HashMap::new(),
            curve_material_ids: IdSeries::new(),
            curve_materials: HashMap::new(),
            point_material_ids: IdSeries::new(),
            point_materials: HashMap::new(),

            default_surface_material_id: 0.into(),
            default_curve_material_id: 0.into(),
            default_point_material_id: 0.into(),
        };

        lib.default_surface_material_id =
            lib.insert_surface_material(SurfaceMaterialSpec::default());
        lib.default_curve_material_id = lib.insert_curve_material(CurveMaterialSpec::default());
        lib.default_point_material_id = lib.insert_point_material(PointMaterialSpec::default());

        lib
    }

    pub fn surface(&self) -> &HashMap<SurfaceMaterialId, SurfaceMaterial> {
        &self.surface_materials
    }

    pub fn curve(&self) -> &HashMap<CurveMaterialId, CurveMaterial> {
        &self.curve_materials
    }

    pub fn point(&self) -> &HashMap<PointMaterialId, PointMaterial> {
        &self.point_materials
    }

    pub fn surface_material(&self, id: SurfaceMaterialId) -> Option<&SurfaceMaterial> {
        self.surface_materials.get(&id)
    }

    pub fn curve_material(&self, id: CurveMaterialId) -> Option<&CurveMaterial> {
        self.curve_materials.get(&id)
    }

    pub fn point_material(&self, id: PointMaterialId) -> Option<&PointMaterial> {
        self.point_materials.get(&id)
    }

    pub fn resolve_surface_material(
        &self,
        id: Option<SurfaceMaterialId>,
    ) -> (SurfaceMaterialId, &SurfaceMaterial) {
        let id = id.unwrap_or(self.default_surface_material_id);
        (id, self.surface_materials.get(&id).unwrap())
    }

    pub fn resolve_curve_material(
        &self,
        id: Option<CurveMaterialId>,
    ) -> (CurveMaterialId, &CurveMaterial) {
        let id = id.unwrap_or(self.default_curve_material_id);
        (id, self.curve_materials.get(&id).unwrap())
    }

    pub fn resolve_point_material(
        &self,
        id: Option<PointMaterialId>,
    ) -> (PointMaterialId, &PointMaterial) {
        let id = id.unwrap_or(self.default_point_material_id);
        (id, self.point_materials.get(&id).unwrap())
    }

    pub fn textures(&self) -> &HashMap<TextureId, Texture> {
        &self.textures
    }

    pub fn set_default_surface_material(&mut self, material: SurfaceMaterialId) {
        self.default_surface_material_id = material;
    }

    pub fn set_default_curve_material(&mut self, material: CurveMaterialId) {
        self.default_curve_material_id = material;
    }

    pub fn set_default_point_material(&mut self, material: PointMaterialId) {
        self.default_point_material_id = material;
    }

    pub fn set_surface_material_by_id(&mut self, id: SurfaceMaterialId, spec: SurfaceMaterialSpec) {
        let material = self.make_surface_material(spec);
        self.surface_materials.insert(id, material);
        self.surface_material_ids.advance(id);
    }

    fn make_surface_material(&mut self, spec: SurfaceMaterialSpec) -> SurfaceMaterial {
        SurfaceMaterial::new(
            self.insert_rgb_texture(spec.diffuse.image()),
            self.insert_vector_map(spec.normal.image()),
            self.insert_rgb_texture(spec.emissive.image()),
            self.insert_rgb_texture(spec.roughness.image()),
            self.insert_rgb_texture(spec.metallic.image()),
            self.insert_rgb_texture(spec.ambient.image()),
            self.insert_rgb_texture(spec.transmit.image()),
            spec.is_translucent(),
        )
    }

    fn find_surface_material_id(
        &mut self,
        material: &SurfaceMaterial,
    ) -> Option<SurfaceMaterialId> {
        self.surface_materials
            .iter()
            .filter_map(|(id, existing_material)| {
                if existing_material == material {
                    Some(id)
                } else {
                    None
                }
            })
            .next()
            .cloned()
    }

    pub fn insert_surface_material(&mut self, spec: SurfaceMaterialSpec) -> SurfaceMaterialId {
        let material = self.make_surface_material(spec);

        match self.find_surface_material_id(&material) {
            Some(id) => id,
            None => {
                let id = self.surface_material_ids.next();
                self.surface_materials.insert(id, material);
                id
            }
        }
    }

    pub fn set_curve_material_by_id(&mut self, id: CurveMaterialId, spec: CurveMaterialSpec) {
        let material = self.make_curve_material(spec);
        self.curve_materials.insert(id, material);
        self.curve_material_ids.advance(id);
    }

    fn make_curve_material(&mut self, spec: CurveMaterialSpec) -> CurveMaterial {
        CurveMaterial::new(self.insert_rgb_texture(spec.color.image()))
    }

    fn find_curve_material_id(&mut self, material: &CurveMaterial) -> Option<CurveMaterialId> {
        self.curve_materials
            .iter()
            .filter_map(|(id, existing_material)| {
                if existing_material == material {
                    Some(id)
                } else {
                    None
                }
            })
            .next()
            .cloned()
    }

    pub fn insert_curve_material(&mut self, spec: CurveMaterialSpec) -> CurveMaterialId {
        let material = self.make_curve_material(spec);

        match self.find_curve_material_id(&material) {
            Some(id) => id,
            None => {
                let id = self.curve_material_ids.next();
                self.curve_materials.insert(id, material);
                id
            }
        }
    }

    pub fn set_point_material_by_id(&mut self, id: PointMaterialId, spec: PointMaterialSpec) {
        let material = self.make_point_material(spec);
        self.point_materials.insert(id, material);
        self.point_material_ids.advance(id);
    }

    fn make_point_material(&mut self, spec: PointMaterialSpec) -> PointMaterial {
        PointMaterial::new(self.insert_rgb_texture(spec.color.image()))
    }

    fn find_point_material_id(&mut self, material: &PointMaterial) -> Option<PointMaterialId> {
        self.point_materials
            .iter()
            .filter_map(|(id, existing_material)| {
                if existing_material == material {
                    Some(id)
                } else {
                    None
                }
            })
            .next()
            .cloned()
    }

    pub fn insert_point_material(&mut self, spec: PointMaterialSpec) -> PointMaterialId {
        let material = self.make_point_material(spec);

        match self.find_point_material_id(&material) {
            Some(id) => id,
            None => {
                let id = self.point_material_ids.next();
                self.point_materials.insert(id, material);
                id
            }
        }
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
}
