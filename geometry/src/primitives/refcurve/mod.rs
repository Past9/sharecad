mod line;

pub use line::*;

pub enum RefCurve<'a> {
    Line(RefLine<'a>),
}
