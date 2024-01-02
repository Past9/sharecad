use geometry::primitives::Curve;
use render::model::CurveMesh;
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
pub struct CurveTesselator<'a> {
    curve: &'a Curve,
    points: BTreeSet<CurveVert>,
}
impl<'a> CurveTesselator<'a> {
    pub fn new(curve: &'a Curve) -> Self {
        let points = BTreeSet::from_iter([
            CurveVert {
                u: curve.domain().0,
                pos: *curve.point(curve.domain().0).eval(),
            },
            CurveVert {
                u: curve.domain().1,
                pos: *curve.point(curve.domain().1).eval(),
            },
        ]);

        Self { curve, points }
    }

    pub fn mesh(&self) -> CurveMesh {
        CurveMesh::new(self.points.iter().map(|v| v.pos).collect())
    }

    pub fn curve(&self) -> &Curve {
        &self.curve
    }

    pub fn vertices(&self) -> &BTreeSet<CurveVert> {
        &self.points
    }

    pub fn insert_with_pos(&mut self, u: f64, pos: Point3) {
        self.points.insert(CurveVert { u, pos });
    }

    pub fn insert(&mut self, u: f64) {
        self.insert_with_pos(u, *self.curve.point(u).eval());
    }

    pub fn tessellate(&mut self, tolerance: f64) {
        let (mut u, u_max) = self.curve.domain();
        let mut params = vec![];

        while u < u_max {
            u += self.delta_u(u, tolerance);
            if u < u_max {
                params.push(u);
            }
        }

        for param in params.into_iter() {
            self.insert(param);
        }
    }

    pub fn delta_u(&mut self, u: f64, tolerance: f64) -> f64 {
        let point = self.curve.point(u);
        let der1 = *point.der1();
        let der2 = *point.der2();

        let p = der1.magnitude().powi(3) / (der1.cross(der2)).magnitude();

        2.0 * (tolerance * (2.0 * p - tolerance)).sqrt() / der1.magnitude()
    }
}
