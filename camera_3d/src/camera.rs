use bytemuck::{Pod, Zeroable};
use cgmath::{point3, Angle, InnerSpace, Matrix4, Point3, Rad, Zero};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
);

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}
impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
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
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Cam) {
        let eye = camera.eye();
        let forward = camera.target - eye.location;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera.set_to_eye(eye.location + forward_norm * self.speed - point3(0.0, 0.0, 0.0));
        }
        if self.is_backward_pressed {
            camera.set_to_eye(eye.location - forward_norm * self.speed - point3(0.0, 0.0, 0.0));
        }

        let right = forward_norm.cross(camera.up());

        let forward = camera.target - eye.location;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            camera.set_to_eye(
                camera.target
                    - (forward + right * self.speed).normalize() * forward_mag
                    - point3(0.0, 0.0, 0.0),
            );
        }

        if self.is_left_pressed {
            camera.set_to_eye(
                camera.target
                    - (forward - right * self.speed).normalize() * forward_mag
                    - point3(0.0, 0.0, 0.0),
            );
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
    zfar: f32,
    _padding: [u32; 3],
}
impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
            zfar: 1.0,
            _padding: [0; 3],
        }
    }

    pub fn update_view_proj(&mut self, camera: &Cam, aspect: f32) {
        self.view_position = camera.eye().location.to_homogeneous().into();
        self.view_proj = camera.build_view_projection_matrix(aspect).into();
        self.zfar = camera.far() - camera.near();
    }
}

pub struct Eye {
    pub dist: f32,
    pub location: Point3<f32>,
}

pub struct Cam {
    target: cgmath::Point3<f32>,
    target_radius: f32,
    clip_radius: f32,
    to_eye: cgmath::Vector3<f32>,
    eye_up: cgmath::Vector3<f32>,
    half_fov: cgmath::Deg<f32>,
}
impl Cam {
    pub fn new(
        target: cgmath::Point3<f32>,
        target_radius: f32,
        clip_radius: f32,
        to_eye: cgmath::Vector3<f32>,
        eye_up: cgmath::Vector3<f32>,
        fov: cgmath::Deg<f32>,
    ) -> Self {
        Self {
            target,
            target_radius,
            clip_radius,
            to_eye: to_eye.normalize(),
            eye_up,
            half_fov: fov / 2.0,
        }
    }

    pub fn up(&self) -> cgmath::Vector3<f32> {
        self.eye_up
    }

    pub fn set_to_eye(&mut self, to_eye: cgmath::Vector3<f32>) {
        self.to_eye = to_eye.normalize();
    }

    pub fn eye(&self) -> Eye {
        let dist = match !self.is_ortho() {
            true => self.target_radius / self.half_fov.sin(),
            false => self.clip_radius,
        };

        Eye {
            dist,
            location: self.target + self.to_eye * dist,
        }
    }

    pub fn set_target_radius(&mut self, target_radius: f32) {
        self.target_radius = target_radius;
    }

    pub fn set_fov(&mut self, fov: cgmath::Deg<f32>) {
        //
    }

    pub fn set_min_target_depth(&mut self, min_target_depth: f32) {
        //
    }

    pub fn near(&self) -> f32 {
        match self.is_ortho() {
            true => -self.clip_radius,
            false => 0.1,
        }
    }

    pub fn far(&self) -> f32 {
        match self.is_ortho() {
            true => self.clip_radius,
            false => self.eye().dist + self.clip_radius,
        }
    }

    pub fn is_ortho(&self) -> bool {
        self.half_fov.is_zero()
    }

    pub fn build_view_projection_matrix(&self, aspect: f32) -> cgmath::Matrix4<f32> {
        let eye = self.eye();
        let (view, proj) = if !self.is_ortho() {
            let view = cgmath::Matrix4::look_at_rh(eye.location, self.target, self.eye_up);
            let proj = Self::perspective_matrix(
                (self.half_fov * 2.0).into(),
                aspect,
                self.near(),
                self.far(),
            );

            (view, proj)
        } else {
            let view = cgmath::Matrix4::look_at_rh(eye.location, self.target, self.eye_up);
            let proj = Self::orthographic_matrix(
                aspect,
                -self.target_radius,
                self.target_radius,
                -self.target_radius,
                self.target_radius,
                self.near(),
                self.far(),
            );

            (view, proj)
        };

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    fn orthographic_matrix(
        aspect: f32,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Matrix4<f32> {
        let c0r0 = match aspect > 1.0 {
            true => 2.0 / (right - left) / aspect,
            false => 2.0 / (right - left),
        };
        let c0r1 = 0.0;
        let c0r2 = 0.0;
        let c0r3 = 0.0;

        let c1r0 = 0.0;
        let c1r1 = match aspect > 1.0 {
            true => 2.0 / (top - bottom),
            false => aspect * 2.0 / (top - bottom),
        };
        let c1r2 = 0.0;
        let c1r3 = 0.0;

        let c2r0 = 0.0;
        let c2r1 = 0.0;
        let c2r2 = -1.0 / (far - near);
        let c2r3 = 0.0;

        let c3r0 = -(right + left) / (right - left);
        let c3r1 = -(top + bottom) / (top - bottom);
        let c3r2 = -(far + near) / (far - near);
        let c3r3 = 1.0;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }

    fn perspective_matrix(fovy: Rad<f32>, aspect: f32, near: f32, far: f32) -> Matrix4<f32> {
        let f = Rad::cot(fovy / 2.0);

        let c0r0 = match aspect > 1.0 {
            true => f / aspect,
            false => f,
        };
        let c0r1 = 0.0;
        let c0r2 = 0.0;
        let c0r3 = 0.0;

        let c1r0 = 0.0;
        let c1r1 = match aspect > 1.0 {
            true => f,
            false => f * aspect,
        };
        let c1r2 = 0.0;
        let c1r3 = 0.0;

        let c2r0 = 0.0;
        let c2r1 = 0.0;
        let c2r2 = (far + near) / (near - far);
        let c2r3 = -1.0;

        let c3r0 = 0.0;
        let c3r1 = 0.0;
        let c3r2 = (2.0 * far * near) / (near - far);
        let c3r3 = 0.0;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }
}
