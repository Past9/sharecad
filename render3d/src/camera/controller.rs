use space::{deg, Point3, Quat};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
};

use super::Camera;

const ZOOM_SENSITIVITY: f64 = 0.1;
const ORBIT_SENSITIVITY: f64 = 0.1;

#[derive(Debug, Clone)]
struct OrbitParams {
    /// The distance between the orbit point and the camera's target or "look at"
    /// point at the start of an orbit drag.
    orbit_to_target_dist: Option<f64>,
}

#[derive(Debug)]
enum DragState<T> {
    None,
    Dragging {
        params: T,
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
    rmb_drag_state: DragState<OrbitParams>,
    mmb_drag_state: DragState<()>,
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
            mmb_drag_state: DragState::None,
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
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = position.cast();

                // Orbit dragging with RMB
                if let DragState::Dragging {
                    params, last_pos, ..
                } = &self.rmb_drag_state
                {
                    self.rmb_drag_state = DragState::Dragging {
                        params: params.clone(),
                        last_pos: *last_pos,
                        current_pos: self.mouse_pos,
                    };
                }

                // Pan dragging with MMB
                if let DragState::Dragging {
                    params, last_pos, ..
                } = &self.mmb_drag_state
                {
                    self.mmb_drag_state = DragState::Dragging {
                        params: params.clone(),
                        last_pos: *last_pos,
                        current_pos: self.mouse_pos,
                    };
                }

                true
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *button == MouseButton::Right {
                    self.rmb_drag_state = match state {
                        ElementState::Pressed => DragState::Dragging {
                            params: OrbitParams {
                                orbit_to_target_dist: None,
                            },
                            last_pos: self.mouse_pos,
                            current_pos: self.mouse_pos,
                        },
                        ElementState::Released => DragState::None,
                    };
                } else if *button == MouseButton::Middle {
                    self.mmb_drag_state = match state {
                        ElementState::Pressed => DragState::Dragging {
                            params: (),
                            last_pos: self.mouse_pos,
                            current_pos: self.mouse_pos,
                        },
                        ElementState::Released => DragState::None,
                    };
                }

                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
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

    pub fn update_camera(&mut self, camera: &mut Camera, dimensions: (u32, u32)) {
        self.update_mouse_zoom(camera);

        self.update_mouse_orbit(camera);
        self.update_mouse_pan(camera, dimensions);
    }

    fn update_mouse_zoom(&mut self, camera: &mut Camera) {
        let radius = camera.target_radius();

        let zoom = if self.scroll_delta > 0.0 {
            (1.0 - ZOOM_SENSITIVITY).powf(self.scroll_delta.abs())
        } else if self.scroll_delta < 0.0 {
            (1.0 + ZOOM_SENSITIVITY).powf(self.scroll_delta.abs())
        } else {
            1.0
        };

        let new_target_radius = radius * zoom;

        camera.set_target_radius(new_target_radius);

        self.scroll_delta = 0.0;
    }

    fn update_mouse_pan(&mut self, camera: &mut Camera, dimensions: (u32, u32)) {
        if let DragState::Dragging {
            ref mut last_pos,
            current_pos,
            ..
        } = self.mmb_drag_state
        {
            // Size in pixels of the displayed scene
            let (w, h) = (dimensions.0 as f64, dimensions.1 as f64);

            // X and Y mouse movement in pixels
            let (x, y) = (current_pos.x - last_pos.x, current_pos.y - last_pos.y);

            // Radius of the target area in world coordinates. This is the radius
            // (in world coordinates) of a circle at the target distance that is
            // circumscribed by the display area.
            let ptr = camera.planar_target_radius() * 2.0;

            // The radius of `ptr` in pixels.
            let ptr_pixels = match w < h {
                true => w,
                false => h,
            };

            // What fraction of the planar target radius the mouse have moved in X and Y
            // directions (in camera local coordinates)
            let ptr_frac_x = x / ptr_pixels;
            let ptr_frac_y = y / ptr_pixels;

            // Vectors to move the camera so that objexts at the target distance stay with
            // the mouse pointer
            let move_x = -ptr_frac_x * ptr * camera.local_x();
            let move_y = ptr_frac_y * ptr * camera.local_y();

            // Move the camera
            camera.set_target(camera.target() + move_x + move_y);

            // Update the DragState so we don't use this mouse movement more than once
            *last_pos = current_pos;
        }
    }

    fn update_mouse_orbit(&mut self, camera: &mut Camera) {
        if let DragState::Dragging {
            ref mut params,
            ref mut last_pos,
            current_pos,
        } = self.rmb_drag_state
        {
            // Set the orbit-to-target distance if this is the first movement
            // of the orbit.
            let original_orbit_to_target_dist = params
                .orbit_to_target_dist
                .get_or_insert_with(|| (camera.target() - self.orbit).magnitude());

            // Move the camera around the orbit point according to the mouse movement
            {
                let (x, y) = (current_pos.x - last_pos.x, current_pos.y - last_pos.y);

                let rotation = Quat::from_axis_angle(camera.local_y(), deg(x * ORBIT_SENSITIVITY))
                    + Quat::from_axis_angle(camera.local_x(), deg(y * ORBIT_SENSITIVITY));

                camera.rotate_around(self.orbit, rotation);
            }

            // Due to floating point imprecision, the camera's target point may "drift away" from the
            // orbit point during an orbit drag. To correct this, we move the target point after
            // rotating the camera (above) so that it's always `original_orbit_to_target_dist` away.
            {
                // The vector from orbit to target point after rotating
                let orbit_to_target = camera.target() - self.orbit;

                // The length of the vector. This is what we're correcting.
                let dist = orbit_to_target.magnitude();

                let corrected_target = if dist > 0.0 {
                    // Move the target slightly along `orbit_to_target` (in whichever direction is needed)
                    // so that `original_orbit_to_target_dist` is maintained.
                    self.orbit + orbit_to_target * (*original_orbit_to_target_dist / dist)
                } else {
                    // If the camera is orbiting its exact target point, just move the target
                    // to the orbit.
                    self.orbit
                };

                // Move the camera to the corrected point
                camera.set_target(corrected_target);
            }

            // Update the DragState so we don't use this mouse movement more than once
            *last_pos = current_pos;
        }
    }
}
