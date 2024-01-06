use crate::{
    math::{Angle, Point3, Vec3},
    primitives::{CurvePoint, CurveSolver},
};

use super::TessellationTolerance;

#[derive(Clone, Debug)]
pub struct CurveSample {
    pub u: f64,
    pub pos: Point3,
    pub der1: Vec3,
    pub der2: Vec3,
}
impl PartialEq for CurveSample {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u
    }
}
impl Eq for CurveSample {}
impl PartialOrd for CurveSample {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.u.partial_cmp(&other.u)
    }
}
impl Ord for CurveSample {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.u.total_cmp(&other.u)
    }
}

#[derive(Clone, Debug)]
pub struct TessellatedCurve {
    pub points: Vec<CurveSample>,
}
impl TessellatedCurve {
    pub fn create(curve: &CurveSolver, tolerance: &TessellationTolerance) -> Self {
        let (min_u, max_u) = curve.domain();
        let mut points: Vec<CurvePoint> = vec![];

        loop {
            if points.len() == 0 {
                points.push(curve.point(min_u));
            } else {
                let last_point = &points[points.len() - 1];
                let next_u = last_point.u() + Self::delta(last_point, tolerance);
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
                .map(|p| CurveSample {
                    u: p.u(),
                    pos: *p.pos(),
                    der1: *p.der1(),
                    der2: *p.der2(),
                })
                .collect(),
        }
    }

    fn delta(point: &CurvePoint, tolerance: &TessellationTolerance) -> f64 {
        match tolerance {
            TessellationTolerance::Distance(distance) => Self::delta_dist(point, *distance),
            TessellationTolerance::Angle(angle) => Self::delta_angle(point, *angle),
            TessellationTolerance::DistanceAndAngle(distance, angle) => {
                Self::delta_dist(point, *distance).min(Self::delta_angle(point, *angle))
            }
        }
    }

    fn delta_angle(point: &CurvePoint, angle: Angle) -> f64 {
        let d1 = point.der1();
        let k = point.curvature();
        angle.radians() / (k * d1.magnitude())
    }

    fn delta_dist(point: &CurvePoint, dist: f64) -> f64 {
        let der1 = *point.der1();
        let p = point.curvature().recip();
        2.0 * (dist * (2.0 * p - dist)).sqrt() / der1.magnitude()
    }
}
