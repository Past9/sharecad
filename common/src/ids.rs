use std::marker::PhantomData;

#[derive(Debug)]
pub struct IdSeries<T: From<u32>> {
    last_id: u32,
    _t: PhantomData<T>,
}
impl<T: From<u32>> IdSeries<T> {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            _t: PhantomData,
        }
    }

    pub fn next(&mut self) -> T {
        self.last_id += 1;
        self.last_id.into()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u32);
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {
        SurfaceId(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CurveId(pub u32);
impl From<u32> for CurveId {
    fn from(id: u32) -> Self {
        CurveId(id)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointId(pub u32);
impl From<u32> for PointId {
    fn from(id: u32) -> Self {
        PointId(id)
    }
}
