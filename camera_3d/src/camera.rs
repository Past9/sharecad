use bytemuck::{Pod, Zeroable};
use cgmath::{point3, vec3, Angle, InnerSpace, Point3, Zero};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
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
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye();
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera.set_to_eye(camera.eye() + forward_norm * self.speed - point3(0.0, 0.0, 0.0));
        }
        if self.is_backward_pressed {
            camera.set_to_eye(camera.eye() - forward_norm * self.speed - point3(0.0, 0.0, 0.0));
        }

        let right = forward_norm.cross(camera.up());

        let forward = camera.target - camera.eye();
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            camera.set_to_eye(
                camera.target
                    - (forward + right * self.speed).normalize() * forward_mag
                    - point3(0.0, 0.0, 0.0),
            );
            //camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }

        if self.is_left_pressed {
            camera.set_to_eye(
                camera.target
                    - (forward - right * self.speed).normalize() * forward_mag
                    - point3(0.0, 0.0, 0.0),
            );
            //camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}
impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Cam) {
        self.view_position = camera.eye().to_homogeneous().into();
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

pub struct Cam {
    target: cgmath::Point3<f32>,
    target_radius: f32,
    to_eye: cgmath::Vector3<f32>,
    eye_up: cgmath::Vector3<f32>,
    half_fov: cgmath::Deg<f32>,
    aspect_ratio: f32,
    znear: f32,
    zfar: f32,
}
impl Cam {
    pub fn new(
        target: cgmath::Point3<f32>,
        target_radius: f32,
        to_eye: cgmath::Vector3<f32>,
        eye_up: cgmath::Vector3<f32>,
        fov: cgmath::Deg<f32>,
        aspect_ratio: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            target,
            target_radius,
            to_eye: to_eye.normalize(),
            eye_up,
            half_fov: fov / 2.0,
            aspect_ratio,
            znear,
            zfar,
        }
    }

    pub fn up(&self) -> cgmath::Vector3<f32> {
        self.eye_up
    }

    pub fn set_to_eye(&mut self, to_eye: cgmath::Vector3<f32>) {
        self.to_eye = to_eye.normalize();
    }

    pub fn eye(&self) -> Point3<f32> {
        if self.half_fov > cgmath::Deg::zero() {
            let eye_dist = self.target_radius / self.half_fov.sin();
            point3(0.0, 0.0, 0.0) + (self.to_eye * eye_dist)
        } else if self.half_fov == cgmath::Deg::zero() {
            let eye_dist = self.target_radius;
            point3(0.0, 0.0, 0.0) + (self.to_eye * eye_dist)
        } else {
            panic!("Negative FOV");
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

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let (view, proj) = if self.half_fov > cgmath::Deg::zero() {
            let eye_dist = self.target_radius / self.half_fov.sin();
            let eye = point3(0.0, 0.0, 0.0) + (self.to_eye * eye_dist);
            let view = cgmath::Matrix4::look_at_rh(eye, self.target, self.eye_up);
            let proj = cgmath::perspective(
                self.half_fov * 2.0,
                self.aspect_ratio,
                self.znear,
                self.zfar,
            );

            (view, proj)
        } else if self.half_fov == cgmath::Deg::zero() {
            let eye_dist = self.target_radius;
            let eye = point3(0.0, 0.0, 0.0) + (self.to_eye * eye_dist);
            let view = cgmath::Matrix4::look_at_rh(eye, self.target, self.eye_up);
            let proj = cgmath::ortho(
                -self.target_radius,
                self.target_radius,
                -self.target_radius,
                self.target_radius,
                -self.target_radius - self.znear,
                self.target_radius + self.zfar,
            );
            (view, proj)
        } else {
            panic!("Negative FOV");
        };

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}
