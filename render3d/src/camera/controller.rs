use space::{deg, Point3, Quat, Vec3};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
};

use super::Camera;

const ZOOM_SENSITIVITY: f64 = 0.1;
const ORBIT_SENSITIVITY: f64 = 0.1;

#[derive(Debug)]
enum DragState {
    None,
    Dragging {
        last_pos: PhysicalPosition<f64>,
        current_pos: PhysicalPosition<f64>,
    },
}

pub struct CameraController {
    orbit: Point3,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    scroll_delta: f64,
    mouse_pos: PhysicalPosition<f64>,
    rmb_drag_state: DragState,
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
            mouse_pos: PhysicalPosition::new(0.0, 0.0),
            rmb_drag_state: DragState::None,
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
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                self.mouse_pos = position.cast();

                // Orbit dragging with RMB
                if let DragState::Dragging {
                    last_pos,
                    current_pos,
                } = self.rmb_drag_state
                {
                    self.rmb_drag_state = DragState::Dragging {
                        last_pos: last_pos,
                        current_pos: self.mouse_pos,
                    };
                }

                true
            }
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                if *button == MouseButton::Right {
                    self.rmb_drag_state = match state {
                        ElementState::Pressed => DragState::Dragging {
                            last_pos: self.mouse_pos,
                            current_pos: self.mouse_pos,
                        },
                        ElementState::Released => DragState::None,
                    };
                }
                true
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => {
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
        self.update_mouse_orbit(camera);
        self.update_zoom(camera)
    }

    fn update_zoom(&mut self, camera: &mut Camera) {
        let radius = camera.target_radius();

        let zoom = if self.scroll_delta > 0.0 {
            (1.0 - ZOOM_SENSITIVITY).powf(self.scroll_delta.abs())
        } else if self.scroll_delta < 0.0 {
            (1.0 + ZOOM_SENSITIVITY).powf(self.scroll_delta.abs())
        } else {
            1.0
        };

        camera.set_target_radius(radius * zoom);

        self.scroll_delta = 0.0;
    }

    fn update_mouse_orbit(&mut self, camera: &mut Camera) {
        if let DragState::Dragging {
            ref mut last_pos,
            current_pos,
        } = self.rmb_drag_state
        {
            let mut rotation = Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0));
            let (x, y) = (current_pos.x - last_pos.x, current_pos.y - last_pos.y);

            rotation += Quat::from_axis_angle(camera.local_y(), deg(x * ORBIT_SENSITIVITY));
            rotation += Quat::from_axis_angle(camera.local_x(), deg(y * ORBIT_SENSITIVITY));

            let xy = camera.local_x().dot(camera.local_y());
            let yz = camera.local_y().dot(camera.local_z());
            let xz = camera.local_x().dot(camera.local_z());

            println!("\nX {:?}", camera.local_x());
            println!("Y {:?}", camera.local_y());
            println!("Z {:?}", camera.local_z());
            println!("XY {}", xy);
            println!("YZ {}", yz);
            println!("XZ {}", xz);

            if xy.is_nan() || yz.is_nan() || xz.is_nan() {
                panic!("NaN");
            }

            camera.rotate_around(self.orbit, rotation);

            *last_pos = current_pos;
        }
    }

    /*
    fn update_key_orbit(&self, camera: &mut Camera) {
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
     */
}
