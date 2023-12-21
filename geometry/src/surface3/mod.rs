mod extrusion;
mod revolution;
mod sweep;
mod translation;

pub use extrusion::*;
pub use revolution::*;
pub use sweep::*;
pub use translation::*;

use space::{Angle, Coincidence, Point3, Vec2, Vec3};

use crate::Curve3;

pub enum Surface3 {
    Extrusion(Extrusion),
    Translation(Translation),
    Revolution(Revolution),
    Sweep(Sweep),
}
impl Surface3 {
    pub fn extrusion(profile: Curve3, direction: Vec3) -> Self {
        Self::Extrusion(Extrusion::new(profile, direction))
    }

    pub fn translation(profile: Curve3, path: Curve3) -> Self {
        Self::Translation(Translation::new(profile, path))
    }

    pub fn revolution(
        profile: Curve3,
        axis_origin: Point3,
        axis_direction: Vec3,
        start_angle: Angle,
        sweep_angle: Angle,
    ) -> Self {
        Self::Revolution(Revolution::new(
            profile,
            axis_origin,
            axis_direction,
            start_angle,
            sweep_angle,
        ))
    }

    pub fn sweep(profile: Curve3, path: Curve3) -> Self {
        Self::Sweep(Sweep::new(profile, path))
    }

    pub fn expect_sweep(self) -> Sweep {
        match self {
            Surface3::Sweep(sweep) => sweep,
            _ => panic!("Expected sweep"),
        }
    }
}
impl Surface3Impl for Surface3 {
    fn u_min(&self) -> f64 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.u_min(),
            Surface3::Translation(translation) => translation.u_min(),
            Surface3::Revolution(revolution) => revolution.u_min(),
            Surface3::Sweep(sweep) => sweep.u_min(),
        }
    }

    fn u_max(&self) -> f64 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.u_max(),
            Surface3::Translation(translation) => translation.u_max(),
            Surface3::Revolution(revolution) => revolution.u_max(),
            Surface3::Sweep(sweep) => sweep.u_max(),
        }
    }

    fn v_min(&self) -> f64 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.v_min(),
            Surface3::Translation(translation) => translation.v_min(),
            Surface3::Revolution(revolution) => revolution.v_min(),
            Surface3::Sweep(sweep) => sweep.v_min(),
        }
    }

    fn v_max(&self) -> f64 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.v_max(),
            Surface3::Translation(translation) => translation.v_max(),
            Surface3::Revolution(revolution) => revolution.v_max(),
            Surface3::Sweep(sweep) => sweep.v_max(),
        }
    }

    fn period_u(&self) -> Option<f64> {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.period_u(),
            Surface3::Translation(translation) => translation.period_u(),
            Surface3::Revolution(revolution) => revolution.period_u(),
            Surface3::Sweep(sweep) => sweep.period_u(),
        }
    }

    fn period_v(&self) -> Option<f64> {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.period_v(),
            Surface3::Translation(translation) => translation.period_v(),
            Surface3::Revolution(revolution) => revolution.period_v(),
            Surface3::Sweep(sweep) => sweep.period_v(),
        }
    }

    fn eval(&self, u: f64, v: f64) -> Point3 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.eval(u, v),
            Surface3::Translation(translation) => translation.eval(u, v),
            Surface3::Revolution(revolution) => revolution.eval(u, v),
            Surface3::Sweep(sweep) => sweep.eval(u, v),
        }
    }

    fn der1(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.der1(u, v),
            Surface3::Translation(translation) => translation.der1(u, v),
            Surface3::Revolution(revolution) => revolution.der1(u, v),
            Surface3::Sweep(sweep) => sweep.der1(u, v),
        }
    }

    fn der2(&self, u: f64, v: f64) -> (Vec3, Vec3, Vec3) {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.der2(u, v),
            Surface3::Translation(translation) => translation.der2(u, v),
            Surface3::Revolution(revolution) => revolution.der2(u, v),
            Surface3::Sweep(sweep) => sweep.der2(u, v),
        }
    }

    fn u_len(&self) -> f64 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.u_len(),
            Surface3::Translation(translation) => translation.u_len(),
            Surface3::Revolution(revolution) => revolution.u_len(),
            Surface3::Sweep(sweep) => sweep.u_len(),
        }
    }

    fn v_len(&self) -> f64 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.v_len(),
            Surface3::Translation(translation) => translation.v_len(),
            Surface3::Revolution(revolution) => revolution.v_len(),
            Surface3::Sweep(sweep) => sweep.v_len(),
        }
    }

    fn is_periodic_u(&self) -> bool {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.is_periodic_u(),
            Surface3::Translation(translation) => translation.is_periodic_u(),
            Surface3::Revolution(revolution) => revolution.is_periodic_u(),
            Surface3::Sweep(sweep) => sweep.is_periodic_u(),
        }
    }

    fn is_periodic_v(&self) -> bool {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.is_periodic_v(),
            Surface3::Translation(translation) => translation.is_periodic_v(),
            Surface3::Revolution(revolution) => revolution.is_periodic_v(),
            Surface3::Sweep(sweep) => sweep.is_periodic_v(),
        }
    }

    fn tangents(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.tangents(u, v),
            Surface3::Translation(translation) => translation.tangents(u, v),
            Surface3::Revolution(revolution) => revolution.tangents(u, v),
            Surface3::Sweep(sweep) => sweep.tangents(u, v),
        }
    }

    fn normal(&self, u: f64, v: f64) -> Vec3 {
        match self {
            Surface3::Extrusion(extrusion) => extrusion.normal(u, v),
            Surface3::Translation(translation) => translation.normal(u, v),
            Surface3::Revolution(revolution) => revolution.normal(u, v),
            Surface3::Sweep(sweep) => sweep.normal(u, v),
        }
    }
}

pub trait Surface3Impl {
    fn u_min(&self) -> f64;
    fn u_max(&self) -> f64;

    fn v_min(&self) -> f64;
    fn v_max(&self) -> f64;

    fn u_len(&self) -> f64 {
        self.u_max() - self.u_min()
    }

    fn v_len(&self) -> f64 {
        self.v_max() - self.v_min()
    }

    fn period_u(&self) -> Option<f64>;

    fn is_periodic_u(&self) -> bool {
        self.period_u().is_some()
    }

    fn period_v(&self) -> Option<f64>;

    fn is_periodic_v(&self) -> bool {
        self.period_v().is_some()
    }

    fn eval(&self, u: f64, v: f64) -> Point3;
    fn der1(&self, u: f64, v: f64) -> (Vec3, Vec3);
    fn der2(&self, u: f64, v: f64) -> (Vec3, Vec3, Vec3);

    fn tangents(&self, u: f64, v: f64) -> (Vec3, Vec3) {
        let (mut der1_u, mut der1_v) = self.der1(u, v);

        /*
        if der1_v.magnitude().cc(0.0) {
            let normal = der1_u.normalize().cross(der1_v.normalize());
            let fixed = normal.cross(der1_u).normalize();
            println!("fix dv {} -> {}", der1_v, fixed);
            der1_v = fixed;
        }

        if der1_u.magnitude().cc(0.0) {
            let normal = der1_u.normalize().cross(der1_v.normalize());
            let fixed = der1_v.cross(normal).normalize();
            println!("fix du {} -> {}", der1_u, fixed);
            der1_u = fixed;
        }
         */

        (der1_u.normalize(), der1_v.normalize())
    }

    fn normal(&self, u: f64, v: f64) -> Vec3 {
        let tangent = self.tangents(u, v);
        tangent.0.cross(tangent.1).normalize()
    }

    /// Returns the coefficients of the First Fundamental Form of
    /// the surface at (u, v))
    fn ff1(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let (du, dv) = self.der1(u, v);

        let e = du.dot(du);
        let f = du.dot(dv);
        let g = dv.dot(dv);

        (e, f, g)
    }

    /// Returns the coefficients of the Second Fundamental Form of
    /// the surface at (u, v))
    fn ff2(&self, u: f64, v: f64) -> (f64, f64, f64) {
        let (du, dv) = self.der1(u, v);
        let (duu, duv, dvv) = self.der2(u, v);

        let normal = dv.cross(du).normalize();

        let l = duu.dot(normal);
        let m = duv.dot(normal);
        let n = dvv.dot(normal);

        (l, m, n)
    }

    fn normal_curvature(&self, u: f64, v: f64, direction: Vec2) -> f64 {
        let (e, f, g) = self.ff1(u, v);
        let (l, m, n) = self.ff2(u, v);

        let du2 = direction.u().powi(2);
        let dudv = direction.u() * direction.v();
        let dv2 = direction.v().powi(2);

        (l * du2 + 2.0 * m * dudv + n * dv2) / (e * du2 + 2.0 * f * dudv + g * dv2)
    }

    fn mean_curvature(&self, u: f64, v: f64) -> f64 {
        let (e, f, g) = self.ff1(u, v);
        let (l, m, n) = self.ff2(u, v);

        0.5 * (e * n - 2.0 * f * m + g * l) / (e * g - f.powi(2))
    }

    fn gaussian_curvature(&self, u: f64, v: f64) -> f64 {
        let (e, f, g) = self.ff1(u, v);
        let (l, m, n) = self.ff2(u, v);

        (l * n - m.powi(2)) / (e * g - f.powi(2))
    }

    fn principal_curvatures(&self, u: f64, v: f64) -> (f64, f64) {
        let h = self.mean_curvature(u, v);
        let k = self.gaussian_curvature(u, v);

        let root = (h.powi(2) - k).sqrt();

        (h + root, h - root)
    }
}

#[cfg(test)]
mod tests {
    use space::lerp;

    use crate::test::{validate_der1_curve, validate_der_1d};

    use super::*;

    /// Validates the first derivative of a function with a 2-dimensional input space
    pub fn validate_der1_2d<F: Fn(f64, f64) -> Vec3, D: Fn(f64, f64) -> (Vec3, Vec3)>(
        function: F,
        derivative: D,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        samples: usize,
        tolerance: f64,
    ) {
        // Move along U, validating the first derivative of each curve along V
        for i in 0..samples {
            // Define u for the current sample
            let u = lerp(u_min, u_max, i as f64 / (samples - 1) as f64);

            validate_der_1d(
                |v| function(u, v),
                |v| derivative(u, v).1,
                v_min,
                v_max,
                samples,
                tolerance,
                &format!("First derivative with respect to V at U = {}", u),
            );
        }

        // Move along V, validating the first derivative of each curve along U
        for i in 0..samples {
            // Define u for the current sample
            let v = lerp(v_min, v_max, i as f64 / (samples - 1) as f64);

            validate_der_1d(
                |u| function(u, v),
                |u| derivative(u, v).0,
                u_min,
                u_max,
                samples,
                tolerance,
                &format!("First derivative with respect to U at V = {}", v),
            );
        }
    }

    /// Validates the second derivatives of a function with a 2-dimensional input space
    pub fn validate_der2_2d<
        F: Fn(f64, f64) -> (Vec3, Vec3),
        D: Fn(f64, f64) -> (Vec3, Vec3, Vec3),
    >(
        function: F,
        derivative: D,
        u_min: f64,
        u_max: f64,
        v_min: f64,
        v_max: f64,
        samples: usize,
        tolerance: f64,
    ) {
        // Move along U, validating the second derivative of each curve along V (dvv)
        for i in 0..samples {
            // Define u for the current sample
            let u = lerp(u_min, u_max, i as f64 / (samples - 1) as f64);

            validate_der_1d(
                |v| function(u, v).1,
                |v| derivative(u, v).2,
                //|v| function(u, v).0,
                //|v| derivative(u, v).0,
                v_min,
                v_max,
                samples,
                tolerance,
                &format!("Second derivative with respect to VV at U = {}", u),
            );
        }

        // Move along V, validating the second derivative of each curve along U (duu)
        for i in 0..samples {
            // Define u for the current sample
            let v = lerp(v_min, v_max, i as f64 / (samples - 1) as f64);

            validate_der_1d(
                |u| function(u, v).0,
                |u| derivative(u, v).0,
                u_min,
                u_max,
                samples,
                tolerance,
                &format!("Second derivative with respect to UU at V = {}", v),
            );
        }
    }

    pub fn validate_ders_2d<S: Surface3Impl>(surface: &S, samples: usize, tolerance: f64) {
        validate_der1_2d(
            |u, v| surface.eval(u, v).into_vec(),
            |u, v| surface.der1(u, v),
            surface.u_min(),
            surface.u_max(),
            surface.v_min(),
            surface.v_max(),
            samples,
            tolerance,
        );

        validate_der2_2d(
            |u, v| surface.der1(u, v),
            |u, v| surface.der2(u, v),
            surface.u_min(),
            surface.u_max(),
            surface.v_min(),
            surface.v_max(),
            samples,
            tolerance,
        );
    }
}
