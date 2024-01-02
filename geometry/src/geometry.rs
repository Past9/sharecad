use std::collections::HashMap;

use common::{CurveId, IdSeries, PointId, SurfaceId};
use space::Point3;

use crate::primitives::{Curve, Surface};

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

    pub fn add_curve(&mut self, curve: Curve) -> CurveId {
        let id = self.curve_ids.next();
        self.curves.insert(id, curve);
        id
    }

    pub fn add_point(&mut self, point: Point3) -> PointId {
        let id = self.point_ids.next();
        self.points.insert(id, point);
        id
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
