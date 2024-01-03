use geometry::primitives::CurveSolver;
use space::Point3;
use std::collections::BTreeSet;

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
        let mut points: BTreeSet<CurveVert> = BTreeSet::new();

        let (mut u, u_max) = curve.domain();
        let mut params = vec![];

        while u < u_max {
            u += Self::delta_u(curve, u, tolerance);
            if u < u_max {
                params.push(u);
            }
        }

        for param in params.into_iter() {
            points.insert(CurveVert {
                u: param,
                pos: *curve.point(u).eval(),
            });
        }

        Self {
            points: points.into_iter().collect(),
        }
    }

    fn delta_u(curve: &CurveSolver, u: f64, tolerance: f64) -> f64 {
        let point = curve.point(u);
        let der1 = *point.der1();
        let der2 = *point.der2();

        let p = der1.magnitude().powi(3) / (der1.cross(der2)).magnitude();

        2.0 * (tolerance * (2.0 * p - tolerance)).sqrt() / der1.magnitude()
    }
}
