use std::marker::PhantomData;

#[derive(Debug)]
pub struct IdSeries<T: From<u32> + Into<u32>> {
    last_id: u32,
    _t: PhantomData<T>,
}
impl<T: From<u32> + Into<u32>> IdSeries<T> {
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

    pub fn advance(&mut self, last_id: T) {
        self.last_id = self.last_id.max(last_id.into());
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SurfaceId(pub u32);
impl From<u32> for SurfaceId {
    fn from(id: u32) -> Self {
        SurfaceId(id)
    }
}
impl From<SurfaceId> for u32 {
    fn from(id: SurfaceId) -> Self {
        id.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CurveId(pub u32);
impl From<u32> for CurveId {
    fn from(id: u32) -> Self {
        CurveId(id)
    }
}
impl From<CurveId> for u32 {
    fn from(id: CurveId) -> Self {
        id.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointId(pub u32);
impl From<u32> for PointId {
    fn from(id: u32) -> Self {
        PointId(id)
    }
}
impl From<PointId> for u32 {
    fn from(id: PointId) -> Self {
        id.0
    }
}
