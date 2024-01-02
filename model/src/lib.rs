use std::collections::HashMap;

use common::{CurveId, PointId, SurfaceId};
use geometry::Geometry;
use visual::material::{CurveMaterialId, MaterialLibrary, PointMaterialId, SurfaceMaterialId};

pub struct PrimitiveModel {
    geometry: Geometry,

    materials: MaterialLibrary,

    surface_materials: HashMap<SurfaceId, SurfaceMaterialId>,
    curve_materials: HashMap<CurveId, CurveMaterialId>,
    point_materials: HashMap<PointId, PointMaterialId>,
}
impl PrimitiveModel {}
