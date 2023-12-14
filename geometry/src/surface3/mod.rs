mod extrusion;
mod revolution;
mod sweep;
mod translation;

pub use extrusion::*;
pub use revolution::*;
pub use sweep::*;
pub use translation::*;

use space::{Point3, Vec3};

use crate::Curve3;

pub enum Surface3 {
    Extrusion(Extrusion),
    Translation(Translation),
    Revolution(Revolution),
    Sweep(Sweep),
}
impl Surface3 {
    pub fn sweep(profile: Curve3, path: Curve3) -> Self {
        Self::Sweep(Sweep::new(profile, path))
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
        let (der1_u, der1_v) = self.der1(u, v);
        (der1_u.normalize(), der1_v.normalize())
    }

    fn normal(&self, u: f64, v: f64) -> Vec3 {
        let tangent = self.tangents(u, v);
        tangent.0.cross(tangent.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn validate_der1<S: Surface3Impl>(surface: &S, samples: usize, tolerance: f64) {
        for i_u in 0..samples {
            for i_v in 0..samples {
                // Define parameters for the current sample
                let u = i_u as f64 / (samples - 1) as f64;
                let v = i_v as f64 / (samples - 1) as f64;

                // Calculate the exact first derivative at (u, v)
                let computed_der1 = surface.der1(u, v);

                // Test convergence on U
                {
                    // U-parameter deviation that we start checking the sample with
                    let mut u_deviation = surface.u_len() / 10.0;

                    // Flag for whether the approximated first derivative below gets close enough to
                    // the computed derivative at U
                    let mut u_converged = false;

                    // Last approximate derivative that does not contain NaNs
                    let mut last_notnan_approx = Vec3::ZERO;

                    // Iteratively approximate the derivative with respect to U by getting the vector
                    // between two points on the curve centered around u, decreasing their distance from u each time.
                    for _ in 0..64 {
                        // Get parameters above and below u, clamped between 0 and 1
                        let u_lo = (u - u_deviation).clamp(0.0, 1.0);
                        let u_hi = (u + u_deviation).clamp(0.0, 1.0);

                        // Evaluate the curve at those parameters
                        let lo_pos = surface.eval(u_lo, v);
                        let hi_pos = surface.eval(u_hi, v);

                        // Approximate the derivative by getting a vector between those two points
                        // and scaling it by the parameter distance between them
                        let converged_der1 = (hi_pos - lo_pos) / (u_hi - u_lo);

                        if !converged_der1.has_nan() {
                            last_notnan_approx = converged_der1;
                        }

                        // Get the difference between the exact derivative vector and the approximated one
                        let dist = (computed_der1.0 - converged_der1).magnitude();

                        // If the distance is within tolerance, we consider the exact derivative
                        // calculation to be valid and stop iteration for this sample.
                        if dist < tolerance {
                            u_converged = true;
                            break;
                        }

                        // If we haven't converged yet, reduce the deviation from u.
                        u_deviation /= 2.0;
                    }

                    // Panic if we never got close enough to the exact derivative calculation.
                    if !u_converged {
                        panic!(
                            "Derivative 1 @ u = {} computed as {}, but converged to {}, outside tolerance {}",
                            u, computed_der1.0, last_notnan_approx, tolerance
                        );
                    }
                }

                // Test convergence on V
                {
                    // V-parameter deviation that we start checking the sample with
                    let mut v_deviation = surface.u_len() / 10.0;

                    // Flag for whether the approximated first derivative below gets close enough to
                    // the computed derivative at V
                    let mut v_converged = false;

                    // Last approximate derivative that does not contain NaNs
                    let mut last_notnan_approx = Vec3::ZERO;

                    // Iteratively approximate the derivative with respect to U by getting the vector
                    // between two points on the curve centered around u, decreasing their distance from u each time.
                    for _ in 0..64 {
                        // Get parameters above and below u, clamped between 0 and 1
                        let v_lo = (v - v_deviation).clamp(0.0, 1.0);
                        let v_hi = (v + v_deviation).clamp(0.0, 1.0);

                        // Evaluate the curve at those parameters
                        let lo_pos = surface.eval(u, v_lo);
                        let hi_pos = surface.eval(u, v_hi);

                        // Approximate the derivative by getting a vector between those two points
                        // and scaling it by the parameter distance between them
                        let converged_der1 = (hi_pos - lo_pos) / (v_hi - v_lo);

                        if !converged_der1.has_nan() {
                            last_notnan_approx = converged_der1;
                        }

                        // Get the difference between the exact derivative vector and the approximated one
                        let dist = (computed_der1.1 - converged_der1).magnitude();

                        // If the distance is within tolerance, we consider the exact derivative
                        // calculation to be valid and stop iteration for this sample.
                        if dist < tolerance {
                            v_converged = true;
                            break;
                        }

                        // If we haven't converged yet, reduce the deviation from u.
                        v_deviation /= 2.0;
                    }

                    // Panic if we never got close enough to the exact derivative calculation.
                    if !v_converged {
                        panic!(
                            "Derivative 1 @ v = {} computed as {}, but converged to {}, outside tolerance {}",
                            v, computed_der1.0, last_notnan_approx, tolerance
                        );
                    }
                }
            }
        }
    }
}
