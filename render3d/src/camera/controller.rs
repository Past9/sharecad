use space::{deg, Point3, Quat, Vec3};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};

use super::Camera;

pub struct CameraController {
    orbit: Point3,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    scroll_delta: f64,
}
impl CameraController {
    pub fn new(orbit: Point3) -> Self {
        Self {
            orbit,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            scroll_delta: 0.0,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                modifiers,
            } => {
                //
                true
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => {
                println!("delta = {:?}", delta);
                self.scroll_delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y as f64,
                    winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition { x: _, y }) => {
                        *y as f64
                    }
                };
                true
            }
            _ => false,
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        self.update_orbit(camera);
        self.update_zoom(camera)
    }

    fn update_zoom(&mut self, camera: &mut Camera) {
        let sensitivity: f64 = 0.1;
        let radius = camera.target_radius();

        let zoom = if self.scroll_delta > 0.0 {
            (1.0 - sensitivity).powf(self.scroll_delta.abs())
        } else if self.scroll_delta < 0.0 {
            (1.0 + sensitivity).powf(self.scroll_delta.abs())
        } else {
            1.0
        };

        camera.set_target_radius(radius * zoom);

        self.scroll_delta = 0.0;
    }

    fn update_orbit(&self, camera: &mut Camera) {
        let mut rotation = Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0));

        if self.is_left_pressed {
            rotation += Quat::from_axis_angle(camera.local_y(), deg(0.1));
        }

        if self.is_right_pressed {
            rotation += Quat::from_axis_angle(camera.local_y(), -deg(0.1));
        }

        if self.is_forward_pressed {
            rotation += Quat::from_axis_angle(camera.local_x(), deg(0.1));
        }

        if self.is_backward_pressed {
            rotation += Quat::from_axis_angle(camera.local_x(), -deg(0.1));
        }

        camera.rotate_around(self.orbit, rotation);
    }
}
