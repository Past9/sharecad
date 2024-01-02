use std::collections::HashMap;

use common::{CurveId, IdSeries, PointId, SurfaceId};
use space::Point3;

use crate::primitives::{Curve, Surface};

struct Geometry {
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
}
