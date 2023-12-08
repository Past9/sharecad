mod helix;
mod line;

pub use helix::*;
pub use line::*;
use space::{vec2, vec3, Mat22, Point3, Quat, Vec2, Vec3};

#[derive(Debug)]
pub struct Curve3Distance {
    uv: Vec2,
    distance: f64,
    cu_pos: Point3,
    cv_pos: Point3,
}

struct Curve3DistanceIter {
    uv: Vec2,
    distance: f64,
    cu_pos: Point3,
    cv_pos: Point3,
    uv_next: Vec2,
}

#[derive(Debug)]
pub enum Curve3 {
    Helix(Helix),
    Line(Line),
}
impl Curve3 {
    pub fn helix(r: f64, h: f64, n: f64, orientation: Quat, translation: Vec3) -> Self {
        Self::Helix(Helix::new(r, h, n, orientation, translation))
    }

    pub fn line(start: Point3, end: Point3) -> Self {
        Self::Line(Line::new(start, end))
    }

    pub fn min_dist(&self, other: &Self) -> Option<Curve3Distance> {
        let initial_uv = vec2(
            (self.u_min() + self.u_max()) / 2.0,
            (other.u_min() + other.u_max()) / 2.0,
        );
        Self::min_dist_from_initial_guess(self, other, initial_uv)
    }

    fn min_dist_from_initial_guess(cu: &Self, cv: &Self, mut uv: Vec2) -> Option<Curve3Distance> {
        println!("start uv = {}", uv);

        let mut best = None;
        let mut uv_next = uv;
        for _ in 0..2 {
            let iter = Self::dist2(cu, cv, uv_next);
            uv_next = iter.uv_next;

            best = Some(Curve3Distance {
                uv: iter.uv,
                cu_pos: iter.cu_pos,
                cv_pos: iter.cv_pos,
                distance: iter.distance,
            })
        }

        best
    }

    fn dist2(cu: &Self, cv: &Self, uv: Vec2) -> Curve3DistanceIter {
        let cu_pos = cu.eval(uv.x);
        let cv_pos = cv.eval(uv.y);
        let cu_der1 = cu.der1(uv.x);
        let cv_der1 = cv.der1(uv.y);
        let cu_der2 = cu.der2(uv.x);
        let cv_der2 = cv.der2(uv.y);

        // Vector from cv(v) to cu(u).
        let cv_cu = cu_pos - cv_pos;

        // Squared distance from cv(v) to cu(u).
        // This is the function we're minimizing.
        let dist2 = cv_cu.magnitude2();

        let gradient = vec2(
            (2.0 * cv_cu * cu_der1).sum(),
            (-2.0 * cv_cu * cv_der1).sum(),
        );

        // Hessian
        let duu = 2.0 * (cu_der1.powi(2) + cv_cu * cu_der2).sum();
        let dvv = 2.0 * (cv_der1.powi(2) - cv_cu * cv_der2).sum();
        let duv_vu = -2.0 * (cu_der1 * cv_der1).sum();
        let hessian = Mat22::new(duu, duv_vu, duv_vu, dvv);
        let hessian_inv = hessian.inverse().unwrap();

        let uv_next = uv - hessian_inv * gradient;

        Curve3DistanceIter {
            uv,
            distance: dist2.sqrt(),
            cu_pos,
            cv_pos,
            uv_next,
        }
    }
}
impl Curve3Impl for Curve3 {
    fn u_min(&self) -> f64 {
        match self {
            Curve3::Helix(helix) => helix.u_min(),
            Curve3::Line(line) => line.u_min(),
        }
    }

    fn u_max(&self) -> f64 {
        match self {
            Curve3::Helix(helix) => helix.u_max(),
            Curve3::Line(line) => line.u_max(),
        }
    }

    fn eval(&self, u: f64) -> Point3 {
        match self {
            Curve3::Helix(helix) => helix.eval(u),
            Curve3::Line(line) => line.eval(u),
        }
    }

    fn der1(&self, u: f64) -> Vec3 {
        match self {
            Curve3::Helix(helix) => helix.der1(u),
            Curve3::Line(line) => line.der1(u),
        }
    }

    fn der2(&self, u: f64) -> Vec3 {
        match self {
            Curve3::Helix(helix) => helix.der2(u),
            Curve3::Line(line) => line.der2(u),
        }
    }
}

pub trait Curve3Impl {
    fn u_min(&self) -> f64;
    fn u_max(&self) -> f64;

    fn u_len(&self) -> f64 {
        self.u_max() - self.u_min()
    }

    fn eval(&self, u: f64) -> Point3;
    fn der1(&self, u: f64) -> Vec3;
    fn der2(&self, u: f64) -> Vec3;

    fn tangent(&self, u: f64) -> Vec3 {
        self.der1(u).normalize()
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
}

#[cfg(test)]
mod tests {
    use space::point3;

    use super::*;

    #[test]
    fn line_line_dist() {
        let l0 = Curve3::line(point3(0.0, -3.0, 0.0), point3(0.0, 1.0, 0.0)); // 0.75
        let l1 = Curve3::line(point3(1.0, 0.0, -1.0), point3(1.0, 0.0, 3.0)); // 0.25

        let dist = l0.min_dist(&l1);

        println!("end = {:#?}", dist);
    }

    pub fn validate_der1<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
        for i in 0..samples {
            // Parameter deviation that we start checking each sample with
            let mut deviation = curve.u_len() / 10.0;

            // Define u for the current sample
            let u = i as f64 / (samples - 1) as f64;

            // Calculate the exact first derivative at u
            let actual_der1 = curve.der1(u);

            // Flag for whether the approximated first derivative below gets close enough to actual_der1
            let mut converged = false;

            // Last approximate derivative that does not contain NaNs
            let mut last_notnan_approx = Vec3::ZERO;

            // Iteratively approximate the derivative by getting the vector between two points on the curve
            // centered around u, decreasing their distance from u each time.
            for _ in 0..64 {
                // Get parameters above and below u, clamped between 0 and 1
                let u_lo = (u - deviation).clamp(0.0, 1.0);
                let u_hi = (u + deviation).clamp(0.0, 1.0);

                // Evaluate the curve at those parameters
                let lo_pos = curve.eval(u_lo);
                let hi_pos = curve.eval(u_hi);

                // Approximate the derivative by getting a vector between those two points
                // and scaling it by the parameter distance between them
                let approx_der1 = (hi_pos - lo_pos) / (u_hi - u_lo);

                if !approx_der1.has_nan() {
                    last_notnan_approx = approx_der1;
                }

                // Get the difference between the exact derivative vector and the approximated one
                let dist = (actual_der1 - approx_der1).magnitude();

                // If the distance is within tolerance, we consider the exact derivative
                // calculation to be valid and stop iteration for this sample.
                if dist < tolerance {
                    converged = true;
                    break;
                }

                // If we haven't converged yet, reduce the deviation from u.
                deviation /= 2.0;
            }

            // Panic if we never got close enough to the exact derivative calculation.
            if !converged {
                panic!(
                    "Derivative 1 @ u = {} is {}, only converged to {}, outside tolerance {}",
                    u, actual_der1, last_notnan_approx, tolerance
                );
            }
        }
    }

    pub fn validate_der2<C: Curve3Impl>(curve: &C, samples: usize, tolerance: f64) {
        for i in 0..samples {
            // Parameter deviation that we start checking each sample with
            let mut deviation = curve.u_len() / 10.0;

            // Define u for the current sample
            let u = i as f64 / (samples - 1) as f64;

            // Calculate the exact second derivative at u
            let actual_der2 = curve.der2(u);

            // Flag for whether the approximated second derivative below gets close enough to actual_der2
            let mut converged = false;

            // Last approximate derivative that does not contain NaNs
            let mut last_notnan_approx = Vec3::ZERO;

            // Iteratively approximate the second derivative by getting the vector between two first derivative
            // vectors on the curve centered around u, decreasing their distance from u each time.
            for _ in 0..64 {
                // Get parameters above and below u, clamped between 0 and 1
                let u_lo = (u - deviation).clamp(0.0, 1.0);
                let u_hi = (u + deviation).clamp(0.0, 1.0);

                // Evaluate the first derivative of the curve at those parameters
                let lo_der1 = curve.der1(u_lo);
                let hi_der1 = curve.der1(u_hi);

                // Approximate the second derivative by getting a vector between those two vectors
                // and scaling it by the parameter distance between them
                let approx_der2 = (hi_der1 - lo_der1) / (u_hi - u_lo);

                if !approx_der2.has_nan() {
                    last_notnan_approx = approx_der2;
                }

                // Get the difference between the exact derivative vector and the approximated one
                let dist = (actual_der2 - approx_der2).magnitude();

                // If the distance is within tolerance, we consider the exact derivative
                // calculation to be valid and stop iteration for this sample.
                if dist < tolerance {
                    converged = true;
                    break;
                }

                // If we haven't converged yet, reduce the deviation from u.
                deviation /= 2.0;
            }

            // Panic if we never got close enough to the exact derivative calculation.
            if !converged {
                panic!(
                    "Derivative 2 @ u = {} is {}, only converged to {}, outside tolerance {}",
                    u, actual_der2, last_notnan_approx, tolerance
                );
            }
        }
    }
}
