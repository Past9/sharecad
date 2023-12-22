use std::cell::OnceCell;

use space::{point2, Mat33, Point2, Point3, Vec2, Vec3};

use crate::{
    axes, axes_der1, axes_der2, Curve, Curve3, Curve3Impl, CurvePoint, CurvePointAxes, ISurface,
    ISurfacePoint,
};

use super::helpers::{
    ff1, ff2, gaussian_curvature, mean_curvature, normal_curvature, principal_curvatures,
};

pub struct SweepSurface {
    profile: Curve,
    path: Curve,
}
impl SweepSurface {
    pub fn new(profile: Curve, path: Curve) -> Self {
        Self { profile, path }
    }
}
impl<'a> ISurface<'a> for SweepSurface {
    type Point = SweepPoint<'a>;

    fn domain(&self) -> (Point2, Point2) {
        let (u_min, u_max) = self.profile.domain();
        let (v_min, v_max) = self.path.domain();
        (point2(u_min, v_min), point2(u_max, v_max))
    }

    fn point(&'a self, uv: Point2) -> Self::Point {
        SweepPoint::new(&self, uv)
    }
}

pub struct SweepPoint<'a> {
    /*
    // These should go away once we implement curve points
    path_start: OnceCell<Point3>,
    path_axes_start: OnceCell<(Mat33, Mat33, Mat33)>,
    path_axes_start_inverse: OnceCell<Mat33>,
    profile_pos: OnceCell<Point3>,
    path_pos: OnceCell<Point3>,
    profile_der1: OnceCell<Vec3>,
    path_der1: OnceCell<Vec3>,
    profile_der2: OnceCell<Vec3>,
    path_der2: OnceCell<Vec3>,
    */
    profile_u: CurvePoint<'a>,
    path_v: CurvePoint<'a>,
    path_start: CurvePoint<'a>,

    path_axes_start_inverse_mat: OnceCell<Mat33>,

    path_axes: OnceCell<(Vec3, Vec3, Vec3)>,
    path_axes_mat: OnceCell<Mat33>,

    path_axes_der1: OnceCell<(Vec3, Vec3, Vec3)>,
    path_axes_der1_mat: OnceCell<Mat33>,

    path_axes_der2: OnceCell<(Vec3, Vec3, Vec3)>,
    path_axes_der2_mat: OnceCell<Mat33>,

    sweep: &'a SweepSurface,
    uv: Point2,
    //path_axes: OnceCell<(Mat33, Mat33, Mat33)>,
    eval: OnceCell<Point3>,
    der1: OnceCell<(Vec3, Vec3)>,
    der2: OnceCell<(Vec3, Vec3, Vec3)>,
    ff1: OnceCell<(f64, f64, f64)>,
    ff2: OnceCell<(f64, f64, f64)>,
    normal_curvature: OnceCell<f64>,
    mean_curvature: OnceCell<f64>,
    gaussian_curvature: OnceCell<f64>,
    principal_curvatures: OnceCell<(f64, f64)>,
}
impl<'a> SweepPoint<'a> {
    pub fn new(sweep: &'a SweepSurface, uv: Point2) -> Self {
        Self {
            /*
            path_start: OnceCell::new(),
            path_axes_start: OnceCell::new(),
            profile_pos: OnceCell::new(),
            path_pos: OnceCell::new(),
            profile_der1: OnceCell::new(),
            path_der1: OnceCell::new(),
            profile_der2: OnceCell::new(),
            path_der2: OnceCell::new(),
             */
            profile_u: sweep.profile.point(uv.u()),
            path_v: sweep.path.point(uv.v()),
            path_start: sweep.path.point(sweep.domain().0.y),

            path_axes_start_inverse_mat: OnceCell::new(),
            path_axes: OnceCell::new(),
            path_axes_mat: OnceCell::new(),
            path_axes_der1: OnceCell::new(),
            path_axes_der1_mat: OnceCell::new(),
            path_axes_der2: OnceCell::new(),
            path_axes_der2_mat: OnceCell::new(),

            sweep,
            uv,
            //path_axes: OnceCell::new(),
            eval: OnceCell::new(),
            der1: OnceCell::new(),
            der2: OnceCell::new(),
            ff1: OnceCell::new(),
            ff2: OnceCell::new(),
            normal_curvature: OnceCell::new(),
            mean_curvature: OnceCell::new(),
            gaussian_curvature: OnceCell::new(),
            principal_curvatures: OnceCell::new(),
        }
    }

    pub fn path_axes(&self) -> &(Vec3, Vec3, Vec3) {
        self.path_axes
            .get_or_init(|| axes(&self.path_v, &self.sweep.path.never_tangent()))
    }

    pub fn path_axes_mat(&self) -> &Mat33 {
        self.path_axes_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn path_axes_der1(&self) -> &(Vec3, Vec3, Vec3) {
        self.path_axes_der1.get_or_init(|| {
            axes_der1(
                &self.path_v,
                &self.sweep.path.never_tangent(),
                self.path_axes(),
            )
        })
    }

    pub fn path_axes_der1_mat(&self) -> &Mat33 {
        self.path_axes_der1_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes_der1();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn path_axes_der2(&self) -> &(Vec3, Vec3, Vec3) {
        self.path_axes_der2.get_or_init(|| {
            axes_der2(
                &self.path_v,
                &self.sweep.path.never_tangent(),
                self.path_axes(),
                self.path_axes_der1(),
            )
        })
    }

    pub fn path_axes_der2_mat(&self) -> &Mat33 {
        self.path_axes_der2_mat.get_or_init(|| {
            let (x, y, z) = *self.path_axes_der2();
            Mat33::from_axes(x, y, z)
        })
    }

    pub fn path_axes_start_inverse_mat(&self) -> &Mat33 {
        self.path_axes_start_inverse_mat.get_or_init(|| {
            let (x, y, z) = axes(&self.path_start, self.sweep.path.never_tangent());
            let matrix = Mat33::from_axes(x, y, z);
            matrix.inverse().unwrap()
        })
    }

    /*
    pub fn path_axes_start_inverse(&'a self) -> &Mat33 {
        self.path_axes_start_inverse
            .get_or_init(|| self.path_start.axes().clone().inverse().unwrap())
    }
     */

    /*
    pub fn path_der1(&self) -> &Vec3 {
        self.path_der1
            .get_or_init(|| self.sweep.path.der1(self.v()))
    }

    pub fn path_der2(&self) -> &Vec3 {
        self.path_der2
            .get_or_init(|| self.sweep.path.der2(self.v()))
    }

    pub fn path_axes_start_inverse(&self) -> &Mat33 {
        self.path_axes_start_inverse
            .get_or_init(|| self.path_axes_start().0.inverse().unwrap())
    }

    pub fn profile_pos(&self) -> &Point3 {
        self.profile_pos
            .get_or_init(|| self.sweep.profile.eval(self.u()))
    }

    pub fn path_pos(&self) -> &Point3 {
        self.path_pos.get_or_init(|| self.sweep.path.eval(self.v()))
    }

    pub fn path_axes_start(&self) -> &(Mat33, Mat33, Mat33) {
        self.path_axes_start.get_or_init(|| {
            let (Point2 { y: v_min, .. }, _) = self.sweep.domain();
            self.calc_path_axes(v_min)
        })
    }

    pub fn path_start(&self) -> &Point3 {
        self.path_start.get_or_init(|| {
            let (Point2 { y: v_min, .. }, _) = self.sweep.domain();
            self.sweep.path.eval(v_min)
        })
    }

    pub fn path_axes(&self) -> &(Mat33, Mat33, Mat33) {
        self.path_axes
            .get_or_init(|| self.calc_path_axes(self.uv.v()))
    }
    */

    /*
    fn calc_path_axes(&self, v: f64) -> (Mat33, Mat33, Mat33) {
        let der1 = self.sweep.path.der1(v);
        let d = self.sweep.path.never_tangent();

        // Compute axes of local coordinate system
        let (i1, i2, i3, d2) = {
            let i1 = der1.normalize();

            let d2 = d - (i1.dot(d)) * i1;

            let i2 = d2.normalize();
            let i3 = i1.cross(i2);

            (i1, i2, i3, d2)
        };

        let der2 = self.sweep.path.der2(v);

        // Compute first derivatives of axes
        let (i1_der1, i2_der1, i3_der1, d2_der1) = {
            let i1_der1 = der1.norm_der1(der2);

            //let d2_der1 = -i1 * (i1_der1.dot(d));
            let d2_der1 = (-i1_der1.dot(d) * i1) - (i1.dot(d) * i1_der1);
            let i2_der1 = d2.norm_der1(d2_der1);

            let i3_der1 = i1.cross(i2_der1) + i1_der1.cross(i2);

            (i1_der1, i2_der1, i3_der1, d2_der1)
        };

        // Compute second derivatives of axes
        let (i1_der2, i2_der2, i3_der2) = {
            let der3 = self.sweep.path.der3(v);

            let i1_der2 = der1.norm_der2(der2, der3);

            //let d2_der2 = -i1 * (i1_der2.dot(d));
            let d2_der2 =
                (-i1_der2.dot(d) * i1) - 2.0 * (i1_der1.dot(d) * i1_der1) - (i1.dot(d) * i1_der2);
            let i2_der2 = d2.norm_der2(d2_der1, d2_der2);

            let i3_der2 = i1.cross(i2_der2) + 2.0 * i1_der1.cross(i2_der1) + i1_der2.cross(i2);

            (i1_der2, i2_der2, i3_der2)
        };

        (
            Mat33::from_axes(i1, i2, i3),
            Mat33::from_axes(i1_der1, i2_der1, i3_der1),
            Mat33::from_axes(i1_der2, i2_der2, i3_der2),
        )
    }
    */
}
impl<'a> ISurfacePoint for SweepPoint<'a> {
    fn uv(&self) -> Point2 {
        self.uv
    }

    fn eval(&self) -> &Point3 {
        self.eval.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            self.path_v.eval() + m * (self.profile_u.eval() - self.path_start.eval())
        })
    }

    fn der1(&self) -> &(Vec3, Vec3) {
        self.der1.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            let du = m * self.profile_u.der1();

            let m_der1 = self.path_axes_der1_mat() * self.path_axes_start_inverse_mat();
            let dv = self.path_v.der1() + m_der1 * (self.profile_u.eval() - self.path_start.eval());

            (du, dv)
        })
    }

    fn der2(&self) -> &(Vec3, Vec3, Vec3) {
        self.der2.get_or_init(|| {
            let m = self.path_axes_mat() * self.path_axes_start_inverse_mat();
            let duu = m * self.profile_u.der2();

            let m_der1 = self.path_axes_der1_mat() * self.path_axes_start_inverse_mat();
            let duv = m_der1 * self.profile_u.der1();

            let m_der2 = self.path_axes_der2_mat() * self.path_axes_start_inverse_mat();
            let dvv =
                self.path_v.der2() + m_der2 * (self.profile_u.eval() - self.path_start.eval());

            (duu, duv, dvv)
        })
    }

    fn ff1(&self) -> &(f64, f64, f64) {
        self.ff1.get_or_init(|| ff1(self))
    }

    fn ff2(&self) -> &(f64, f64, f64) {
        self.ff2.get_or_init(|| ff2(self))
    }

    fn normal_curvature(&self, direction: Vec2) -> f64 {
        *self
            .normal_curvature
            .get_or_init(|| normal_curvature(self, direction))
    }

    fn mean_curvature(&self) -> f64 {
        *self.mean_curvature.get_or_init(|| mean_curvature(self))
    }

    fn gaussian_curvature(&self) -> f64 {
        *self
            .gaussian_curvature
            .get_or_init(|| gaussian_curvature(self))
    }

    fn principal_curvatures(&self) -> &(f64, f64) {
        self.principal_curvatures
            .get_or_init(|| principal_curvatures(self))
    }
}
