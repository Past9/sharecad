use std::{collections::HashMap, os::windows::ffi::OsStringExt};

use common::{CurveId, IdSeries, PointId, SurfaceId};
use space::{point3, Angle, Point3, Quat, Vec3};

use crate::primitives::{Arc, Curve, CurveSolver, Line, Surface, SurfaceSolver, Sweep};

/*
fn test_ref_geometry() {
    let mut geometry = RefGeometry::new();
    let p0 = geometry.create_point(Point3::ZERO);
    let p1 = geometry.create_point(point3(0.0, 1.0, 0.0));
    let line = geometry.create_line_between(p0, p1);
}

pub struct RefGeometry {
    points: HashMap<PointId, Point3>,
    curves: HashMap<CurveId, RefCurve>,

    curve_ids: IdSeries<CurveId>,
    point_ids: IdSeries<PointId>,
}
impl RefGeometry {
    pub fn new() -> Self {
        Self {
            curves: HashMap::new(),
            points: HashMap::new(),

            curve_ids: IdSeries::new(),
            point_ids: IdSeries::new(),
        }
    }

    pub fn create_point(&mut self, point: Point3) -> PointId {
        let id = self.point_ids.next();
        self.points.insert(id, point);
        id
    }

    pub fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId {
        let id = self.curve_ids.next();
        let line = RefCurve::Line(RefLine::new(id, start, end));
        self.create_curve(line)
    }

    pub fn create_curve(&mut self, curve: RefCurve) -> CurveId {
        let id = self.curve_ids.next();
        self.curves.insert(id, curve);
        id
    }

    pub fn get_point(&self, id: PointId) -> Option<&Point3> {
        self.points.get(&id)
    }
}
*/

fn test_geometry() {
    let mut geom = Geometry::new();

    let profile_start = geom.create_point(Point3::ZERO);
    let profile_end = geom.create_point(point3(0.0, 1.0, 0.0));
    let profile = geom.create_line_between(profile_start, profile_end);

    let path_start = geom.create_point(Point3::ZERO);
    let path_end = geom.create_point(point3(0.0, 0.0, 3.0));
    let path = geom.create_line_between(path_start, path_end);

    let sweep = geom.create_sweep(profile, path);
}

#[derive(Debug)]
pub struct Geometry {
    surfaces: HashMap<SurfaceId, Surface>,
    curves: HashMap<CurveId, Curve>,
    points: HashMap<PointId, Point3>,

    surface_ids: IdSeries<SurfaceId>,
    curve_ids: IdSeries<CurveId>,
    point_ids: IdSeries<PointId>,
}
impl Geometry {
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

    pub fn add_surface(&mut self, surface: Surface) -> SurfaceId {
        let id = self.surface_ids.next();
        self.surfaces.insert(id, surface);
        id
    }

    pub fn create_point(&mut self, point: Point3) -> PointId {
        let id = self.point_ids.next();
        self.points.insert(id, point);
        id
    }

    pub fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId {
        let id = self.curve_ids.next();
        let line = Line::new(start, end);
        self.curves.insert(id, line.into());
        id
    }

    pub fn create_arc(
        &mut self,
        r: f64,
        angle: Angle,
        orientation: Quat,
        translation: Vec3,
    ) -> CurveId {
        let id = self.curve_ids.next();
        let arc = Arc::new(r, angle, orientation, translation);
        self.curves.insert(id, arc.into());
        id
    }

    pub fn create_sweep(&mut self, profile: CurveId, path: CurveId) -> SurfaceId {
        let id = self.surface_ids.next();
        let sweep = Sweep::new(profile, path);
        self.surfaces.insert(id, sweep.into());
        id
    }

    pub fn point(&self, id: PointId) -> Option<&Point3> {
        self.points.get(&id)
    }

    pub fn curve(&self, id: CurveId) -> Option<&Curve> {
        self.curves.get(&id)
    }

    pub fn curve_solver(&self, id: CurveId) -> Option<CurveSolver> {
        self.curves.get(&id).map(|c| c.solver(self))
    }

    pub fn surface(&self, id: SurfaceId) -> Option<&Surface> {
        self.surfaces.get(&id)
    }

    pub fn surface_solver(&self, id: SurfaceId) -> Option<SurfaceSolver> {
        self.surfaces.get(&id).map(|s| s.solver(self))
    }

    pub fn surfaces(&self) -> &HashMap<SurfaceId, Surface> {
        &self.surfaces
    }

    pub fn curves(&self) -> &HashMap<CurveId, Curve> {
        &self.curves
    }

    pub fn points(&self) -> &HashMap<PointId, Point3> {
        &self.points
    }
}
