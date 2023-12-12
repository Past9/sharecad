mod helix;
mod line;

pub use helix::*;
pub use line::*;
use space::{vec2, vec3, Coincidence, Mat22, Point3, Quat, Vec2, Vec3, COINCIDENT_TOL, NEWTON_TOL};

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

    pub fn min_distance_to(&self, other: &Self) -> Option<Curve3Distance> {
        let extrema = Self::distance_extrema(self, other);
        Curve3Distance::shortest(&extrema)
    }

    pub fn line_deviation(&self, u_start: f64, u_end: f64) -> Option<Curve3Distance> {
        let line = Self::line(self.eval(u_start), self.eval(u_end));

        let extrema = Self::find_distance_extrema(
            self,
            &line,
            vec2(u_start, line.u_min()),
            vec2(u_end, line.u_max()),
        );

        Curve3Distance::longest(&extrema)
    }

    pub fn distance_extrema(cu: &Self, cv: &Self) -> Vec<Curve3Distance> {
        let uv_min = vec2(cu.u_min(), cv.u_min());
        let uv_max = vec2(cu.u_max(), cv.u_max());

        let cu_start = cu.eval(cu.u_min());
        let cu_end = cu.eval(cu.u_max());
        let cv_start = cv.eval(cv.u_min());
        let cv_end = cv.eval(cv.u_max());

        let mut extrema = vec![
            Curve3Distance {
                uv: vec2(cu.u_min(), cv.u_min()),
                distance: (cu_start - cv_start).magnitude(),
                cu_pos: cu_start,
                cv_pos: cv_start,
            },
            Curve3Distance {
                uv: vec2(cu.u_min(), cv.u_max()),
                distance: (cu_start - cv_end).magnitude(),
                cu_pos: cu_start,
                cv_pos: cv_end,
            },
            Curve3Distance {
                uv: vec2(cu.u_max(), cv.u_min()),
                distance: (cu_end - cv_start).magnitude(),
                cu_pos: cu_end,
                cv_pos: cv_start,
            },
            Curve3Distance {
                uv: vec2(cu.u_max(), cv.u_max()),
                distance: (cu_end - cv_end).magnitude(),
                cu_pos: cu_end,
                cv_pos: cv_end,
            },
        ];

        extrema.extend(Self::find_distance_extrema(cu, cv, uv_min, uv_max));

        extrema
    }

    fn find_distance_extrema(
        cu: &Self,
        cv: &Self,
        uv_min: Vec2,
        uv_max: Vec2,
    ) -> Vec<Curve3Distance> {
        //let uv_min = vec2(cu.u_min(), cv.u_min());
        //let uv_max = vec2(cu.u_max(), cv.u_max());

        let mut results = vec![];

        let mut u_params = cu
            .distance_extrema_params()
            .into_iter()
            .filter(|u| *u > uv_min.x && *u < uv_max.x)
            .collect::<Vec<f64>>();

        let mut v_params = cv
            .distance_extrema_params()
            .into_iter()
            .filter(|v| *v > uv_min.y && *v < uv_max.y)
            .collect::<Vec<f64>>();

        println!(
            "initial u_params = {:?}, v_params = {:?}",
            u_params, v_params
        );

        if u_params.len() == 0 {
            u_params.push((uv_min.x + uv_max.x) / 2.0);
        }

        if v_params.len() == 0 {
            v_params.push((uv_min.y + uv_max.y) / 2.0);
        }

        println!("fixed u_params = {:?}, v_params = {:?}", u_params, v_params);

        for u in u_params.iter() {
            for v in v_params.iter() {
                if let Some(result) =
                    Self::local_distance_extrema(cu, cv, vec2(*u, *v), uv_min, uv_max)
                {
                    results.push(result);
                }
            }
        }

        if results.len() == 0 {
            panic!("NO RESULTS");
        }

        results
    }

    /// Returns initial parameters to use when trying to find extrema of
    /// the curve's distance from another entity.
    fn distance_extrema_params(&self) -> Vec<f64> {
        let segments = match self {
            Curve3::Helix(helix) => 3 * helix.n().ceil() as u32,
            Curve3::Line(_) => 2,
        };

        let segments = self.param_segments(segments, true);

        //println!("param_segments {:?}", segments);

        segments
    }

    fn local_distance_extrema(
        cu: &Self,
        cv: &Self,
        uv: Vec2,
        uv_min: Vec2,
        uv_max: Vec2,
    ) -> Option<Curve3Distance> {
        let mut converged = false;
        let mut last = Self::distance_iter(cu, cv, uv, uv_min, uv_max);

        for _ in 0..MAX_NEWTON_ITER {
            let iter = Self::distance_iter(cu, cv, last.uv_next, uv_min, uv_max);

            if (iter.uv - last.uv).magnitude().cc_newton(NEWTON_TOL) {
                converged = true;
                break;
            }

            last = iter;
        }

        if converged {
            Some(Curve3Distance {
                uv: last.uv,
                distance: last.distance,
                cu_pos: last.cu_pos,
                cv_pos: last.cv_pos,
            })
        } else {
            None
        }
    }

    fn distance_iter(
        cu: &Self,
        cv: &Self,
        uv: Vec2,
        uv_min: Vec2,
        uv_max: Vec2,
    ) -> Curve3DistanceIter {
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

        // Newton's method
        let uv_next = (uv - hessian_inv * gradient).clamp(uv_min, uv_max);

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

    fn period(&self) -> Option<f64> {
        match self {
            Curve3::Helix(helix) => helix.period(),
            Curve3::Line(line) => line.period(),
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
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use space::point3;

    use super::*;

    /*
    #[test]
    fn line_line_dist() {
        let l0 = Curve3::line(point3(0.0, -3.0, 0.0), point3(0.0, 1.0, 0.0));
        let l1 = Curve3::line(point3(1.0, 0.0, -1.0), point3(1.0, 0.0, 3.0));

        let res = l0.min_dist(&l1);

        assert_eq!(
            Some(Curve3Distance {
                uv: vec2(0.75, 0.25),
                distance: 1.0,
                cu_pos: point3(0.0, 0.0, 0.0),
                cv_pos: point3(1.0, 0.0, 0.0)
            }),
            res
        );
    }

    #[test]
    fn dist() {
        let c0 = Curve3::helix(1.0, 0.2, 5.0, Quat::ZERO, Vec3::ZERO);

        println!("helix {:#?}", c0);

        let c1 = Curve3::line(point3(2.0, -5.0, 2.0), point3(2.0, 5.0, 2.0));

        let num_iter = 1;
        let start = Instant::now();
        for _ in 0..num_iter {
            let res = c0.min_dist(&c1);
            println!("res = {:#?}", res);
        }
        let end = Instant::now();
        let dur = end - start;
        println!(
            "{} iter in {}us, {}us per iter",
            num_iter,
            dur.as_micros(),
            dur.as_micros() / num_iter
        );
    }
    */

    #[test]
    fn dist2() {
        let c0 = Curve3::helix(1.0, 0.2, 5.0, Quat::ZERO, Vec3::ZERO);
        let c1 = Curve3::line(point3(2.0, -5.0, 2.0), point3(2.0, 5.0, 2.0));

        let results = Curve3::distance_extrema(&c0, &c1);

        println!("results = {:#?}", results);
        println!("results.len() = {}", results.len());
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
