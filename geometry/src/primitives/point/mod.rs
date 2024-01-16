mod cc_point;

pub use cc_point::*;

use crate::math::{Scalar, Vec3};

#[derive(Clone, Debug)]
pub enum Point<S: Scalar> {
    Point(Vec3<S>),
    CCPoint(CCPoint<S>),
}
impl<S: Scalar> Point<S> {
    pub fn pos(&self) -> Vec3<S> {
        match self {
            Point::Point(point3) => *point3,
            Point::CCPoint(cc_point) => cc_point.pos,
        }
    }
}
impl<S: Scalar> From<Vec3<S>> for Point<S> {
    fn from(point: Vec3<S>) -> Self {
        Self::Point(point)
    }
}
impl<S: Scalar> From<CCPoint<S>> for Point<S> {
    fn from(cc_point: CCPoint<S>) -> Self {
        Self::CCPoint(cc_point)
    }
}
