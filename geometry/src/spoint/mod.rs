use std::cell::OnceCell;

use space::{point2, Mat33, Point2, Point3, Vec2, Vec3};

use crate::{Curve3, Curve3Impl};

pub trait ISurface<'a> {
    type Point: ISurfacePoint;

    fn domain(&self) -> (Point2, Point2);

    fn domain_span(&self) -> Vec2 {
        let (min, max) = self.domain();
        max - min
    }

    fn point(&'a self, at: Point2) -> Self::Point;
}

pub enum SurfacePoint<'a> {
    Sweep(SweepPoint<'a>),
}
impl<'a> SurfacePoint<'a> {
    pub fn u(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.u(),
        }
    }

    pub fn v(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.v(),
        }
    }

    pub fn uv(&self) -> Point2 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.uv(),
        }
    }

    pub fn eval(&self) -> &Point3 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.eval(),
        }
    }

    pub fn der1(&self) -> &(Vec3, Vec3) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.der1(),
        }
    }

    pub fn der2(&self) -> &(Vec3, Vec3, Vec3) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.der2(),
        }
    }

    pub fn ff1(&self) -> &(f64, f64, f64) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.ff1(),
        }
    }

    pub fn ff2(&self) -> &(f64, f64, f64) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.ff2(),
        }
    }

    pub fn normal_curvature(&self, direction: Vec2) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.normal_curvature(direction),
        }
    }

    pub fn mean_curvature(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.mean_curvature(),
        }
    }

    pub fn gaussian_curvature(&self) -> f64 {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.gaussian_curvature(),
        }
    }

    pub fn principal_curvatures(&self) -> &(f64, f64) {
        match self {
            SurfacePoint::Sweep(sweep) => sweep.principal_curvatures(),
        }
    }
}
impl<'a> From<SweepPoint<'a>> for SurfacePoint<'a> {
    fn from(point: SweepPoint<'a>) -> Self {
        Self::Sweep(point)
    }
}

pub enum Surface {
    Sweep(SweepSurface),
}
impl Surface {
    pub fn domain(&self) -> (Point2, Point2) {
        match self {
            Surface::Sweep(sweep) => sweep.domain(),
        }
    }

    pub fn point(&self, uv: Point2) -> SurfacePoint {
        match self {
            Surface::Sweep(sweep) => SurfacePoint::from(sweep.point(uv)),
        }
    }
}

pub struct SweepSurface {
    profile: Curve3,
    path: Curve3,
}
impl SweepSurface {
    pub fn new(profile: Curve3, path: Curve3) -> Self {
        Self { profile, path }
    }
}
impl<'a> ISurface<'a> for SweepSurface {
    type Point = SweepPoint<'a>;

    fn domain(&self) -> (Point2, Point2) {
        (
            point2(self.profile.u_min(), self.path.u_min()),
            point2(self.profile.u_max(), self.path.u_max()),
        )
    }

    fn point(&'a self, uv: Point2) -> Self::Point {
        SweepPoint::new(&self, uv)
    }
}

pub trait ISurfacePoint {
    fn u(&self) -> f64 {
        self.uv().u()
    }

    fn v(&self) -> f64 {
        self.uv().v()
    }

    fn uv(&self) -> Point2;
    fn eval(&self) -> &Point3;
    fn der1(&self) -> &(Vec3, Vec3);
    fn der2(&self) -> &(Vec3, Vec3, Vec3);
    fn ff1(&self) -> &(f64, f64, f64);
    fn ff2(&self) -> &(f64, f64, f64);
    fn normal_curvature(&self, direction: Vec2) -> f64;
    fn mean_curvature(&self) -> f64;
    fn gaussian_curvature(&self) -> f64;
    fn principal_curvatures(&self) -> &(f64, f64);
}

pub struct SweepPoint<'a> {
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

    sweep: &'a SweepSurface,
    uv: Point2,
    path_axes: OnceCell<(Mat33, Mat33, Mat33)>,
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
            path_start: OnceCell::new(),
            path_axes_start: OnceCell::new(),
            path_axes_start_inverse: OnceCell::new(),
            profile_pos: OnceCell::new(),
            path_pos: OnceCell::new(),
            profile_der1: OnceCell::new(),
            path_der1: OnceCell::new(),
            profile_der2: OnceCell::new(),
            path_der2: OnceCell::new(),

            sweep,
            uv,
            path_axes: OnceCell::new(),
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

    pub fn profile_der1(&self) -> &Vec3 {
        self.profile_der1
            .get_or_init(|| self.sweep.profile.der1(self.u()))
    }

    pub fn profile_der2(&self) -> &Vec3 {
        self.profile_der2
            .get_or_init(|| self.sweep.profile.der2(self.u()))
    }

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
}
impl<'a> ISurfacePoint for SweepPoint<'a> {
    fn uv(&self) -> Point2 {
        self.uv
    }

    fn eval(&self) -> &Point3 {
        self.eval.get_or_init(|| {
            let m = self.path_axes().0 * self.path_axes_start_inverse();
            self.path_pos() + m * (self.profile_pos() - self.path_start())
        })
    }

    fn der1(&self) -> &(Vec3, Vec3) {
        self.der1.get_or_init(|| {
            let m = self.path_axes().0 * self.path_axes_start_inverse();
            let du = m * self.profile_der1();

            let m_der1 = self.path_axes().1 * self.path_axes_start_inverse();
            let dv = self.path_der1() + m_der1 * (self.profile_pos() - self.path_start());

            (du, dv)
        })
    }

    fn der2(&self) -> &(Vec3, Vec3, Vec3) {
        self.der2.get_or_init(|| {
            let m = self.path_axes().0 * self.path_axes_start_inverse();
            let duu = m * self.profile_der2();

            let m_der1 = self.path_axes().1 * self.path_axes_start_inverse();
            let duv = m_der1 * self.profile_der1();

            let m_der2 = self.path_axes().2 * self.path_axes_start_inverse();
            let dvv = self.path_der2() + m_der2 * (self.profile_pos() - self.path_start());

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

fn ff1<P: ISurfacePoint>(point: &P) -> (f64, f64, f64) {
    let (du, dv) = point.der1();

    let e = du.dot(*du);
    let f = du.dot(*dv);
    let g = dv.dot(*dv);

    (e, f, g)
}

fn ff2<P: ISurfacePoint>(point: &P) -> (f64, f64, f64) {
    let (du, dv) = point.der1();
    let (duu, duv, dvv) = point.der2();

    let normal = dv.cross(*du).normalize();

    let l = duu.dot(normal);
    let m = duv.dot(normal);
    let n = dvv.dot(normal);

    (l, m, n)
}

fn normal_curvature<P: ISurfacePoint>(point: &P, direction: Vec2) -> f64 {
    let (e, f, g) = point.ff1();
    let (l, m, n) = point.ff2();

    let du2 = direction.u().powi(2);
    let dudv = direction.u() * direction.v();
    let dv2 = direction.v().powi(2);

    (l * du2 + 2.0 * m * dudv + n * dv2) / (e * du2 + 2.0 * f * dudv + g * dv2)
}

fn mean_curvature<P: ISurfacePoint>(point: &P) -> f64 {
    let (e, f, g) = point.ff1();
    let (l, m, n) = point.ff2();

    0.5 * (e * n - 2.0 * f * m + g * l) / (e * g - f.powi(2))
}

fn gaussian_curvature<P: ISurfacePoint>(point: &P) -> f64 {
    let (e, f, g) = point.ff1();
    let (l, m, n) = point.ff2();
    (l * n - m.powi(2)) / (e * g - f.powi(2))
}

fn principal_curvatures<P: ISurfacePoint>(point: &P) -> (f64, f64) {
    let h = point.mean_curvature();
    let k = point.gaussian_curvature();

    let root = (h.powi(2) - k).sqrt();

    (h + root, h - root)
}
