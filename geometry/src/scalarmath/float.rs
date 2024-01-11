use super::{SAdd, Scalar};

#[derive(Copy, Clone)]
pub struct Float(pub f64);
impl Scalar for Float {
    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }
}
impl SAdd<Self> for Float {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
