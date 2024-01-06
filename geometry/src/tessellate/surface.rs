use crate::{
    math::{lerp, point2, Angle, Coincidence, Point2, Point3, Vec3},
    primitives::{ISurfacePoint, SurfacePoint, SurfaceSolver},
};

use super::{
    bsp::{BspTree, TreeSplit},
    TessellationTolerance,
};

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
    pub fn create_bsp(surface: &SurfaceSolver, tolerance: &TessellationTolerance) -> BspTree {
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

        bsp
    }

    pub fn create(surface: &SurfaceSolver, tolerance: &TessellationTolerance) -> Self {
        let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = surface.domain();

        // Get a BSP tree splitting the surface into quads by tolerance
        let bsp = Self::create_bsp(surface, tolerance);

        // Extract a list of parameter values from the BSP tree
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

        // Convert the parameters into points for the `delaunator` crate
        let uv_flat = params
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
                    surface.est_tangent_v(point2(uv.x, uv.y))
                    //Self::find_dv(surface, uv.x, uv.y)
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

    fn tess_uvs(surface: &SurfaceSolver, tolerance: &TessellationTolerance) -> Vec<Point2> {
        let (Point2 { x: u_min, y: v_min }, Point2 { x: u_max, y: v_max }) = surface.domain();

        let bsp = Self::create_bsp(surface, tolerance);

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

    /*
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
    */

    fn delta_u(point: &SurfacePoint, tolerance: &TessellationTolerance) -> f64 {
        match tolerance {
            TessellationTolerance::Distance(distance) => Self::delta_u_dist(point, *distance),
            TessellationTolerance::Angle(angle) => Self::delta_u_angle(point, *angle),
            TessellationTolerance::DistanceAndAngle(distance, angle) => {
                Self::delta_u_dist(point, *distance).min(Self::delta_u_angle(point, *angle))
            }
        }
    }

    fn delta_v(point: &SurfacePoint, tolerance: &TessellationTolerance) -> f64 {
        match tolerance {
            TessellationTolerance::Distance(distance) => Self::delta_v_dist(point, *distance),
            TessellationTolerance::Angle(angle) => Self::delta_v_angle(point, *angle),
            TessellationTolerance::DistanceAndAngle(distance, angle) => {
                Self::delta_v_dist(point, *distance).min(Self::delta_v_angle(point, *angle))
            }
        }
    }

    fn delta_u_angle(point: &SurfacePoint, angle: Angle) -> f64 {
        let (du, _) = point.der1();
        let p = point.curvature_u().recip();
        (p * angle.radians()) / du.magnitude()
    }

    fn delta_v_angle(point: &SurfacePoint, angle: Angle) -> f64 {
        let (_, dv) = point.der1();
        let p = point.curvature_v().recip();
        (p * angle.radians()) / dv.magnitude()
    }

    fn delta_u_dist(point: &SurfacePoint, dist: f64) -> f64 {
        let du = point.der1().0;
        let p = point.curvature_u().recip();
        2.0 * (dist * (2.0 * (p) - dist)).sqrt() / du.magnitude()
    }

    fn delta_v_dist(point: &SurfacePoint, dist: f64) -> f64 {
        let dv = point.der1().1;
        let p = point.curvature_v().recip();
        2.0 * (dist * (2.0 * (p) - dist)).sqrt() / dv.magnitude()
    }
}
