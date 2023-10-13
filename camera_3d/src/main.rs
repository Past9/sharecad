mod camera;
mod instance;
mod texture;
mod window;

fn main() {
    pollster::block_on(window::run());
}
