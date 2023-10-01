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

    /// Map the "normalized" u (0.0 to 1.0) to the actual parameter domain
    /// of the curve with linear interpolation.
    fn denormalize_u(&self, u: f64) -> f64 {
        (1.0 - u) * self.u_min() + u * self.u_max()
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
        let tangent = self.tangent(u);
        (tangent, tangent.orthogonal())
    }

    fn eval_normalized(&self, u: f64) -> Point2 {
        self.eval(self.denormalize_u(u))
    }

    fn der1_normalized(&self, u: f64) -> Vec2 {
        self.der1(self.denormalize_u(u))
    }

    fn der2_normalized(&self, u: f64) -> Vec2 {
        self.der1(self.denormalize_u(u))
    }

    fn tangent_normalized(&self, u: f64) -> Vec2 {
        self.der1_normalized(u).normalize()
    }
}
