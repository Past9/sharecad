mod arc;
mod helix;
mod line;

pub use arc::*;
pub use helix::*;
pub use line::*;
use space::{
    vec2, vec3, Angle, Coincidence, Mat22, Mat33, Point3, Quat, Vec2, Vec3, COINCIDENT_TOL,
    NEWTON_TOL,
};

const MAX_NEWTON_ITER: u32 = 64;

#[derive(Debug, PartialEq, Clone)]
pub struct Curve3Distance {
    pub uv: Vec2,
    pub distance: f64,
    pub cu_pos: Point3,
    pub cv_pos: Point3,
}
impl Curve3Distance {
    pub fn shortest(distances: &[Self]) -> Option<Self> {
        if distances.len() == 0 {
            return None;
        }

        let mut shortest = &distances[0];
        for dist in distances.iter() {
            if dist.distance < shortest.distance {
                shortest = dist;
            }
        }

        Some(shortest.clone())
    }

    pub fn longest(distances: &[Self]) -> Option<Self> {
        if distances.len() == 0 {
            return None;
        }

        let mut longest = &distances[0];
        for dist in distances.iter() {
            if dist.distance > longest.distance {
                longest = dist;
            }
        }

        Some(longest.clone())
    }

    pub fn dedup(mut distances: Vec<Self>) -> Vec<Self> {
        if distances.len() == 0 {
            return distances;
        }

        distances.sort_by(|a, b| a.uv.x.total_cmp(&b.uv.x).then(a.uv.y.total_cmp(&b.uv.y)));

        //println!("\n\n\n\ndistances = {:#?}\n\n\n\n", distances);

        let avg = |dists: &[&Self]| {
            //println!("dists len = {}", dists.len());
            let total = dists.iter().fold(
                Self {
                    uv: Vec2::ZERO,
                    distance: 0.0,
                    cu_pos: Point3::ZERO,
                    cv_pos: Point3::ZERO,
                },
                |a, b| Self {
                    uv: a.uv + b.uv,
                    distance: a.distance + b.distance,
                    cu_pos: a.cu_pos + b.cu_pos,
                    cv_pos: a.cv_pos + b.cv_pos,
                },
            );

            let len = dists.len() as f64;
            Self {
                uv: total.uv / len,
                distance: total.distance / len,
                cu_pos: total.cu_pos / len,
                cv_pos: total.cv_pos / len,
            }
        };

        let mut deduped = vec![];
        let mut compare = &distances[0];
        let mut dupes = vec![&distances[0]];
        for i in 1..distances.len() {
            if Self::are_dupes(compare, &distances[i]) {
                dupes.push(&distances[i])
            } else {
                deduped.push(avg(&dupes));
                dupes = vec![&distances[i]];
                compare = &distances[i];
            }
        }

        if dupes.len() > 0 {
            deduped.push(avg(&dupes));
        }

        deduped

        //distances
    }

    fn are_dupes(a: &Self, b: &Self) -> bool {
        a.cu_pos.cc(b.cu_pos) && a.cv_pos.cc(b.cv_pos)
    }

    fn dedup_2(a: &Self, b: &Self) -> (Self, Option<Self>) {
        if Self::are_dupes(a, b) {
            (
                Self {
                    uv: (a.uv + b.uv) / 2.0,
                    distance: (a.distance + b.distance) / 2.0,
                    cu_pos: (a.cu_pos + b.cu_pos) / 2.0,
                    cv_pos: (a.cv_pos + b.cv_pos) / 2.0,
                },
                None,
            )
        } else {
            (a.clone(), Some(b.clone()))
        }
    }
}

#[derive(Debug)]
struct Curve3DistanceIter {
    uv: Vec2,
    distance: f64,
    cu_pos: Point3,
    cv_pos: Point3,
    uv_next: Vec2,
}

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

    fn frenet(&self, u: f64) -> Mat33 {
        let d1 = self.der1(u);
        let d2 = self.der2(u);

        let b = d1.cross(d2).normalize();

        let x = d1.normalize();
        let z = b;
        let y = z.cross(x);

        Mat33::from_axes(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
