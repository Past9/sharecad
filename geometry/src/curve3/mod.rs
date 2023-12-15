mod arc;
mod helix;
mod line;

pub use arc::*;
pub use helix::*;
pub use line::*;
use space::{Angle, Mat33, Point3, Quat, Vec3};

#[derive(Debug, Clone)]
pub enum Curve3 {
    Arc(Arc),
    Helix(Helix),
    Line(Line),
}
impl Curve3 {
    pub fn arc(r: f64, angle: Angle, orientation: Quat, translation: Vec3) -> Self {
        Self::Arc(Arc::new(r, angle, orientation, translation))
    }

    pub fn helix(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        Self::Helix(Helix::new(r, h, n, orientation, translation))
    }

    pub fn line(start: Point3, end: Point3) -> Self {
        Self::Line(Line::new(start, end))
    }

    pub fn curvature(&self, u: f64) -> f64 {
        let der1 = self.der1(u);
        let der2 = self.der2(u);

        (der1.cross(der2)).magnitude() / der1.magnitude().powi(3)
    }

    pub fn param_segments(&self, segments: u32, include_ends: bool) -> Vec<f64> {
        let increment = self.u_len() / segments as f64;

        let mut params = Vec::with_capacity(match include_ends {
            true => segments + 1,
            false => segments - 1,
        } as usize);

        if include_ends {
            params.push(self.u_min());
        }

        for i in 1..segments {
            params.push(increment * i as f64);
        }

        if include_ends {
            params.push(self.u_max());
        }

        params
    }
}
impl Curve3Impl for Curve3 {
    fn u_min(&self) -> f64 {
        match self {
            Curve3::Arc(arc) => arc.u_min(),
            Curve3::Helix(helix) => helix.u_min(),
            Curve3::Line(line) => line.u_min(),
        }
    }

    fn u_max(&self) -> f64 {
        match self {
            Curve3::Arc(arc) => arc.u_max(),
            Curve3::Helix(helix) => helix.u_max(),
            Curve3::Line(line) => line.u_max(),
        }
    }

    fn eval(&self, u: f64) -> Point3 {
        match self {
            Curve3::Arc(arc) => arc.eval(u),
            Curve3::Helix(helix) => helix.eval(u),
            Curve3::Line(line) => line.eval(u),
        }
    }

    fn der1(&self, u: f64) -> Vec3 {
        match self {
            Curve3::Arc(arc) => arc.der1(u),
            Curve3::Helix(helix) => helix.der1(u),
            Curve3::Line(line) => line.der1(u),
        }
    }

    fn der2(&self, u: f64) -> Vec3 {
        match self {
            Curve3::Arc(arc) => arc.der2(u),
            Curve3::Helix(helix) => helix.der2(u),
            Curve3::Line(line) => line.der2(u),
        }
    }

    fn der3(&self, u: f64) -> Vec3 {
        match self {
            Curve3::Arc(arc) => arc.der3(u),
            Curve3::Helix(helix) => helix.der3(u),
            Curve3::Line(line) => line.der3(u),
        }
    }

    fn period(&self) -> Option<f64> {
        match self {
            Curve3::Arc(arc) => arc.period(),
            Curve3::Helix(helix) => helix.period(),
            Curve3::Line(line) => line.period(),
        }
    }

    fn u_len(&self) -> f64 {
        match self {
            Curve3::Arc(arc) => arc.u_len(),
            Curve3::Helix(helix) => helix.u_len(),
            Curve3::Line(line) => line.u_len(),
        }
    }

    fn is_periodic(&self) -> bool {
        match self {
            Curve3::Arc(arc) => arc.is_periodic(),
            Curve3::Helix(helix) => helix.is_periodic(),
            Curve3::Line(line) => line.is_periodic(),
        }
    }

    fn tangent(&self, u: f64) -> Vec3 {
        match self {
            Curve3::Arc(arc) => arc.tangent(u),
            Curve3::Helix(helix) => helix.tangent(u),
            Curve3::Line(line) => line.tangent(u),
        }
    }

    fn local_coords(&self, u: f64) -> Mat33 {
        match self {
            Curve3::Arc(arc) => arc.local_coords(u),
            Curve3::Helix(helix) => helix.local_coords(u),
            Curve3::Line(line) => line.local_coords(u),
        }
    }

    fn curvature(&self, u: f64) -> f64 {
        match self {
            Curve3::Arc(arc) => arc.curvature(u),
            Curve3::Helix(helix) => helix.curvature(u),
            Curve3::Line(line) => line.curvature(u),
        }
    }

    fn eval_sections(&self, chords: u32) -> Vec<Point3> {
        match self {
            Curve3::Arc(arc) => arc.eval_sections(chords),
            Curve3::Helix(helix) => helix.eval_sections(chords),
            Curve3::Line(line) => line.eval_sections(chords),
        }
    }

    fn frenet(&self, u: f64) -> Mat33 {
        match self {
            Curve3::Arc(arc) => arc.frenet(u),
            Curve3::Helix(helix) => helix.frenet(u),
            Curve3::Line(line) => line.frenet(u),
        }
    }
}

pub trait Curve3Impl {
    fn u_min(&self) -> f64;
    fn u_max(&self) -> f64;

    fn u_len(&self) -> f64 {
        self.u_max() - self.u_min()
    }

    fn period(&self) -> Option<f64>;

    fn is_periodic(&self) -> bool {
        self.period().is_some()
    }

    fn eval(&self, u: f64) -> Point3;
    fn der1(&self, u: f64) -> Vec3;
    fn der2(&self, u: f64) -> Vec3;
    fn der3(&self, u: f64) -> Vec3;

    fn tangent(&self, u: f64) -> Vec3 {
        self.der1(u).normalize()
    }

    fn local_coords(&self, u: f64) -> Mat33 {
        // X axis is tangent to the curve
        let der1 = self.der1(u);
        let x = der1.normalize();

        // To find Y, first find any vector D that is
        // not parallel to the X-axis.
        let d = x.non_parallel();

        // Find the component of D that is perpendicular
        // to the X-axis. Normalize it and use it as the
        // Y-axis
        let d2 = d - (x.dot(d)) * x;
        let y = d2.normalize();

        // Z-axis is perpendicular to X and Y axes
        let z = x.cross(y);

        Mat33::from_axes(x, y, z)
    }

    fn curvature(&self, u: f64) -> f64 {
        let der1 = self.der1(u);
        let der2 = self.der1(u);

        let num = der1.cross(der2).magnitude();
        let den = der1.magnitude().powi(3);

        num / den
    }

    fn eval_sections(&self, chords: u32) -> Vec<Point3> {
        let u_min = self.u_min();
        let u_max = self.u_max();
        let param_interval = self.u_len() / chords as f64;

        let mut points = Vec::with_capacity(chords as usize + 1);
        for i in 0..=chords {
            let u = match i {
                0 => u_min,
                i if i == chords => u_max,
                i => u_min + param_interval * i as f64,
            };

            points.push(self.eval(u));
        }

        points
    }

    fn frenet(&self, u: f64) -> Mat33 {
        let d1 = self.der1(u);
        let d2 = self.der2(u);

        /*
        let b = d1.cross(d2).normalize();

        let x = d1.normalize();
        let z = b;
        let y = z.cross(x);
         */

        let x = d1;
        let y = d2.normalize();
        let z = x.cross(y);

        Mat33::from_axes(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use space::lerp;

    use super::*;

    /// Validates the derivative of a function with a 1-dimensional input space
    pub(crate) fn validate_der_1d<F: Fn(f64) -> Vec3, D: Fn(f64) -> Vec3>(
        function: F,
        derivative: D,
        u_min: f64,
        u_max: f64,
        samples: usize,
        tolerance: f64,
        name: &str,
    ) {
        let u_len = u_max - u_min;

        for i in 0..samples {
            // Parameter deviation that we start checking each sample with
            let mut deviation = u_len / 10.0;

            // Define u for the current sample
            let u = lerp(u_min, u_max, i as f64 / (samples - 1) as f64);

            // Calculate the exact first derivative at u
            let computed_derivative = derivative(u);

            // Flag for whether the approximated first derivative below gets close enough to actual_der1
            let mut converged = false;

            // Last approximate derivative that does not contain NaNs
            let mut last_notnan_approx = Vec3::ZERO;

            // Iteratively approximate the derivative by getting the vector between two points on the curve
            // centered around u, decreasing their distance from u each time.
            for _ in 0..64 {
                // Get parameters above and below u, clamped between 0 and 1
                let u_lo = (u - deviation).clamp(u_min, u_max);
                let u_hi = (u + deviation).clamp(u_min, u_max);

                // Evaluate the curve at those parameters
                let lo_pos = function(u_lo);
                let hi_pos = function(u_hi);

                // Approximate the derivative by getting a vector between those two points
                // and scaling it by the parameter distance between them
                let estimated_derivative = (hi_pos - lo_pos) / (u_hi - u_lo);

                if !estimated_derivative.has_nan() {
                    last_notnan_approx = estimated_derivative;
                }

                // Get the difference between the exact derivative vector and the approximated one
                let error = (computed_derivative - estimated_derivative).magnitude();

                // If the distance is within tolerance, we consider the exact derivative
                // calculation to be valid and stop iteration for this sample.
                if error < tolerance {
                    converged = true;
                    break;
                }

                // If we haven't converged yet, reduce the deviation from u.
                deviation /= 2.0;
            }

            // Panic if we never got close enough to the exact derivative calculation.
            if !converged {
                panic!(
                    "{} @ u = {} is {}, only converged to {}",
                    name, u, computed_derivative, last_notnan_approx,
                );
            }
        }
    }

    pub fn validate_ders_1d<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
        validate_der1_1d(curve, samples, tolerance);
        validate_der2_1d(curve, samples, tolerance);
        validate_der3_1d(curve, samples, tolerance);
    }

    pub fn validate_der1_1d<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
        validate_der_1d(
            |u| curve.eval(u).into_vec(),
            |u| curve.der1(u),
            curve.u_min(),
            curve.u_max(),
            samples,
            tolerance,
            "First derivative",
        );
    }

    pub fn validate_der2_1d<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
        validate_der_1d(
            |u| curve.der1(u),
            |u| curve.der2(u),
            curve.u_min(),
            curve.u_max(),
            samples,
            tolerance,
            "Second derivative",
        );
    }

    pub fn validate_der3_1d<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
        validate_der_1d(
            |u| curve.der2(u),
            |u| curve.der3(u),
            curve.u_min(),
            curve.u_max(),
            samples,
            tolerance,
            "Third derivative",
        );
    }
}
