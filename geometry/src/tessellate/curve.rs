use crate::{
    math::{Angle, Point3},
    primitives::{CurvePoint, CurveSolver},
};

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

pub enum TessellationTolerance {
    Distance(f64),
    Angle(Angle),
    DistanceAndAngle(f64, Angle),
}

pub struct TessellatedCurve {
    pub points: Vec<CurveVert>,
}
impl TessellatedCurve {
    pub fn create(curve: &CurveSolver, tolerance: TessellationTolerance) -> Self {
        let (min_u, max_u) = curve.domain();
        let mut points: Vec<CurvePoint> = vec![];

        loop {
            if points.len() == 0 {
                points.push(curve.point(min_u));
            } else {
                let last_point = &points[points.len() - 1];

                let delta = match tolerance {
                    TessellationTolerance::Distance(distance) => {
                        Self::delta_u_dist(last_point, distance)
                    }
                    TessellationTolerance::Angle(angle) => Self::delta_u_angle(last_point, angle),
                    TessellationTolerance::DistanceAndAngle(distance, angle) => {
                        Self::delta_u_dist(last_point, distance)
                            .min(Self::delta_u_angle(last_point, angle))
                    }
                };

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

    fn delta_u_angle(point: &CurvePoint, angle: Angle) -> f64 {
        let p = point.curvature().recip();
        let d_mag = point.der1().magnitude();
        (p * angle.radians()) / d_mag
    }

    fn delta_u_dist(point: &CurvePoint, dist: f64) -> f64 {
        let der1 = *point.der1();
        let der2 = *point.der2();

        let p = der1.magnitude().powi(3) / (der1.cross(der2)).magnitude();

        2.0 * (dist * (2.0 * p - dist)).sqrt() / der1.magnitude()
    }
}
