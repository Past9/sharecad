use space::{Mat33, Vec2};

use crate::{Curve2, Curve2Impl};

pub fn offset(base: Curve2, offset: Vec2) -> Offset {
    Offset::new(base, offset)
}

pub struct Offset {
    pub offset: Vec2,
    pub base: Box<Curve2>,
}
impl Offset {
    pub fn new(base: Curve2, offset: Vec2) -> Self {
        // We don't want to create and offset curve on another offset curve.
        // Instead we'll use the original base curve with the sum of the previous
        // and new offsets.
        let (base, offset) = match base {
            Curve2::Offset(Self {
                offset: previous_offset,
                base,
            }) => (*base, previous_offset + offset),
            other => (other, offset),
        };

        Self {
            offset,
            base: Box::new(base),
        }
    }
}
impl Curve2Impl for Offset {
    fn u_min(&self) -> f64 {
        self.base.u_min()
    }

    fn u_max(&self) -> f64 {
        self.base.u_max()
    }

    fn eval(&self, u: f64) -> space::Point2 {
        let (local_x, local_y) = self.base.local_axes(u);
        //println!("local {}, {}", local_x, local_y);
        let rotation = Mat33::rotation_from_axes(local_x, local_y);
        //println!("rotation {:?}", rotation);
        self.base.eval(u) + (self.offset.to_point().transform(rotation)).to_vec()
    }

    fn der1(&self, u: f64) -> Vec2 {
        todo!()
    }

    fn der2(&self, u: f64) -> Vec2 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use space::{deg, point2, vec2};

    use crate::segment;

    use super::*;

    #[test]
    fn offset_test() {
        println!(
            "{:?}",
            Mat33::rotation_from_axes(vec2(1.0, 0.0), vec2(0.0, -1.0))
        );

        let segment = segment(point2(2.0, 2.0), point2(7.0, 4.0));
        let offset = offset(Curve2::Segment(segment.clone()), vec2(3.0, -1.0));

        let samples = 10;
        for i in 0..=samples {
            let u = i as f64 / samples as f64;

            offset.eval(u);

            //println!("normal = {}", segment.tangent(u).orthogonal());

            println!(
                "{}, {}",
                segment.eval_normalized(u),
                offset.eval_normalized(u)
            );
        }
    }
}
