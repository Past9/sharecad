mod cc_point;

pub use cc_point::*;

use crate::math::Point3;

#[derive(Clone, Debug)]
pub enum Point {
    Point(Point3),
    CCPoint(CCPoint),
}
impl Point {
    pub fn pos(&self) -> Point3 {
        match self {
            Point::Point(point3) => *point3,
            Point::CCPoint(cc_point) => cc_point.pos,
        }
    }
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
