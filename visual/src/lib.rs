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
    fn set_surface_material(&mut self, surface: SurfaceId, material: SurfaceMaterialId);
    fn set_curve_material(&mut self, curve: CurveId, material: CurveMaterialId);
    fn set_point_material(&mut self, point: PointId, material: PointMaterialId);
    fn get_surface_material(&self, surface: SurfaceId) -> Option<SurfaceMaterialId>;
    fn get_curve_material(&self, curve: CurveId) -> Option<CurveMaterialId>;
    fn get_point_material(&self, point: PointId) -> Option<PointMaterialId>;
}

pub struct GeometryVisuals {
    surface_materials: HashMap<SurfaceId, SurfaceMaterialId>,
    curve_materials: HashMap<CurveId, CurveMaterialId>,
    point_materials: HashMap<PointId, PointMaterialId>,
}
impl GeometryVisuals {
    pub fn new() -> Self {
        Self {
            surface_materials: HashMap::new(),
            curve_materials: HashMap::new(),
            point_materials: HashMap::new(),
        }
    }
}
impl IGeometryVisuals for GeometryVisuals {
    fn set_surface_material(&mut self, surface: SurfaceId, material: SurfaceMaterialId) {
        self.surface_materials.insert(surface, material);
    }

    fn set_curve_material(&mut self, curve: CurveId, material: CurveMaterialId) {
        self.curve_materials.insert(curve, material);
    }

    fn set_point_material(&mut self, point: PointId, material: PointMaterialId) {
        self.point_materials.insert(point, material);
    }

    fn get_surface_material(&self, surface: SurfaceId) -> Option<SurfaceMaterialId> {
        self.surface_materials.get(&surface).cloned()
    }

    fn get_curve_material(&self, curve: CurveId) -> Option<CurveMaterialId> {
        self.curve_materials.get(&curve).cloned()
    }

    fn get_point_material(&self, point: PointId) -> Option<PointMaterialId> {
        self.point_materials.get(&point).cloned()
    }
}
