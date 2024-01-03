use geometry::primitives::{CurvePoint, CurveSolver};
use space::Point3;

#[derive(Clone, Debug)]
pub struct CurveVert {
    pub u: f64,
    pub pos: Point3,
}
impl PartialEq for CurveVert {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u
    }
}
impl Eq for CurveVert {}
impl PartialOrd for CurveVert {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.u.partial_cmp(&other.u)
    }
}
impl Ord for CurveVert {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.u.total_cmp(&other.u)
    }
}

pub struct TessellatedCurve {
    pub points: Vec<CurveVert>,
}
impl TessellatedCurve {
    pub fn by_tolerance(curve: &CurveSolver, tolerance: f64) -> Self {
        let (min_u, max_u) = curve.domain();
        let mut points: Vec<CurvePoint> = vec![];

        loop {
            if points.len() == 0 {
                points.push(curve.point(min_u));
            } else {
                let last_point = &points[points.len() - 1];
                let delta = Self::delta_u(last_point, tolerance);
                let next_u = last_point.u() + delta;
                if next_u < max_u {
                    points.push(curve.point(next_u));
                } else {
                    points.push(curve.point(max_u));
                    break;
                }
            }
        }

        Self {
            points: points
                .into_iter()
                .map(|p| CurveVert {
                    u: p.u(),
                    pos: *p.eval(),
                })
                .collect(),
        }
    }

    fn delta_u(point: &CurvePoint, tolerance: f64) -> f64 {
        let der1 = *point.der1();
        let der2 = *point.der2();

        let p = der1.magnitude().powi(3) / (der1.cross(der2)).magnitude();

        2.0 * (tolerance * (2.0 * p - tolerance)).sqrt() / der1.magnitude()
    }
}
