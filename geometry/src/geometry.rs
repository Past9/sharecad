use std::collections::HashMap;

use common::{CurveId, IdSeries, PointId, SurfaceId};
use space::{point3, Angle, Point3, Quat, Vec3};

use crate::primitives::{Arc, Curve, CurveSolver, Line, Surface, SurfaceSolver, Sweep};

fn test_geometry() {
    let mut geom = PrimitiveGeometry::new();

    let profile_start = geom.create_point(Point3::ZERO);
    let profile_end = geom.create_point(point3(0.0, 1.0, 0.0));
    let profile = geom.create_line_between(profile_start, profile_end);

    let path_start = geom.create_point(Point3::ZERO);
    let path_end = geom.create_point(point3(0.0, 0.0, 3.0));
    let path = geom.create_line_between(path_start, path_end);

    let sweep = geom.create_sweep(profile, path);
}

pub trait IGeometry {
    fn create_point(&mut self, point: Point3) -> PointId;
    fn create_curve(&mut self, curve: Curve) -> CurveId;
    fn create_surface(&mut self, surface: Surface) -> SurfaceId;
    fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId;
    fn create_arc(&mut self, r: f64, angle: Angle, orientation: Quat, translation: Vec3)
        -> CurveId;
    fn create_sweep(&mut self, profile: CurveId, path: CurveId) -> SurfaceId;
    fn point(&self, id: PointId) -> Option<&Point3>;
    fn curve(&self, id: CurveId) -> Option<&Curve>;
    fn surface(&self, id: SurfaceId) -> Option<&Surface>;
    fn curve_solver(&self, id: CurveId) -> Option<CurveSolver>;
    fn surface_solver(&self, id: SurfaceId) -> Option<SurfaceSolver>;
    fn surfaces(&self) -> &HashMap<SurfaceId, Surface>;
    fn curves(&self) -> &HashMap<CurveId, Curve>;
    fn points(&self) -> &HashMap<PointId, Point3>;
}

#[derive(Debug)]
pub struct PrimitiveGeometry {
    surfaces: HashMap<SurfaceId, Surface>,
    curves: HashMap<CurveId, Curve>,
    points: HashMap<PointId, Point3>,

    surface_ids: IdSeries<SurfaceId>,
    curve_ids: IdSeries<CurveId>,
    point_ids: IdSeries<PointId>,
}
impl PrimitiveGeometry {
    pub fn new() -> Self {
        Self {
            surfaces: HashMap::new(),
            curves: HashMap::new(),
            points: HashMap::new(),

            surface_ids: IdSeries::new(),
            curve_ids: IdSeries::new(),
            point_ids: IdSeries::new(),
        }
    }
}
impl IGeometry for PrimitiveGeometry {
    fn create_point(&mut self, point: Point3) -> PointId {
        let id = self.point_ids.next();
        self.points.insert(id, point);
        id
    }

    fn create_curve(&mut self, curve: Curve) -> CurveId {
        let id = self.curve_ids.next();
        self.curves.insert(id, curve);
        id
    }

    fn create_surface(&mut self, surface: Surface) -> SurfaceId {
        let id = self.surface_ids.next();
        self.surfaces.insert(id, surface);
        id
    }

    fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId {
        self.create_curve(Line::new(start, end).into())
    }

    fn create_arc(
        &mut self,
        r: f64,
        angle: Angle,
        orientation: Quat,
        translation: Vec3,
    ) -> CurveId {
        self.create_curve(Arc::new(r, angle, orientation, translation).into())
    }

    fn create_sweep(&mut self, profile: CurveId, path: CurveId) -> SurfaceId {
        let id = self.surface_ids.next();
        let sweep = Sweep::new(profile, path);
        self.surfaces.insert(id, sweep.into());
        id
    }

    fn point(&self, id: PointId) -> Option<&Point3> {
        self.points.get(&id)
    }

    fn curve(&self, id: CurveId) -> Option<&Curve> {
        self.curves.get(&id)
    }

    fn curve_solver(&self, id: CurveId) -> Option<CurveSolver> {
        self.curves.get(&id).map(|c| c.solver(self))
    }

    fn surface(&self, id: SurfaceId) -> Option<&Surface> {
        self.surfaces.get(&id)
    }

    fn surface_solver(&self, id: SurfaceId) -> Option<SurfaceSolver> {
        self.surfaces.get(&id).map(|s| s.solver(self))
    }

    fn surfaces(&self) -> &HashMap<SurfaceId, Surface> {
        &self.surfaces
    }

    fn curves(&self) -> &HashMap<CurveId, Curve> {
        &self.curves
    }

    fn points(&self) -> &HashMap<PointId, Point3> {
        &self.points
    }
}
