use space::{deg, vec2, Point3, Quat};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
};

use crate::render::PositionRenderer;

use super::Camera;

const ZOOM_SENSITIVITY: f64 = 0.2;
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

pub struct EventResult {
    pub processed: bool,
    pub requests: Vec<CameraControllerRequest>,
}
impl EventResult {
    pub fn processed<const N: usize>(requests: [CameraControllerRequest; N]) -> Self {
        Self {
            processed: true,
            requests: requests.to_vec(),
        }
    }

    pub fn unprocessed<const N: usize>(requests: [CameraControllerRequest; N]) -> Self {
        Self {
            processed: false,
            requests: requests.to_vec(),
        }
    }
}

#[derive(Clone)]
pub enum CameraControllerRequest {
    RequestOrbitPoint,
}

pub struct CameraController {
    orbit_point: Point3,
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
    pub fn new() -> Self {
        Self {
            orbit_point: Point3::ZERO,
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

    pub fn set_orbit_point(&mut self, orbit_point: Point3) {
        println!("new orbit: {:?}", self.orbit_point);
        self.orbit_point = orbit_point;
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> EventResult {
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
                        EventResult::processed([])
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        EventResult::processed([])
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        EventResult::processed([])
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        EventResult::processed([])
                    }
                    _ => EventResult::unprocessed([]),
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

                EventResult::processed([])
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *button == MouseButton::Right {
                    if *state == ElementState::Pressed {
                        self.rmb_drag_state = DragState::Dragging {
                            params: OrbitParams {
                                orbit_to_target_dist: None,
                            },
                            last_pos: self.mouse_pos,
                            current_pos: self.mouse_pos,
                        };
                        EventResult::processed([CameraControllerRequest::RequestOrbitPoint])
                    } else {
                        self.rmb_drag_state = DragState::None;
                        EventResult::processed([])
                    }
                } else if *button == MouseButton::Middle {
                    self.mmb_drag_state = match state {
                        ElementState::Pressed => DragState::Dragging {
                            params: (),
                            last_pos: self.mouse_pos,
                            current_pos: self.mouse_pos,
                        },
                        ElementState::Released => DragState::None,
                    };
                    EventResult::processed([])
                } else {
                    EventResult::processed([])
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y as f64,
                    winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition { x: _, y }) => {
                        *y as f64
                    }
                };

                EventResult::processed([])
            }
            _ => EventResult::unprocessed([]),
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dimensions: (u32, u32)) {
        self.update_mouse_zoom(camera, dimensions);

        self.update_mouse_orbit(camera);
        self.update_mouse_pan(camera, dimensions);
    }

    fn update_mouse_zoom(&mut self, camera: &mut Camera, dimensions: (u32, u32)) {
        if self.scroll_delta == 0.0 {
            return;
        }

        let radius = camera.target_radius();

        // Calculate a zoom factor. The target radius will be multiplied by this.
        let zoom = if self.scroll_delta > 0.0 {
            (1.0 - ZOOM_SENSITIVITY).powf(self.scroll_delta.abs())
        } else if self.scroll_delta < 0.0 {
            (1.0 + ZOOM_SENSITIVITY).powf(self.scroll_delta.abs())
        } else {
            1.0
        };

        // Adjust the target position so that the object under the mouse pointer
        // at the target distance stays in the same spot on the screen after zooming
        {
            let (w, h) = (dimensions.0 as f64, dimensions.1 as f64);

            // The radius of `ptr` in pixels.
            let ptr_pixels = match w < h {
                true => w,
                false => h,
            };

            // X and Y mouse position with respect to an origin (0, 0) at
            // the center of the screen, positive X to the right and positive Y up.
            // Distances are expressed as fractions of the camera's planar_target_radius;
            let mouse = vec2(
                2.0 * (self.mouse_pos.x - w / 2.0) / ptr_pixels,
                -2.0 * (self.mouse_pos.y - h / 2.0) / ptr_pixels,
            );

            // The distance in world space to move the target so the point under the mouse pointer
            // at the target distance remains in the same spot on the screen.
            let target_move_dist = mouse.magnitude() * camera.planar_target_radius() * (1.0 - zoom);

            // Generate a vector by which to move the target point by going that distance
            // along the direction from the center of the screen to the mouse pointer
            let target_move = target_move_dist * mouse.normalize();

            // Translation vectors to move relative to the camera's local coordinate system
            let move_x = target_move.x * camera.local_x();
            let move_y = target_move.y * camera.local_y();

            // Move the camera
            camera.set_target(camera.target() + move_x + move_y);
        }

        // Now zoom the camera
        camera.set_target_radius(radius * zoom);

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

            // Diameter of the target area in world coordinates. This is the radius
            // (in world coordinates) of a circle at the target distance that is
            // circumscribed by the display area.
            let ptd = camera.planar_target_radius() * 2.0;

            // The radius of `ptr` in pixels.
            let ptd_pixels = match w < h {
                true => w,
                false => h,
            };

            // What fraction of the planar target radius the mouse has moved in X and Y
            // directions (in camera local coordinates)
            let ptd_frac_x = x / ptd_pixels;
            let ptd_frac_y = y / ptd_pixels;

            // Vectors to move the camera so that objexts at the target distance stay with
            // the mouse pointer
            let move_x = -ptd_frac_x * ptd * camera.local_x();
            let move_y = ptd_frac_y * ptd * camera.local_y();

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
                .get_or_insert_with(|| (camera.target() - self.orbit_point).magnitude());

            // Move the camera around the orbit point according to the mouse movement
            {
                let (x, y) = (current_pos.x - last_pos.x, current_pos.y - last_pos.y);

                let rotation = Quat::from_axis_angle(camera.local_y(), deg(x * ORBIT_SENSITIVITY))
                    + Quat::from_axis_angle(camera.local_x(), deg(y * ORBIT_SENSITIVITY));

                camera.rotate_around(self.orbit_point, rotation);
            }

            // Due to floating point imprecision, the camera's target point may "drift away" from the
            // orbit point during an orbit drag. To correct this, we move the target point after
            // rotating the camera (above) so that it's always `original_orbit_to_target_dist` away.
            {
                // The vector from orbit to target point after rotating
                let orbit_to_target = camera.target() - self.orbit_point;

                // The length of the vector. This is what we're correcting.
                let dist = orbit_to_target.magnitude();

                let corrected_target = if dist > 0.0 {
                    // Move the target slightly along `orbit_to_target` (in whichever direction is needed)
                    // so that `original_orbit_to_target_dist` is maintained.
                    self.orbit_point + orbit_to_target * (*original_orbit_to_target_dist / dist)
                } else {
                    // If the camera is orbiting its exact target point, just move the target
                    // to the orbit.
                    self.orbit_point
                };

                // Move the camera to the corrected point
                camera.set_target(corrected_target);
            }

            // Update the DragState so we don't use this mouse movement more than once
            *last_pos = current_pos;
        }
    }
}
