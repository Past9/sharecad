use std::cell::OnceCell;

use common::PointId;
use space::{Point3, Vec3};

use crate::{primitives::CurvePointAxes, RefGeometry};

pub struct RefLine<'a> {
    geometry: &'a RefGeometry<'a>,
    start: PointId,
    end: PointId,
}
impl<'a> RefLine<'a> {
    pub fn new(geometry: &'a RefGeometry, start: PointId, end: PointId) -> Self {
        Self {
            geometry,
            start,
            end,
        }
    }

    pub fn point(&'a self, u: f64) -> RefLinePoint<'a> {
        RefLinePoint {
            u,
            start: self.geometry.get_point(self.start).unwrap(),
            end: self.geometry.get_point(self.end).unwrap(),

            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            der3: OnceCell::new(),
            axes: OnceCell::new(),
        }
    }
}

pub struct RefLinePoint<'a> {
    u: f64,
    start: &'a Point3,
    end: &'a Point3,

    eval: OnceCell<Point3>,
    der1: OnceCell<Vec3>,
    der2: OnceCell<Vec3>,
    der3: OnceCell<Vec3>,
    axes: OnceCell<CurvePointAxes<'a>>,
}
impl<'a> RefLinePoint<'a> {}
