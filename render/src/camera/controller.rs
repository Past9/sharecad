use geometry::math::{deg, vec2, Angle, Quat, Vec2, Vec3};

use super::Camera;
use crate::input::{InputEvent, MouseButton};

const ZOOM_SENSITIVITY: f64 = 0.2;
const ORBIT_SENSITIVITY: f64 = 0.4;

#[derive(Debug, Clone)]
struct OrbitParams {
    starting_camera: Camera,
    rotation: Quat<f64>,
}

#[derive(Debug)]
enum DragState<T> {
    None,
    Dragging {
        params: T,
        start_pos: Vec2<f64>,
        last_pos: Vec2<f64>,
        current_pos: Vec2<f64>,
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

#[derive(Clone, Copy)]
pub enum OrbitPointMode {
    Locked,
    Adaptive,
}

pub struct CameraController {
    size: (u32, u32),
    camera: Camera,
    orbit_point: Vec3<f64>,
    orbit_point_mode: OrbitPointMode,
    scroll_delta: f64,
    is_ctrl_pressed: bool,
    mouse_pos: Vec2<f64>,
    orbit_drag_state: DragState<OrbitParams>,
    pan_drag_state: DragState<()>,
}
impl CameraController {
    pub fn new(camera: Camera) -> Self {
        Self {
            size: (0, 0),
            camera,
            orbit_point: Vec3::ZERO,
            orbit_point_mode: OrbitPointMode::Adaptive,
            scroll_delta: 0.0,
            is_ctrl_pressed: false,
            mouse_pos: Vec2::ZERO,
            orbit_drag_state: DragState::None,
            pan_drag_state: DragState::None,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;
    }

    pub fn set_orbit_point(&mut self, orbit_point: Vec3<f64>) {
        self.orbit_point = orbit_point;
    }

    pub fn orbit_point(&self) -> Vec3<f64> {
        self.orbit_point
    }

    pub fn set_orbit_point_mode(&mut self, mode: OrbitPointMode) {
        self.orbit_point_mode = mode;
    }

    pub fn orbit_point_mode(&self) -> OrbitPointMode {
        self.orbit_point_mode
    }

    pub fn process_events(&mut self, event: &InputEvent) -> EventResult {
        let result = match event {
            InputEvent::KeyDown(key) => {
                if key.is_control() {
                    self.is_ctrl_pressed = true;
                }
                EventResult::processed([])
            }
            InputEvent::KeyUp(key) => {
                if key.is_control() {
                    self.is_ctrl_pressed = false;
                    self.stop_orbit();
                    self.stop_pan();
                }
                EventResult::processed([])
            }
            InputEvent::CursorMoved(position) => {
                self.mouse_pos = *position;

                self.update_orbit();
                self.update_pan();

                EventResult::processed([])
            }
            InputEvent::MouseDown(button) => {
                if *button == MouseButton::Secondary {
                    if self.is_ctrl_pressed {
                        self.stop_orbit();
                        self.start_pan()
                    } else {
                        self.stop_pan();
                        self.start_orbit()
                    }
                } else if *button == MouseButton::Aux {
                    self.stop_orbit();
                    self.start_pan()
                } else {
                    self.stop_pan()
                }
            }
            InputEvent::MouseUp(button) => {
                if *button == MouseButton::Secondary {
                    if self.is_ctrl_pressed {
                        self.stop_pan()
                    } else {
                        self.stop_orbit()
                    }
                } else if *button == MouseButton::Aux {
                    self.stop_pan()
                } else {
                    self.stop_pan()
                }
            }
            InputEvent::MouseWheel(delta) => {
                self.scroll_delta = delta.y;
                EventResult::processed([])
            }
            _ => EventResult::unprocessed([]),
        };

        self.apply_zoom();
        self.apply_orbit();
        self.apply_pan();

        if let DragState::Dragging {
            ref mut last_pos,
            current_pos,
            ..
        } = &mut self.orbit_drag_state
        {
            *last_pos = *current_pos;
        }

        if let DragState::Dragging {
            ref mut last_pos,
            current_pos,
            ..
        } = &mut self.pan_drag_state
        {
            *last_pos = *current_pos;
        }

        result
    }

    fn start_orbit(&mut self) -> EventResult {
        self.orbit_drag_state = DragState::Dragging {
            params: OrbitParams {
                starting_camera: self.camera.clone(),
                rotation: Quat::from_axis_angle(Vec3::UNIT_Y, Angle::ZERO),
            },
            start_pos: self.mouse_pos,
            last_pos: self.mouse_pos,
            current_pos: self.mouse_pos,
        };
        match self.orbit_point_mode {
            OrbitPointMode::Locked => EventResult::processed([]),
            OrbitPointMode::Adaptive => {
                EventResult::processed([CameraControllerRequest::RequestOrbitPoint])
            }
        }
    }

    fn update_orbit(&mut self) {
        if let DragState::Dragging {
            params,
            start_pos,
            current_pos,
            ..
        } = &self.orbit_drag_state
        {
            self.orbit_drag_state = DragState::Dragging {
                params: params.clone(),
                start_pos: *start_pos,
                last_pos: *current_pos,
                current_pos: self.mouse_pos,
            };
        }
    }

    fn stop_orbit(&mut self) -> EventResult {
        self.orbit_drag_state = DragState::None;
        EventResult::processed([])
    }

    fn start_pan(&mut self) -> EventResult {
        self.pan_drag_state = DragState::Dragging {
            params: (),
            start_pos: self.mouse_pos,
            last_pos: self.mouse_pos,
            current_pos: self.mouse_pos,
        };
        EventResult::processed([])
    }

    fn update_pan(&mut self) {
        if let DragState::Dragging {
            params,
            start_pos,
            current_pos,
            ..
        } = &self.pan_drag_state
        {
            self.pan_drag_state = DragState::Dragging {
                params: params.clone(),
                start_pos: *start_pos,
                last_pos: *current_pos,
                current_pos: self.mouse_pos,
            };
        }
    }

    fn stop_pan(&mut self) -> EventResult {
        self.pan_drag_state = DragState::None;
        EventResult::processed([])
    }

    fn apply_zoom(&mut self) {
        if self.scroll_delta == 0.0 {
            return;
        }

        let radius = self.camera.target_radius();

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
            let (w, h) = (self.size.0 as f64, self.size.1 as f64);

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
            let target_move_dist =
                mouse.magnitude() * self.camera.planar_target_radius() * (1.0 - zoom);

            // Generate a vector by which to move the target point by going that distance
            // along the direction from the center of the screen to the mouse pointer
            let target_move = target_move_dist * mouse.normalize();

            // Translation vectors to move relative to the camera's local coordinate system
            let move_x = target_move.x * self.camera.local_x();
            let move_y = target_move.y * self.camera.local_y();

            // Move the camera
            self.camera
                .set_target(self.camera.target() + move_x + move_y);
        }

        // Now zoom the camera
        self.camera.set_target_radius(radius * zoom);

        self.scroll_delta = 0.0;
    }

    fn apply_pan(&mut self) {
        if let DragState::Dragging {
            last_pos,
            current_pos,
            ..
        } = self.pan_drag_state
        {
            // Size in pixels of the displayed scene
            let (w, h) = (self.size.0 as f64, self.size.1 as f64);

            // X and Y mouse movement in pixels
            let (x, y) = (current_pos.x - last_pos.x, current_pos.y - last_pos.y);

            // Diameter of the target area in world coordinates. This is the radius
            // (in world coordinates) of a circle at the target distance that is
            // circumscribed by the display area.
            let ptd = self.camera.planar_target_radius() * 2.0;

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
            let move_x = -ptd_frac_x * ptd * self.camera.local_x();
            let move_y = ptd_frac_y * ptd * self.camera.local_y();

            // Move the camera
            self.camera
                .set_target(self.camera.target() + move_x + move_y);
        }
    }

    fn apply_orbit(&mut self) {
        if let DragState::Dragging {
            ref mut params,
            last_pos,
            current_pos,
            ..
        } = self.orbit_drag_state
        {
            // Move the camera around the orbit point according to the mouse movement
            let (x, y) = (current_pos.x - last_pos.x, current_pos.y - last_pos.y);

            params.rotation = params.rotation
                * (Quat::from_axis_angle(
                    params.starting_camera.local_y(),
                    deg(x * ORBIT_SENSITIVITY),
                ) + Quat::from_axis_angle(
                    params.starting_camera.local_x(),
                    deg(y * ORBIT_SENSITIVITY),
                ));

            params.rotation = params.rotation.normalize();

            let mut camera = params.starting_camera.clone();
            camera.rotate_around(self.orbit_point, params.rotation);

            self.camera = camera;
        }
    }
}
