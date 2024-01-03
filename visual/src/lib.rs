use std::collections::HashMap;

use common::{CurveId, PointId, SurfaceId};
use material::{
    CurveMaterialId, CurveMaterialSpec, MaterialLibrary, PointMaterialId, PointMaterialSpec,
    SurfaceMaterialId, SurfaceMaterialSpec,
};

pub mod color;
pub mod material;
pub mod texture;

pub trait IGeometryVisuals {
    fn set_default_surface_material(&mut self, spec: SurfaceMaterialSpec);
    fn set_default_curve_material(&mut self, spec: CurveMaterialSpec);
    fn set_default_point_material(&mut self, spec: PointMaterialSpec);
    fn create_surface_material(&mut self, spec: SurfaceMaterialSpec) -> SurfaceMaterialId;
    fn create_curve_material(&mut self, spec: CurveMaterialSpec) -> CurveMaterialId;
    fn create_point_material(&mut self, spec: PointMaterialSpec) -> PointMaterialId;
    fn set_surface_material(&mut self, surface: SurfaceId, material: SurfaceMaterialId);
    fn set_curve_material(&mut self, curve: CurveId, material: CurveMaterialId);
    fn set_point_material(&mut self, point: PointId, material: PointMaterialId);
}

pub struct GeometryVisuals {
    materials: MaterialLibrary,

    default_surface_material_id: SurfaceMaterialId,
    default_curve_material_id: CurveMaterialId,
    default_point_material_id: PointMaterialId,

    surface_materials: HashMap<SurfaceId, SurfaceMaterialId>,
    curve_materials: HashMap<CurveId, CurveMaterialId>,
    point_materials: HashMap<PointId, PointMaterialId>,
}
impl GeometryVisuals {
    pub fn new() -> Self {
        let mut materials = MaterialLibrary::new();

        let default_surface_material_id =
            materials.insert_surface_material(SurfaceMaterialSpec::default());
        let default_curve_material_id =
            materials.insert_curve_material(CurveMaterialSpec::default());
        let default_point_material_id =
            materials.insert_point_material(PointMaterialSpec::default());

        Self {
            materials,

            default_surface_material_id,
            default_curve_material_id,
            default_point_material_id,

            surface_materials: HashMap::new(),
            curve_materials: HashMap::new(),
            point_materials: HashMap::new(),
        }
    }
}
impl IGeometryVisuals for GeometryVisuals {
    fn set_default_surface_material(&mut self, spec: SurfaceMaterialSpec) {
        self.materials
            .set_surface_material_by_id(self.default_surface_material_id, spec)
    }

    fn set_default_curve_material(&mut self, spec: CurveMaterialSpec) {
        self.materials
            .set_curve_material_by_id(self.default_curve_material_id, spec)
    }

    fn set_default_point_material(&mut self, spec: PointMaterialSpec) {
        self.materials
            .set_point_material_by_id(self.default_point_material_id, spec)
    }

    fn create_surface_material(&mut self, spec: SurfaceMaterialSpec) -> SurfaceMaterialId {
        self.materials.insert_surface_material(spec)
    }

    fn create_curve_material(&mut self, spec: CurveMaterialSpec) -> CurveMaterialId {
        self.materials.insert_curve_material(spec)
    }

    fn create_point_material(&mut self, spec: PointMaterialSpec) -> PointMaterialId {
        self.materials.insert_point_material(spec)
    }

    fn set_surface_material(&mut self, surface: SurfaceId, material: SurfaceMaterialId) {
        self.surface_materials.insert(surface, material);
    }

    fn set_curve_material(&mut self, curve: CurveId, material: CurveMaterialId) {
        self.curve_materials.insert(curve, material);
    }

    fn set_point_material(&mut self, point: PointId, material: PointMaterialId) {
        self.point_materials.insert(point, material);
    }
}
