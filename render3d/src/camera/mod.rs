mod controller;

pub use controller::*;

use bytemuck::{Pod, Zeroable};
use space::{Angle, Mat44, Point3, Quat, Vec3};

pub struct Eye {
    pub dist: f64,
    pub location: Point3,
}

#[derive(Debug)]
pub struct Camera {
    target: Point3,
    target_radius: f64,
    clip_radius: f64,
    to_eye: Vec3,
    up: Vec3,
    half_fov: Angle,
}
impl Camera {
    pub fn new(
        target: Point3,
        target_radius: f64,
        clip_radius: f64,
        to_eye: Vec3,
        up: Vec3,
        fov: Angle,
    ) -> Self {
        Self {
            target,
            target_radius,
            clip_radius,
            to_eye: to_eye.normalize(),
            up: up.normalize(),
            half_fov: fov / 2.0,
        }
    }

    pub fn rotate_around(&mut self, orbit: Point3, rotation: Quat) {
        let orbit_to_target = self.target - orbit;
        self.set_target(orbit + rotation * orbit_to_target);
        self.set_to_eye(rotation * self.to_eye);
        self.up = (rotation * self.up).normalize();
    }

    pub fn forward(&self) -> Vec3 {
        -self.to_eye
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    pub fn right(&self) -> Vec3 {
        self.up.cross(self.forward()).normalize()
    }

    pub fn local_z(&self) -> Vec3 {
        self.forward()
    }

    pub fn local_y(&self) -> Vec3 {
        self.up()
    }

    pub fn local_x(&self) -> Vec3 {
        self.right()
    }

    pub fn target_radius(&self) -> f64 {
        self.target_radius
    }

    pub fn planar_target_radius(&self) -> f64 {
        match self.is_ortho() {
            true => self.target_radius,
            false => self.eye_dist() * self.half_fov.tan(),
        }
    }

    pub fn target(&self) -> Point3 {
        self.target
    }

    pub fn set_target(&mut self, target: Point3) {
        self.target = target;
    }

    pub fn set_to_eye(&mut self, to_eye: Vec3) {
        self.to_eye = to_eye.normalize();
    }

    pub fn eye_dist(&self) -> f64 {
        match self.is_ortho() {
            true => self.clip_radius,
            false => self.target_radius / self.half_fov.sin(),
        }
    }

    pub fn eye(&self) -> Eye {
        let dist = self.eye_dist();
        Eye {
            dist,
            location: self.target + self.to_eye * dist,
        }
    }

    pub fn set_target_radius(&mut self, target_radius: f64) {
        self.target_radius = target_radius;
    }

    pub fn fov(&self) -> Angle {
        self.half_fov * 2.0
    }

    pub fn set_fov(&mut self, fov: Angle) {
        self.half_fov = fov / 2.0;
    }

    pub fn near(&self) -> f64 {
        match self.is_ortho() {
            true => -self.clip_radius,
            false => 0.1,
        }
    }

    pub fn far(&self) -> f64 {
        match self.is_ortho() {
            true => self.clip_radius,
            false => self.eye().dist + self.clip_radius,
        }
    }

    pub fn is_ortho(&self) -> bool {
        self.half_fov.is_zero()
    }

    pub fn to_raw(&self, aspect: f64) -> CameraUniform {
        let eye_pos = self.eye().location;

        let view_position = [eye_pos.x as f32, eye_pos.y as f32, eye_pos.z as f32, 1.0];
        let view_proj = self.build_view_projection_matrix(aspect).transpose().into();
        let zfar = (self.far() - self.near()) as f32;

        CameraUniform {
            view_position,
            view_proj,
            zfar,
            _padding: [0; 3],
        }
    }

    pub fn build_view_projection_matrix(&self, aspect: f64) -> Mat44 {
        let eye = self.eye();
        let view = Mat44::look_at_rh(eye.location, self.target, self.up);
        let proj = match self.is_ortho() {
            true => Self::orthographic_matrix(
                aspect,
                -self.target_radius,
                self.target_radius,
                -self.target_radius,
                self.target_radius,
                self.near(),
                self.far(),
            ),
            false => Self::perspective_matrix(
                (self.half_fov * 2.0).into(),
                aspect,
                self.near(),
                self.far(),
            ),
        };

        proj * view
    }

    fn orthographic_matrix(
        aspect: f64,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> Mat44 {
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
        let c2r2 = 1.0 / (far - near);
        let c2r3 = 0.0;

        let c3r0 = -(right + left) / (right - left);
        let c3r1 = -(top + bottom) / (top - bottom);
        let c3r2 = -(far + near) / (far - near);
        let c3r3 = 1.0;

        /*
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat44::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
        */

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat44::new(
            c0r0, c1r0, c2r0, c3r0,
            c0r1, c1r1, c2r1, c3r1,
            c0r2, c1r2, c2r2, c3r2,
            c0r3, c1r3, c2r3, c3r3,
        )
    }

    fn perspective_matrix(fovy: Angle, aspect: f64, near: f64, far: f64) -> Mat44 {
        let f = (fovy / 2.0).cot();

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
        let c2r2 = (far + near) / (far - near);
        let c2r3 = 1.0;

        let c3r0 = 0.0;
        let c3r1 = 0.0;
        let c3r2 = (2.0 * far * near) / (far - near);
        let c3r3 = 1.0;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat44::new(
            c0r0, c1r0, c2r0, c3r0,
            c0r1, c1r1, c2r1, c3r1,
            c0r2, c1r2, c2r2, c3r2,
            c0r3, c1r3, c2r3, c3r3,
        )
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
