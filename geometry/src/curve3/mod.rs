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

    fn never_tangent(&self) -> Vec3 {
        match self {
            Curve3::Arc(arc) => arc.never_tangent(),
            Curve3::Helix(helix) => helix.never_tangent(),
            Curve3::Line(line) => line.never_tangent(),
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

    fn never_tangent(&self) -> Vec3;

    fn eval(&self, u: f64) -> Point3;
    fn der1(&self, u: f64) -> Vec3;
    fn der2(&self, u: f64) -> Vec3;
    fn der3(&self, u: f64) -> Vec3;

    fn tangent(&self, u: f64) -> Vec3 {
        self.der1(u).normalize()
    }

    fn curvature(&self, u: f64) -> f64 {
        let der1 = self.der1(u);
        let der2 = self.der2(u);

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
