mod cc_point;

pub use cc_point::*;

use crate::math::Point3;

pub enum Point {
    Point(Point3),
    CCPoint(CCPoint),
}
impl From<Point3> for Point {
    fn from(point: Point3) -> Self {
        Self::Point(point)
    }
}
impl From<CCPoint> for Point {
    fn from(cc_point: CCPoint) -> Self {
        Self::CCPoint(cc_point)
    }
}
