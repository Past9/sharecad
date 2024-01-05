use crate::{
    math::{lerp, point2, Coincidence, Point2, Point3, Vec3},
    primitives::{SurfacePoint, SurfaceSolver},
};

use super::bsp::{BspTree, TreeSplit};

pub struct SurfaceVert {
    pub u: f64,
    pub v: f64,
    pub pos: Point3,
    pub tangents: (Vec3, Vec3),
    pub normal: Vec3,
}

pub struct TessellatedSurface {
    pub points: Vec<SurfaceVert>,
    pub indices: Vec<u32>,
}
impl TessellatedSurface {
    pub fn by_tolerance(surface: &SurfaceSolver, tolerance: f64) -> Self {
        let uv_flat = Self::tess_uvs(surface, tolerance)
            .into_iter()
            .map(|p| delaunator::Point { x: p.x, y: p.y })
            .collect::<Vec<_>>();

        // Compute the Delaunay triangulation in UV space
        let uv_mesh = delaunator::triangulate(&uv_flat);

        // Set the mesh indices
        let indices: Vec<u32> = uv_mesh.triangles.into_iter().map(|i| i as u32).collect();

        // Calculate the mesh vertices in Euclidean space
        let points: Vec<SurfaceVert> = uv_flat
            .into_iter()
            .filter_map(|uv| {
                let point = surface.point(point2(uv.x, uv.y));
                let (du, dv) = point.der1();

                let dv = if dv.cc(Vec3::ZERO) {
                    Self::find_dv(surface, uv.x, uv.y)
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
                        pos: *point.pos(),
                        tangents: (tangent, bitangent),
                        normal: normal,
                    })
                } else {
                    panic!("no solution");
                }
            })
            .collect();

        Self { points, indices }
    }

    fn tess_uvs(surface: &SurfaceSolver, tolerance: f64) -> Vec<Point2> {
        let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = surface.domain();

        let mut bsp = BspTree::new(v_max, v_min, u_min, u_max);

        bsp.split_spaces(&|n: f64, s: f64, w: f64, e: f64| {
            //
            let nw = point2(w, n);
            let ne = point2(e, n);
            let sw = point2(w, s);
            let se = point2(e, s);

            // Fron NW corner, right and down
            let sp_nw = surface.point(nw);
            if Self::delta_u(&sp_nw, tolerance) < (ne - nw).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if Self::delta_v(&sp_nw, tolerance) < (nw - sw).magnitude() {
                return Some(TreeSplit::Ns);
            }

            // Fron SE corner, left and up
            let sp_se = surface.point(se);
            if Self::delta_u(&sp_se, tolerance) < (sw - se).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if Self::delta_v(&sp_se, tolerance) < (se - ne).magnitude() {
                return Some(TreeSplit::Ns);
            }

            // Fron NE corner, left and down
            let sp_ne = surface.point(ne);
            if Self::delta_u(&sp_ne, tolerance) < (nw - ne).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if Self::delta_v(&sp_ne, tolerance) < (ne - se).magnitude() {
                return Some(TreeSplit::Ns);
            }

            // Fron SW corner, right and up
            let sp_sw = surface.point(sw);
            if Self::delta_u(&sp_sw, tolerance) < (se - sw).magnitude() {
                return Some(TreeSplit::Ew);
            }

            if Self::delta_v(&sp_sw, tolerance) < (sw - nw).magnitude() {
                return Some(TreeSplit::Ns);
            }

            None
        });

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

    fn find_dv(surface: &SurfaceSolver, u: f64, v: f64) -> Option<Vec3> {
        let func = |u: f64, v: f64| -> Vec3 {
            let point = surface.point(point2(u, v));
            let (_, dv) = point.der1();
            dv.normalize()
        };

        let (Point2 { x: u_min, .. }, Point2 { x: u_max, .. }) = surface.domain();

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

    fn delta_u(point: &SurfacePoint, tolerance: f64) -> f64 {
        let du = point.der1().0;
        let duu = point.der2().0;

        let k = du.cross(duu).magnitude() / du.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / du.magnitude()
    }

    fn delta_v(point: &SurfacePoint, tolerance: f64) -> f64 {
        let dv = point.der1().1;
        let dvv = point.der2().2;

        let k = dv.cross(dvv).magnitude() / dv.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / dv.magnitude()
    }
}
