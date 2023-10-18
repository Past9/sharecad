mod camera;
mod light;
mod material;
mod model;
mod render;
mod scene;
mod state;
mod texture;
mod window;

fn main() {
    pollster::block_on(window::run());
}
