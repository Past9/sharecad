mod bsp;

use std::{collections::BTreeSet, time::Instant};

use geometry::{
    Curve3, Curve3Impl, ISurface, ISurfacePoint, Surface3, Surface3Impl, SweepPoint, SweepSurface,
};
use render::model::{CurveMesh, SurfaceMesh, SurfaceVertex};
use space::{lerp, point2, vec2, Coincidence, Point2, Point3, Vec3};

use crate::bsp::{BspTree, TreeSplit};

#[derive(Clone, Debug)]
pub struct CurvePoint {
    pub u: f64,
    pub pos: Point3,
}
impl PartialEq for CurvePoint {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u
    }
}
impl Eq for CurvePoint {}
impl PartialOrd for CurvePoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.u.partial_cmp(&other.u)
    }
}
impl Ord for CurvePoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.u.total_cmp(&other.u)
    }
}

pub struct Curve3Tesselator<'a> {
    curve: &'a Curve3,
    points: BTreeSet<CurvePoint>,
}
impl<'a> Curve3Tesselator<'a> {
    pub fn new(curve: &'a Curve3) -> Self {
        let points = BTreeSet::from_iter([
            CurvePoint {
                u: curve.u_min(),
                pos: curve.eval(curve.u_min()),
            },
            CurvePoint {
                pos: curve.eval(curve.u_max()),
                u: curve.u_max(),
            },
        ]);

        Self { curve, points }
    }

    pub fn mesh(&self) -> CurveMesh {
        CurveMesh::new(self.points.iter().map(|v| v.pos).collect())
    }

    pub fn curve(&self) -> &Curve3 {
        &self.curve
    }

    pub fn vertices(&self) -> &BTreeSet<CurvePoint> {
        &self.points
    }

    pub fn insert_with_pos(&mut self, u: f64, pos: Point3) {
        self.points.insert(CurvePoint { u, pos });
    }

    pub fn insert(&mut self, u: f64) {
        self.insert_with_pos(u, self.curve.eval(u));
    }

    pub fn tessellate(&mut self, tolerance: f64) {
        let u_max = self.curve.u_max();
        let mut u = self.curve.u_min();
        let mut params = vec![];

        while u < u_max {
            u += self.delta_u(u, tolerance);
            if u < u_max {
                params.push(u);
            }
        }

        for param in params.into_iter() {
            self.insert(param);
        }
    }

    pub fn delta_u(&mut self, u: f64, tolerance: f64) -> f64 {
        let der1 = self.curve.der1(u);
        let der2 = self.curve.der2(u);

        let p = der1.magnitude().powi(3) / (der1.cross(der2)).magnitude();

        2.0 * (tolerance * (2.0 * p - tolerance)).sqrt() / der1.magnitude()
    }
}

pub struct SurfaceVert {
    pub u: f64,
    pub v: f64,
    pub pos: Point3,
    pub tangents: (Vec3, Vec3),
    pub normal: Vec3,
}

pub struct SurfacePointTessellator<'a> {
    surface: &'a SweepSurface,
    points: Vec<SurfaceVert>,
    indices: Vec<u32>,
}
impl<'a> SurfacePointTessellator<'a> {
    pub fn new(surface: &'a SweepSurface) -> Self {
        Self {
            surface,
            points: vec![],
            indices: vec![],
        }
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    pub fn mesh(&self) -> SurfaceMesh {
        SurfaceMesh::new(
            self.points
                .iter()
                .map(|p| SurfaceVertex {
                    position: p.pos,
                    tex_coords: point2(p.u, p.v),
                    normal: p.normal,
                    tangent: p.tangents.0,
                    bitangent: p.tangents.1,
                    param_coords: vec2(p.u, p.v),
                })
                .collect(),
            self.indices.clone(),
        )
    }

    pub fn tess_uvs3(&self, tolerance: f64) -> Vec<Point2> {
        let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = self.surface.domain();

        let mut bsp = BspTree::new(v_max, v_min, u_min, u_max);

        let start = Instant::now();
        bsp.split_spaces(&|n: f64, s: f64, w: f64, e: f64| {
            //
            let nw = point2(w, n);
            let ne = point2(e, n);
            let sw = point2(w, s);
            let se = point2(e, s);

            let sp_nw = self.surface.point(nw);
            let sp_ne = self.surface.point(ne);
            let sp_sw = self.surface.point(sw);
            let sp_se = self.surface.point(se);

            // U curvature
            if self.delta_u(&sp_nw, tolerance) < (ne - nw).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if self.delta_u(&sp_ne, tolerance) < (nw - ne).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if self.delta_u(&sp_sw, tolerance) < (se - sw).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if self.delta_u(&sp_se, tolerance) < (sw - se).magnitude() {
                return Some(TreeSplit::Ew);
            }

            // V curvature
            if self.delta_v(&sp_nw, tolerance) < (nw - sw).magnitude() {
                return Some(TreeSplit::Ns);
            }

            if self.delta_v(&sp_sw, tolerance) < (sw - nw).magnitude() {
                return Some(TreeSplit::Ns);
            }

            if self.delta_v(&sp_ne, tolerance) < (ne - se).magnitude() {
                return Some(TreeSplit::Ns);
            }

            if self.delta_v(&sp_se, tolerance) < (se - ne).magnitude() {
                return Some(TreeSplit::Ns);
            }

            None
        });
        let end = Instant::now();
        println!("BSP tree in {}us", (end - start).as_micros());

        let mut params = vec![
            point2(u_min, v_min),
            point2(u_min, v_max),
            point2(u_max, v_min),
            point2(u_max, v_max),
        ];

        bsp.visit_splits(
            &mut |n: f64, s: f64, w: f64, e: f64, split: TreeSplit| match split {
                TreeSplit::Ew => {
                    let u = (w + e) / 2.0;
                    params.push(point2(u, n));
                    params.push(point2(u, s));
                }
                TreeSplit::Ns => {
                    let v = (n + s) / 2.0;
                    params.push(point2(w, v));
                    params.push(point2(e, v));
                }
            },
        );

        params
    }

    pub fn find_dv(&self, u: f64, v: f64) -> Option<Vec3> {
        let func = |u: f64, v: f64| -> Vec3 {
            let point = self.surface.point(point2(u, v));
            let (_, dv) = point.der1();
            dv.normalize()
        };

        let (Point2 { x: u_min, .. }, Point2 { x: u_max, .. }) = self.surface.domain();

        const START_DIST: f64 = 0.1;

        let max_rows = 40;

        let end_u = u;
        let start_u = {
            let dist_to_max = (u_max - end_u).abs();
            let dist_to_min = (u_min - end_u).abs();

            //end_u + START_DIST

            if dist_to_max < dist_to_min {
                // If closer to top of U range, start from below
                end_u - START_DIST
            } else {
                // Otherwise start from above
                end_u + START_DIST
            }
        };

        let initial_h = (end_u - start_u).abs();
        let mut h = initial_h;

        let mut a = vec![vec![Vec3::ZERO; max_rows]; max_rows];

        let test_u = lerp(start_u, end_u, 1.0 - h);
        a[0][0] = func(test_u, v);

        let mut solution = None;

        for i in 0..max_rows - 1 {
            h = h / 2.0;

            let test_u = lerp(start_u, end_u, 1.0 - h);

            a[i + 1][0] = func(test_u, v);

            for j in 0..=i {
                let num = 4f64.powi(j as i32 + 1) * a[i + 1][j] - a[i][j];
                let den = 4f64.powi(j as i32 + 1) - 1.0;
                a[i + 1][j + 1] = num / den;
            }

            let latest = a[i + 1][i + 1];
            let previous = a[i][i];

            if (latest - previous).magnitude() < 0.0001 {
                solution = Some(latest);
                break;
            }
        }

        solution
    }

    pub fn tessellate(&mut self, tolerance: f64) {
        let uv_flat = self
            .tess_uvs3(tolerance)
            .into_iter()
            .map(|p| delaunator::Point { x: p.x, y: p.y })
            .collect::<Vec<_>>();

        // Compute the Delaunay triangulation in UV space
        let uv_mesh = delaunator::triangulate(&uv_flat);

        // Set the mesh indices
        self.indices = uv_mesh.triangles.into_iter().map(|i| i as u32).collect();

        // Calculate the mesh vertices in Euclidean space
        self.points = uv_flat
            .into_iter()
            .filter_map(|uv| {
                let point = self.surface.point(point2(uv.x, uv.y));
                let (du, dv) = point.der1();

                let dv = if dv.cc(Vec3::ZERO) {
                    self.find_dv(uv.x, uv.y)
                } else {
                    Some(*dv)
                };

                if let Some(dv) = dv {
                    let tangent = du.normalize();
                    let bitangent = dv.normalize();
                    let normal = du.cross(dv).normalize();
                    Some(SurfaceVert {
                        u: uv.x,
                        v: uv.y,
                        pos: *point.eval(),
                        tangents: (tangent, bitangent),
                        normal: normal,
                    })
                } else {
                    panic!("no solution");
                    None
                }
            })
            .collect();
    }

    pub fn delta_u(&self, point: &SweepPoint, tolerance: f64) -> f64 {
        let du = point.der1().0;
        let duu = point.der2().0;

        let k = du.cross(duu).magnitude() / du.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / du.magnitude()
    }

    pub fn delta_v(&self, point: &SweepPoint, tolerance: f64) -> f64 {
        let dv = point.der1().1;
        let dvv = point.der2().2;

        let k = dv.cross(dvv).magnitude() / dv.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / dv.magnitude()
    }
}

pub struct Surface3Tessellator<'a> {
    surface: &'a Surface3,
    points: Vec<SurfaceVert>,
    indices: Vec<u32>,
}
impl<'a> Surface3Tessellator<'a> {
    pub fn new(surface: &'a Surface3) -> Self {
        Self {
            surface,
            points: vec![],
            indices: vec![],
        }
    }

    pub fn num_points(&self) -> usize {
        self.points.len()
    }

    pub fn mesh(&self) -> SurfaceMesh {
        SurfaceMesh::new(
            self.points
                .iter()
                .map(|p| SurfaceVertex {
                    position: p.pos,
                    tex_coords: point2(p.u, p.v),
                    normal: p.normal,
                    tangent: p.tangents.0,
                    bitangent: p.tangents.1,
                    param_coords: vec2(p.u, p.v),
                })
                .collect(),
            self.indices.clone(),
        )
    }

    pub fn tess_uvs3(&self, tolerance: f64) -> Vec<Point2> {
        let u_min = self.surface.u_min();
        let v_min = self.surface.v_min();
        let u_max = self.surface.u_max();
        let v_max = self.surface.v_max();

        let mut bsp = BspTree::new(v_max, v_min, u_min, u_max);

        let start = Instant::now();
        bsp.split_spaces(&|n: f64, s: f64, w: f64, e: f64| {
            //
            let nw = point2(w, n);
            let ne = point2(e, n);
            let sw = point2(w, s);
            let se = point2(e, s);

            // U curvature
            if self.delta_u(nw.x, nw.y, tolerance) < (ne - nw).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if self.delta_u(ne.x, ne.y, tolerance) < (nw - ne).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if self.delta_u(sw.x, sw.y, tolerance) < (se - sw).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if self.delta_u(se.x, se.y, tolerance) < (sw - se).magnitude() {
                return Some(TreeSplit::Ew);
            }

            // V curvature
            if self.delta_v(nw.x, nw.y, tolerance) < (nw - sw).magnitude() {
                return Some(TreeSplit::Ns);
            }

            if self.delta_v(sw.x, sw.y, tolerance) < (sw - nw).magnitude() {
                return Some(TreeSplit::Ns);
            }

            if self.delta_v(ne.x, ne.y, tolerance) < (ne - se).magnitude() {
                return Some(TreeSplit::Ns);
            }

            if self.delta_v(se.x, se.y, tolerance) < (se - ne).magnitude() {
                return Some(TreeSplit::Ns);
            }

            None
        });
        let end = Instant::now();
        println!("BSP tree in {}us", (end - start).as_micros());

        let mut params = vec![
            point2(u_min, v_min),
            point2(u_min, v_max),
            point2(u_max, v_min),
            point2(u_max, v_max),
        ];

        bsp.visit_splits(
            &mut |n: f64, s: f64, w: f64, e: f64, split: TreeSplit| match split {
                TreeSplit::Ew => {
                    let u = (w + e) / 2.0;
                    params.push(point2(u, n));
                    params.push(point2(u, s));
                }
                TreeSplit::Ns => {
                    let v = (n + s) / 2.0;
                    params.push(point2(w, v));
                    params.push(point2(e, v));
                }
            },
        );

        params
    }

    pub fn find_dv(&self, u: f64, v: f64) -> Option<Vec3> {
        let func = |u: f64, v: f64| -> Vec3 {
            let (_, dv) = self.surface.der1(u, v);
            dv.normalize()
        };

        let u_max = self.surface.u_max();
        let u_min = self.surface.u_min();
        const START_DIST: f64 = 0.1;

        let max_rows = 40;

        let end_u = u;
        let start_u = {
            let dist_to_max = (u_max - end_u).abs();
            let dist_to_min = (u_min - end_u).abs();

            //end_u + START_DIST

            if dist_to_max < dist_to_min {
                // If closer to top of U range, start from below
                end_u - START_DIST
            } else {
                // Otherwise start from above
                end_u + START_DIST
            }
        };

        let initial_h = (end_u - start_u).abs();
        let mut h = initial_h;

        let mut a = vec![vec![Vec3::ZERO; max_rows]; max_rows];

        let test_u = lerp(start_u, end_u, 1.0 - h);
        a[0][0] = func(test_u, v);

        let mut solution = None;

        for i in 0..max_rows - 1 {
            h = h / 2.0;

            let test_u = lerp(start_u, end_u, 1.0 - h);

            a[i + 1][0] = func(test_u, v);

            for j in 0..=i {
                let num = 4f64.powi(j as i32 + 1) * a[i + 1][j] - a[i][j];
                let den = 4f64.powi(j as i32 + 1) - 1.0;
                a[i + 1][j + 1] = num / den;
            }

            let latest = a[i + 1][i + 1];
            let previous = a[i][i];

            if (latest - previous).magnitude() < 0.0001 {
                solution = Some(latest);
                break;
            }
        }

        solution
    }

    pub fn tessellate(&mut self, tolerance: f64) {
        let uv_flat = self
            .tess_uvs3(tolerance)
            .into_iter()
            .map(|p| delaunator::Point { x: p.x, y: p.y })
            .collect::<Vec<_>>();

        // Compute the Delaunay triangulation in UV space
        let uv_mesh = delaunator::triangulate(&uv_flat);

        // Set the mesh indices
        self.indices = uv_mesh.triangles.into_iter().map(|i| i as u32).collect();

        // Calculate the mesh vertices in Euclidean space
        self.points = uv_flat
            .into_iter()
            .filter_map(|uv| {
                let (du, dv) = self.surface.der1(uv.x, uv.y);

                let dv = if dv.cc(Vec3::ZERO) {
                    self.find_dv(uv.x, uv.y)
                } else {
                    Some(dv)
                };

                if let Some(dv) = dv {
                    let tangent = du.normalize();
                    let bitangent = dv.normalize();
                    let normal = du.cross(dv).normalize();
                    Some(SurfaceVert {
                        u: uv.x,
                        v: uv.y,
                        pos: self.surface.eval(uv.x, uv.y),
                        tangents: (tangent, bitangent),
                        normal: normal,
                    })
                } else {
                    panic!("no solution");
                    None
                }
            })
            .collect();
    }

    pub fn delta_u(&self, u: f64, v: f64, tolerance: f64) -> f64 {
        let du = self.surface.der1(u, v).0;
        let duu = self.surface.der2(u, v).0;

        let k = du.cross(duu).magnitude() / du.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / du.magnitude()
    }

    pub fn delta_v(&self, u: f64, v: f64, tolerance: f64) -> f64 {
        let dv = self.surface.der1(u, v).1;
        let dvv = self.surface.der2(u, v).2;

        let k = dv.cross(dvv).magnitude() / dv.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / dv.magnitude()
    }
}
