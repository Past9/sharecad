use super::{SAdd, Scalar};

#[derive(Copy, Clone)]
pub struct Vec2<S: Scalar> {
    pub x: S,
    pub y: S,
}

impl<S: Scalar> SAdd<S> for Vec2<S> {
    type Output = Vec2<S>;

    fn add(self, rhs: S) -> Self::Output {
        Vec2 {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}
impl<S: Scalar> std::fmt::Display for Vec2<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}, {}]", self.x, self.y))
    }
}

impl<S: Scalar> SAdd<Vec2<S>> for S {
    type Output = Vec2<S>;

    fn add(self, rhs: Vec2<S>) -> Self::Output {
        Vec2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}
