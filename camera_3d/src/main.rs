mod camera;
mod instance;
mod model;
mod resources;
mod state;
mod texture;
mod vertex;
mod window;

fn main() {
    pollster::block_on(window::run());
}
