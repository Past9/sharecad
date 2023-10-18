use std::marker::PhantomData;

mod camera;
mod instance;
mod light;
mod model;
mod pipeline;
mod render;
mod resources;
mod scene;
mod state;
mod texture;
mod vertex;
mod window;

#[derive(Debug)]
pub(crate) struct IdSeries<T: From<u32>> {
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

fn main() {
    pollster::block_on(window::run());
}
