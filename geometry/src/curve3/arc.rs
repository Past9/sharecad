use std::f64::consts::TAU;

use space::{point3, vec3, Angle, Quat, Vec3};

use crate::Curve3Impl;

#[derive(Debug, Clone)]
pub struct Arc {
    r: f64,
    angle: Angle,
    orientation: Quat,
    translation: Vec3,
}
impl Arc {
    pub fn new(r: f64, angle: Angle, orientation: Quat, translation: Vec3) -> Self {
        Self {
            r,
            angle,
            orientation,
            translation,
        }
    }
}
impl Curve3Impl for Arc {
    fn u_min(&self) -> f64 {
        0.0
    }

    fn u_max(&self) -> f64 {
        self.angle.radians()
    }

    fn period(&self) -> Option<f64> {
        Some(TAU)
    }

    fn eval(&self, u: f64) -> space::Point3 {
        let point = point3(self.r * u.cos(), self.r * u.sin(), 0.0);
        self.orientation * point + self.translation
    }

    fn der1(&self, u: f64) -> Vec3 {
        let der1 = vec3(self.r * -u.sin(), self.r * u.cos(), 0.0);
        self.orientation * der1
    }

    fn der2(&self, u: f64) -> Vec3 {
        let der2 = vec3(self.r * -u.cos(), self.r * -u.sin(), 0.0);
        self.orientation * der2
    }

    fn der3(&self, u: f64) -> Vec3 {
        let der3 = vec3(self.r * u.sin(), self.r * -u.cos(), 0.0);
        self.orientation * der3
    }
}

#[cfg(test)]
mod tests {
    use super::Arc;
    use crate::curve3::tests::validate_ders_1d;
    use space::{deg, vec3, Quat};

    fn test_arc() -> Arc {
        Arc::new(
            1.0,
            deg(360.0),
            Quat::from_axis_angle(vec3(1.0, 0.0, 0.0), deg(90.0)),
            vec3(1.0, 2.0, 3.0),
        )
    }

    #[test]
    fn arc_validate_ders() {
        validate_ders_1d(&test_arc(), 100, 1e-7);
    }
}
