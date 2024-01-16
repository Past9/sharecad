use super::{ArcPointSolver, HelixPointSolver, LinePointSolver, SSCurvePoint};
use crate::math::{Scalar, Vec3};

pub trait ICurvePoint<S: Scalar> {
    fn u(&self) -> S;
    fn pos(&self) -> &Vec3<S>;
    fn der1(&self) -> &Vec3<S>;
    fn der2(&self) -> &Vec3<S>;
    fn der3(&self) -> &Vec3<S>;

    fn curvature(&self) -> S {
        (self.der1().magnitude().powi(3) / (self.der1().cross(*self.der2())).magnitude()).recip()
    }
}

pub enum CurvePoint<S: Scalar> {
    Line(LinePointSolver<S>),
    Helix(HelixPointSolver<S>),
    Arc(ArcPointSolver<S>),
    SSCurve(SSCurvePoint<S>),
}
impl<S: Scalar> CurvePoint<S> {
    pub fn u(&self) -> S {
        match self {
            CurvePoint::Line(line) => line.u(),
            CurvePoint::Helix(helix) => helix.u(),
            CurvePoint::Arc(arc) => arc.u(),
            CurvePoint::SSCurve(ss_curve) => ss_curve.u(),
        }
    }

    pub fn pos(&self) -> &Vec3<S> {
        match self {
            CurvePoint::Line(line) => line.pos(),
            CurvePoint::Helix(helix) => helix.pos(),
            CurvePoint::Arc(arc) => arc.pos(),
            CurvePoint::SSCurve(ss_curve) => ss_curve.pos(),
        }
    }

    pub fn der1(&self) -> &Vec3<S> {
        match self {
            CurvePoint::Line(line) => line.der1(),
            CurvePoint::Helix(helix) => helix.der1(),
            CurvePoint::Arc(arc) => arc.der1(),
            CurvePoint::SSCurve(ss_curve) => ss_curve.der1(),
        }
    }

    pub fn der2(&self) -> &Vec3<S> {
        match self {
            CurvePoint::Line(line) => line.der2(),
            CurvePoint::Helix(helix) => helix.der2(),
            CurvePoint::Arc(arc) => arc.der2(),
            CurvePoint::SSCurve(ss_curve) => ss_curve.der2(),
        }
    }

    pub fn der3(&self) -> &Vec3<S> {
        match self {
            CurvePoint::Line(line) => line.der3(),
            CurvePoint::Helix(helix) => helix.der3(),
            CurvePoint::Arc(arc) => arc.der3(),
            CurvePoint::SSCurve(ss_curve) => ss_curve.der3(),
        }
    }

    pub fn curvature(&self) -> S {
        match self {
            CurvePoint::Line(line) => line.curvature(),
            CurvePoint::Helix(helix) => helix.curvature(),
            CurvePoint::Arc(arc) => arc.curvature(),
            CurvePoint::SSCurve(ss_curve) => ss_curve.curvature(),
        }
    }
}
impl<S: Scalar> From<LinePointSolver<S>> for CurvePoint<S> {
    fn from(point: LinePointSolver<S>) -> Self {
        Self::Line(point)
    }
}
impl<S: Scalar> From<HelixPointSolver<S>> for CurvePoint<S> {
    fn from(point: HelixPointSolver<S>) -> Self {
        Self::Helix(point)
    }
}
impl<S: Scalar> From<ArcPointSolver<S>> for CurvePoint<S> {
    fn from(point: ArcPointSolver<S>) -> Self {
        Self::Arc(point)
    }
}
impl<S: Scalar> From<SSCurvePoint<S>> for CurvePoint<S> {
    fn from(point: SSCurvePoint<S>) -> Self {
        Self::SSCurve(point)
    }
}

pub(crate) fn axes<S: Scalar>(
    point: &CurvePoint<S>,
    never_tangent: &Vec3<S>,
) -> (Vec3<S>, Vec3<S>, Vec3<S>) {
    let i1 = point.der1().normalize();
    let d = *never_tangent;

    let d2 = d - i1 * (i1.dot(d));

    let i2 = d2.normalize();
    let i3 = i1.cross(i2);

    (i1, i2, i3)
}

pub(crate) fn axes_der1<S: Scalar>(
    point: &CurvePoint<S>,
    never_tangent: &Vec3<S>,
    axes: &(Vec3<S>, Vec3<S>, Vec3<S>),
) -> (Vec3<S>, Vec3<S>, Vec3<S>) {
    let (i1, i2, _) = *axes;

    let d = *never_tangent;
    let d2 = d - i1 * (i1.dot(d));

    let der1 = point.der1();
    let der2 = *point.der2();
    let i1_der1 = der1.norm_der1(der2);

    //let d2_der1 = -i1 * (i1_der1.dot(d));
    let d2_der1 = (i1 * -i1_der1.dot(d)) - (i1_der1 * i1.dot(d));
    let i2_der1 = d2.norm_der1(d2_der1);

    let i3_der1 = i1.cross(i2_der1) + i1_der1.cross(i2);

    (i1_der1, i2_der1, i3_der1)
}

pub(crate) fn axes_der2<S: Scalar>(
    point: &CurvePoint<S>,
    never_tangent: &Vec3<S>,
    axes: &(Vec3<S>, Vec3<S>, Vec3<S>),
    axes_der1: &(Vec3<S>, Vec3<S>, Vec3<S>),
) -> (Vec3<S>, Vec3<S>, Vec3<S>) {
    let (i1, i2, _) = *axes;
    let (i1_der1, i2_der1, _) = *axes_der1;

    let d = *never_tangent;
    let d2 = d - i1 * i1.dot(d);
    let d2_der1 = (i1 * -i1_der1.dot(d)) - (i1_der1 * i1.dot(d));

    let der1 = *point.der1();
    let der2 = *point.der2();
    let der3 = *point.der3();

    let i1_der2 = der1.norm_der2(der2, der3);

    //let d2_der2 = -i1 * (i1_der2.dot(d));
    let d2_der2 =
        (i1 * -i1_der2.dot(d)) - (i1_der1 * i1_der1.dot(d)) * S::TWO - (i1_der2 * i1.dot(d));
    let i2_der2 = d2.norm_der2(d2_der1, d2_der2);

    let i3_der2 = i1.cross(i2_der2) + i1_der1.cross(i2_der1) * S::TWO + i1_der2.cross(i2);

    (i1_der2, i2_der2, i3_der2)
}
