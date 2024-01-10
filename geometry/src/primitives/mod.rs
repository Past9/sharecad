mod curve;
mod point;
mod surface;

pub use curve::*;
pub use point::*;
pub use surface::*;

#[derive(Clone, Debug)]
pub enum Point {
    CCPoint(CCPoint),
}
impl Point {
    //
}
impl From<CCPoint> for Point {
    fn from(cc_point: CCPoint) -> Self {
        Self::CCPoint(cc_point)
    }
}
