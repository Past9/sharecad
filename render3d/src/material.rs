use crate::texture::TextureId;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);
impl From<u32> for MaterialId {
    fn from(id: u32) -> Self {
        MaterialId(id)
    }
}

#[derive(Debug)]
pub struct Material {
    pub id: MaterialId,
    pub name: String,
    pub diffuse: TextureId,
    pub normal: TextureId,
}
impl Material {
    pub fn new(id: MaterialId, name: &str, diffuse: TextureId, normal: TextureId) -> Self {
        Self {
            id,
            name: name.into(),
            diffuse,
            normal,
        }
    }
}
