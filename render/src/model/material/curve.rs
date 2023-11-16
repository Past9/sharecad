#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CurveMaterialId(pub u32);
impl From<u32> for CurveMaterialId {
    fn from(id: u32) -> Self {
        CurveMaterialId(id)
    }
}
