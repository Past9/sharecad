use std::collections::HashMap;

use crate::{
    math::{Angle, Quat, Scalar, Vec3},
    primitives::{Point, SSCurve, SSCurveParams},
};
use common::{CurveId, IdSeries, PointId, SurfaceId};

use crate::primitives::{Arc, Curve, CurveSolver, Helix, Line, Surface, SurfaceSolver, Sweep};

pub trait IGeometry<S: Scalar> {
    fn create_point(&mut self, point: Point<S>) -> PointId;
    fn create_curve(&mut self, curve: Curve<S>) -> CurveId;
    fn create_surface(&mut self, surface: Surface<S>) -> SurfaceId;
    fn create_point3(&mut self, point: Vec3<S>) -> PointId;
    fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId;
    fn create_helix(
        &mut self,
        r: S,
        h: S,
        n: S,
        orientation: Quat<S>,
        translation: Vec3<S>,
    ) -> CurveId;
    fn create_arc(
        &mut self,
        r: S,
        angle: Angle<S>,
        orientation: Quat<S>,
        translation: Vec3<S>,
    ) -> CurveId;
    fn create_ss_curve(
        &mut self,
        s0: SurfaceId,
        s1: SurfaceId,
        points: Vec<SSCurveParams<S>>,
    ) -> CurveId;
    fn create_sweep(&mut self, profile: CurveId, path: CurveId) -> SurfaceId;
    fn point(&self, id: PointId) -> Option<&Point<S>>;
    fn curve(&self, id: CurveId) -> Option<&Curve<S>>;
    fn surface(&self, id: SurfaceId) -> Option<&Surface<S>>;
    fn curve_solver(&self, id: CurveId) -> Option<CurveSolver<S>>;
    fn surface_solver(&self, id: SurfaceId) -> Option<SurfaceSolver<S>>;
    fn surfaces(&self) -> &HashMap<SurfaceId, Surface<S>>;
    fn curves(&self) -> &HashMap<CurveId, Curve<S>>;
    fn points(&self) -> &HashMap<PointId, Point<S>>;
}

#[derive(Debug)]
pub struct PrimitiveGeometry<S: Scalar> {
    surfaces: HashMap<SurfaceId, Surface<S>>,
    curves: HashMap<CurveId, Curve<S>>,
    points: HashMap<PointId, Point<S>>,

    surface_ids: IdSeries<SurfaceId>,
    curve_ids: IdSeries<CurveId>,
    point_ids: IdSeries<PointId>,
}
impl<S: Scalar> PrimitiveGeometry<S> {
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
impl<S: Scalar> IGeometry<S> for PrimitiveGeometry<S> {
    fn create_point(&mut self, point: Point<S>) -> PointId {
        let id = self.point_ids.next();
        self.points.insert(id, point);
        id
    }

    fn create_curve(&mut self, curve: Curve<S>) -> CurveId {
        let id = self.curve_ids.next();
        self.curves.insert(id, curve);
        id
    }

    fn create_surface(&mut self, surface: Surface<S>) -> SurfaceId {
        let id = self.surface_ids.next();
        self.surfaces.insert(id, surface);
        id
    }

    fn create_point3(&mut self, point3: Vec3<S>) -> PointId {
        self.create_point(Point::Point(point3))
    }

    fn create_line_between(&mut self, start: PointId, end: PointId) -> CurveId {
        self.create_curve(Line::new(start, end).into())
    }

    fn create_helix(
        &mut self,
        r: S,
        h: S,
        n: S,
        orientation: Quat<S>,
        translation: Vec3<S>,
    ) -> CurveId {
        self.create_curve(Helix::new(r, h, n, orientation, translation).into())
    }

    fn create_arc(
        &mut self,
        r: S,
        angle: Angle<S>,
        orientation: Quat<S>,
        translation: Vec3<S>,
    ) -> CurveId {
        self.create_curve(Arc::new(r, angle, orientation, translation).into())
    }

    fn create_ss_curve(
        &mut self,
        s0: SurfaceId,
        s1: SurfaceId,
        points: Vec<SSCurveParams<S>>,
    ) -> CurveId {
        self.create_curve(SSCurve::new(s0, s1, points).into())
    }

    fn create_sweep(&mut self, profile: CurveId, path: CurveId) -> SurfaceId {
        let id = self.surface_ids.next();
        let sweep = Sweep::new(profile, path);
        self.surfaces.insert(id, sweep.into());
        id
    }

    fn point(&self, id: PointId) -> Option<&Point<S>> {
        self.points.get(&id)
    }

    fn curve(&self, id: CurveId) -> Option<&Curve<S>> {
        self.curves.get(&id)
    }

    fn curve_solver(&self, id: CurveId) -> Option<CurveSolver<S>> {
        self.curves.get(&id).map(|c| c.solver(self))
    }

    fn surface(&self, id: SurfaceId) -> Option<&Surface<S>> {
        self.surfaces.get(&id)
    }

    fn surface_solver(&self, id: SurfaceId) -> Option<SurfaceSolver<S>> {
        self.surfaces.get(&id).map(|s| s.solver(self))
    }

    fn surfaces(&self) -> &HashMap<SurfaceId, Surface<S>> {
        &self.surfaces
    }

    fn curves(&self) -> &HashMap<CurveId, Curve<S>> {
        &self.curves
    }

    fn points(&self) -> &HashMap<PointId, Point<S>> {
        &self.points
    }
}
