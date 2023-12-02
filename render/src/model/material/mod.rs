mod curve;
mod point;
mod surface;

use std::collections::HashMap;

pub use curve::*;
pub use point::*;
pub use surface::*;

use crate::{
    scene::IdSeries,
    texture::{ImageTextureKind, Texture, TextureId, TextureImage},
};

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
}
impl MaterialLibrary {
    pub fn new() -> Self {
        Self {
            texture_ids: IdSeries::new(),
            textures: HashMap::new(),

            surface_material_ids: IdSeries::new(),
            surface_materials: HashMap::new(),
            curve_material_ids: IdSeries::new(),
            curve_materials: HashMap::new(),
            point_material_ids: IdSeries::new(),
            point_materials: HashMap::new(),
        }
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

    pub fn textures(&self) -> &HashMap<TextureId, Texture> {
        &self.textures
    }

    pub fn insert_surface_material(&mut self, spec: SurfaceMaterialSpec) -> SurfaceMaterialId {
        let diffuse_id = self.insert_rgb_texture(spec.diffuse.image());
        let normal_id = self.insert_vector_map(spec.normal.image());
        let emissive_id = self.insert_rgb_texture(spec.emissive.image());
        let roughness_id = self.insert_rgb_texture(spec.roughness.image());
        let metallic_id = self.insert_rgb_texture(spec.metallic.image());
        let ambient_id = self.insert_rgb_texture(spec.ambient.image());
        let transmit_id = self.insert_rgb_texture(spec.transmit.image());
        let is_translucent = spec.is_translucent();

        let id = self
            .surface_materials
            .iter()
            .filter_map(|(id, material)| {
                if diffuse_id == material.diffuse
                    && normal_id == material.normal
                    && emissive_id == material.emissive
                    && roughness_id == material.roughness
                    && metallic_id == material.metallic
                    && ambient_id == material.ambient
                    && transmit_id == material.transmit
                    && is_translucent == material.is_translucent
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
                let id = self.surface_material_ids.next();
                self.surface_materials.insert(
                    id,
                    SurfaceMaterial::new(
                        id,
                        diffuse_id,
                        normal_id,
                        emissive_id,
                        roughness_id,
                        metallic_id,
                        ambient_id,
                        transmit_id,
                        is_translucent,
                    ),
                );
                id
            }
        }
    }

    pub fn insert_curve_material(&mut self, spec: CurveMaterialSpec) -> CurveMaterialId {
        let color_id = self.insert_rgb_texture(spec.color.image());

        let id = self
            .curve_materials
            .iter()
            .filter_map(|(id, material)| {
                if color_id == material.color {
                    Some(id)
                } else {
                    None
                }
            })
            .next();

        match id {
            Some(id) => *id,
            None => {
                let id = self.curve_material_ids.next();
                self.curve_materials
                    .insert(id, CurveMaterial::new(id, color_id));
                id
            }
        }
    }

    pub fn insert_point_material(&mut self, spec: PointMaterialSpec) -> PointMaterialId {
        let color_id = self.insert_rgb_texture(spec.color.image());

        let id = self
            .point_materials
            .iter()
            .filter_map(|(id, material)| {
                if color_id == material.color {
                    Some(id)
                } else {
                    None
                }
            })
            .next();

        match id {
            Some(id) => *id,
            None => {
                let id = self.point_material_ids.next();
                self.point_materials
                    .insert(id, PointMaterial::new(id, color_id));
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
