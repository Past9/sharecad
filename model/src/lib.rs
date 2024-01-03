use std::collections::HashMap;

use common::{CurveId, PointId, SurfaceId};
use geometry::{
    primitives::{Curve, Surface},
    IGeometry, PrimitiveGeometry,
};
use space::{Angle, Point3, Quat, Vec3};
use visual::{
    material::{CurveMaterialId, PointMaterialId, SurfaceMaterialId},
    GeometryVisuals, IGeometryVisuals,
};

pub struct PrimitiveModel {
    geometry: PrimitiveGeometry,
    visuals: GeometryVisuals,
}
impl PrimitiveModel {
    pub fn new() -> Self {
        Self {
            geometry: PrimitiveGeometry::new(),
            visuals: GeometryVisuals::new(),
        }
    }
}
impl IGeometry for PrimitiveModel {
    fn create_point(&mut self, point: Point3) -> PointId {
        self.geometry.create_point(point)
    }

    fn create_curve(&mut self, curve: Curve) -> CurveId {
        self.geometry.create_curve(curve)
    }

    fn create_surface(&mut self, surface: Surface) -> SurfaceId {
        self.geometry.create_surface(surface)
    }

    fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId {
        self.geometry.create_line_between(start, end)
    }

    fn create_arc(
        &mut self,
        r: f64,
        angle: Angle,
        orientation: Quat,
        translation: Vec3,
    ) -> CurveId {
        self.geometry.create_arc(r, angle, orientation, translation)
    }

    fn create_sweep(&mut self, profile: CurveId, path: CurveId) -> SurfaceId {
        self.geometry.create_sweep(profile, path)
    }

    fn point(&self, id: PointId) -> Option<&Point3> {
        self.geometry.point(id)
    }

    fn curve(&self, id: CurveId) -> Option<&Curve> {
        self.geometry.curve(id)
    }

    fn surface(&self, id: SurfaceId) -> Option<&Surface> {
        self.geometry.surface(id)
    }

    fn curve_solver(&self, id: CurveId) -> Option<geometry::primitives::CurveSolver> {
        self.geometry.curve_solver(id)
    }

    fn surface_solver(&self, id: SurfaceId) -> Option<geometry::primitives::SurfaceSolver> {
        self.geometry.surface_solver(id)
    }

    fn surfaces(&self) -> &HashMap<SurfaceId, Surface> {
        self.geometry.surfaces()
    }

    fn curves(&self) -> &HashMap<CurveId, Curve> {
        self.geometry.curves()
    }

    fn points(&self) -> &HashMap<PointId, Point3> {
        self.geometry.points()
    }
}
impl IGeometryVisuals for PrimitiveModel {
    fn set_default_surface_material(&mut self, spec: visual::material::SurfaceMaterialSpec) {
        self.visuals.set_default_surface_material(spec)
    }

    fn set_default_curve_material(&mut self, spec: visual::material::CurveMaterialSpec) {
        self.visuals.set_default_curve_material(spec)
    }

    fn set_default_point_material(&mut self, spec: visual::material::PointMaterialSpec) {
        self.visuals.set_default_point_material(spec)
    }

    fn create_surface_material(
        &mut self,
        spec: visual::material::SurfaceMaterialSpec,
    ) -> SurfaceMaterialId {
        self.visuals.create_surface_material(spec)
    }

    fn create_curve_material(
        &mut self,
        spec: visual::material::CurveMaterialSpec,
    ) -> CurveMaterialId {
        self.visuals.create_curve_material(spec)
    }

    fn create_point_material(
        &mut self,
        spec: visual::material::PointMaterialSpec,
    ) -> PointMaterialId {
        self.visuals.create_point_material(spec)
    }

    fn set_surface_material(&mut self, surface: SurfaceId, material: SurfaceMaterialId) {
        self.visuals.set_surface_material(surface, material)
    }

    fn set_curve_material(&mut self, curve: CurveId, material: CurveMaterialId) {
        self.visuals.set_curve_material(curve, material)
    }

    fn set_point_material(&mut self, point: PointId, material: PointMaterialId) {
        self.visuals.set_point_material(point, material)
    }
}
