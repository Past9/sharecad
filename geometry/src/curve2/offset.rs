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
        let rotation = Mat33::rotation_from_axes(local_x, local_y);
        self.base.eval(u) + (self.offset.to_point().transform(rotation)).to_vec()
    }

    fn der1(&self, _u: f64) -> Vec2 {
        todo!()
    }

    fn der2(&self, _u: f64) -> Vec2 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use space::{assert_cc, point2, vec2};

    use crate::{arc, segment};

    use super::*;

    #[test]
    fn offsets_segment() {
        let base = segment(point2(2.0, 1.0), point2(5.0, 4.0));

        // Offset the base curve by [0.0, 1.0] in the base's local coordinate system
        let offset_curve = offset(Curve2::Segment(base.clone()), vec2(0.0, 1.0));

        // Should result in the same segment, but translated by [-sqrt(2) / 2, sqrt(2) / 2]
        let translation = Mat33::translation(vec2(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0));
        let expected = segment(
            point2(2.0, 1.0).transform(&translation),
            point2(5.0, 4.0).transform(translation),
        );

        assert_cc!(expected.eval(0.0), offset_curve.eval(0.0));
        assert_cc!(expected.eval(0.5), offset_curve.eval(0.5));
        assert_cc!(expected.eval(1.0), offset_curve.eval(1.0));

        // Offset the base curve by [1.0, 1.0] in the base's local coordinate system
        let offset_curve = offset(Curve2::Segment(base.clone()), vec2(1.0, 1.0));

        // Should result in the same segment, but translated by [0, sqrt(2)]
        let translation = Mat33::translation(vec2(0.0, 2f64.sqrt()));
        let expected = segment(
            point2(2.0, 1.0).transform(&translation),
            point2(5.0, 4.0).transform(translation),
        );

        assert_cc!(expected.eval(0.0), offset_curve.eval(0.0));
        assert_cc!(expected.eval(0.5), offset_curve.eval(0.5));
        assert_cc!(expected.eval(1.0), offset_curve.eval(1.0));

        // Offset the base curve by [1.0, 0.0] in the base's local coordinate system
        let offset_curve = offset(Curve2::Segment(base.clone()), vec2(1.0, 0.0));

        // Should result in the same segment, but translated by [sqrt(2) / 2, sqrt(2) / 2]
        let translation = Mat33::translation(vec2(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0));
        let expected = segment(
            point2(2.0, 1.0).transform(&translation),
            point2(5.0, 4.0).transform(translation),
        );

        assert_cc!(expected.eval(0.0), offset_curve.eval(0.0));
        assert_cc!(expected.eval(0.5), offset_curve.eval(0.5));
        assert_cc!(expected.eval(1.0), offset_curve.eval(1.0));
    }

    #[test]
    fn offsets_arc() {
        let base = arc(Mat33::IDENTITY, 1.0, 1.0);

        // Offset the base curve by [0.0, 1.0] in the base's local coordinate system
        let offset_curve = offset(Curve2::Arc(base.clone()), vec2(0.0, -1.0));

        // Should result in the same segment, but translated by [-sqrt(2) / 2, sqrt(2) / 2]
        //let translation = Mat33::translation(vec2(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0));

        let samples = 8;
        for i in 0..=samples {
            let u = i as f64 / samples as f64;

            This is wrong

            println!(
                "{}, {}",
                base.eval_normalized(u),
                offset_curve.eval_normalized(u)
            );
        }
    }
}
