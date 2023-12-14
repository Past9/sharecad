mod arc;
mod offset;
mod segment;

use space::{Point2, Vec2};

pub use arc::*;
pub use offset::*;
pub use segment::*;

pub enum Curve2 {
    Arc(Arc),
    Segment(Segment),
    Offset(Offset),
}
impl Curve2 {
    pub fn is_offset(&self) -> bool {
        match self {
            Curve2::Offset(_) => true,
            _ => false,
        }
    }
}
impl Curve2Impl for Curve2 {
    fn u_min(&self) -> f64 {
        match self {
            Curve2::Arc(arc) => arc.u_min(),
            Curve2::Segment(segment) => segment.u_min(),
            Curve2::Offset(offset) => offset.u_min(),
        }
    }

    fn u_max(&self) -> f64 {
        match self {
            Curve2::Arc(arc) => arc.u_max(),
            Curve2::Segment(segment) => segment.u_max(),
            Curve2::Offset(offset) => offset.u_max(),
        }
    }

    fn eval(&self, u: f64) -> Point2 {
        match self {
            Curve2::Arc(arc) => arc.eval(u),
            Curve2::Segment(segment) => segment.eval(u),
            Curve2::Offset(offset) => offset.eval(u),
        }
    }

    fn der1(&self, u: f64) -> Vec2 {
        match self {
            Curve2::Arc(arc) => arc.der1(u),
            Curve2::Segment(segment) => segment.der1(u),
            Curve2::Offset(offset) => offset.der1(u),
        }
    }

    fn der2(&self, u: f64) -> Vec2 {
        match self {
            Curve2::Arc(arc) => arc.der2(u),
            Curve2::Segment(segment) => segment.der2(u),
            Curve2::Offset(offset) => offset.der2(u),
        }
    }
}

pub trait Curve2Impl {
    fn u_min(&self) -> f64;
    fn u_max(&self) -> f64;

    fn u_len(&self) -> f64 {
        self.u_max() - self.u_min()
    }

    fn eval(&self, u: f64) -> Point2;
    fn der1(&self, u: f64) -> Vec2;
    fn der2(&self, u: f64) -> Vec2;

    fn tangent(&self, u: f64) -> Vec2 {
        self.der1(u).normalize()
    }

    fn normal(&self, u: f64) -> Vec2 {
        self.tangent(u).orthogonal()
    }

    fn local_axes(&self, u: f64) -> (Vec2, Vec2) {
        todo!("Make this consistent with Curve3Impl::frenet(...)");
        let tangent = self.tangent(u);
        (-tangent.orthogonal(), tangent)
    }

    fn curvature(&self, u: f64) -> f64 {
        let der1 = self.der1(u);
        let der2 = self.der2(u);

        let num = (der1.x * der2.y) - (der1.y * der2.x);
        let den = der1.dot(der1).powf(1.5);

        num / den
    }
}

#[cfg(test)]
mod tests {
    use space::{deg, vec2, Mat33};

    use super::*;

    pub fn validate_der1<C: Curve2Impl>(curve: &C, samples: usize, tolerance: f64) {
        for i in 0..samples {
            // Parameter deviation that we start checking each sample with
            let mut deviation = curve.u_len() / 10.0;

            // Define u for the current sample
            let u = i as f64 / (samples - 1) as f64;

            // Calculate the exact first derivative at u
            let actual_der1 = curve.der1(u);

            // Flag for whether the approximated first derivative below gets close enough to actual_der1
            let mut converged = false;

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
                panic!("First derivative failed to converge at u = {}, possible incorrect der1(...) function", u);
            }
        }
    }

    pub fn validate_der2<C: Curve2Impl>(curve: &C, samples: usize, tolerance: f64) {
        for i in 0..samples {
            // Parameter deviation that we start checking each sample with
            let mut deviation = curve.u_len() / 10.0;

            // Define u for the current sample
            let u = i as f64 / (samples - 1) as f64;

            // Calculate the exact second derivative at u
            let actual_der2 = curve.der2(u);

            // Flag for whether the approximated second derivative below gets close enough to actual_der2
            let mut converged = false;

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
                let approx_der1 = (hi_der1 - lo_der1) / (u_hi - u_lo);

                // Get the difference between the exact derivative vector and the approximated one
                let dist = (actual_der2 - approx_der1).magnitude();

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
                panic!("Second derivative failed to converge at u = {}, possible incorrect der2(...) function", u);
            }
        }
    }
}
