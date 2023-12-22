use space::{Point3, Vec3};

use crate::{ICurve, ICurvePoint};

pub struct ArcCurve {}
impl<'a> ICurve<'a> for ArcCurve {
    type Point = ArcPoint<'a>;

    fn domain(&self) -> (f64, f64) {
        todo!()
    }

    fn point(&'a self, u: f64) -> Self::Point {
        todo!()
    }
}

pub struct ArcPoint<'a> {
    arc: &'a ArcCurve,
}
impl<'a> ArcPoint<'a> {}
impl<'a> ICurvePoint for ArcPoint<'a> {
    fn u(&self) -> f64 {
        todo!()
    }

    fn eval(&self) -> &Point3 {
        todo!()
    }

    fn der1(&self) -> &Vec3 {
        todo!()
    }

    fn der2(&self) -> &Vec3 {
        todo!()
    }

    fn der3(&self) -> &Vec3 {
        todo!()
    }

    fn never_tangent(&self) -> &Vec3 {
        todo!()
    }
}
