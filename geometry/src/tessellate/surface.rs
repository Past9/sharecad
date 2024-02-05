use crate::{
    math::{vec2, Angle, Coincidence, Scalar, Vec2, Vec3},
    primitives::{ISurfacePoint, SurfacePoint, SurfaceSolver},
};

use super::{
    bsp::{BspTree, TreeSplit},
    TessellationTolerance,
};

pub struct SurfaceVert {
    pub u: f64,
    pub v: f64,
    pub pos: Vec3<f64>,
    pub tangents: (Vec3<f64>, Vec3<f64>),
    pub normal: Vec3<f64>,
}

pub struct TessellatedSurface {
    pub points: Vec<SurfaceVert>,
    pub indices: Vec<u32>,
}
impl TessellatedSurface {
    pub fn create_bsp(surface: &SurfaceSolver<f64>, tolerance: &TessellationTolerance) -> BspTree {
        let (Vec2 { x: u_min, y: v_min }, Vec2 { x: u_max, y: v_max }) = surface.domain();

        let mut bsp = BspTree::new(v_max, v_min, u_min, u_max);

        bsp.split_spaces(&|n: f64, s: f64, w: f64, e: f64| {
            //
            let nw = vec2(w, n);
            let ne = vec2(e, n);
            let sw = vec2(w, s);
            let se = vec2(e, s);

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

    pub fn create(surface: &SurfaceSolver<f64>, tolerance: &TessellationTolerance) -> Self {
        let (Vec2 { x: u_min, y: v_min }, Vec2 { x: u_max, y: v_max }) = surface.domain();

        // Get a BSP tree splitting the surface into quads by tolerance
        let bsp = Self::create_bsp(surface, tolerance);

        // Extract a list of parameter values from the BSP tree
        let mut params = vec![
            vec2(u_min, v_min),
            vec2(u_min, v_max),
            vec2(u_max, v_min),
            vec2(u_max, v_max),
        ];

        bsp.visit_splits(
            &mut |n: f64, s: f64, w: f64, e: f64, split: TreeSplit| match split {
                TreeSplit::Ew => {
                    let u = (w + e) / 2.0;
                    params.push(vec2(u, n));
                    params.push(vec2(u, s));
                }
                TreeSplit::Ns => {
                    let v = (n + s) / 2.0;
                    params.push(vec2(w, v));
                    params.push(vec2(e, v));
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
                let point = surface.point(vec2(uv.x, uv.y));
                let (du, dv) = point.der1();

                let dv = if dv.cc(Vec3::ZERO) {
                    surface.est_tangent_v(vec2(uv.x, uv.y))
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

    fn delta_u(point: &SurfacePoint<f64>, tolerance: &TessellationTolerance) -> f64 {
        match tolerance {
            TessellationTolerance::Distance(distance) => Self::delta_u_dist(point, *distance),
            TessellationTolerance::Angle(angle) => Self::delta_u_angle(point, *angle),
            TessellationTolerance::DistanceAndAngle(distance, angle) => {
                Self::delta_u_dist(point, *distance).min(Self::delta_u_angle(point, *angle))
            }
        }
    }

    fn delta_v(point: &SurfacePoint<f64>, tolerance: &TessellationTolerance) -> f64 {
        match tolerance {
            TessellationTolerance::Distance(distance) => Self::delta_v_dist(point, *distance),
            TessellationTolerance::Angle(angle) => Self::delta_v_angle(point, *angle),
            TessellationTolerance::DistanceAndAngle(distance, angle) => {
                Self::delta_v_dist(point, *distance).min(Self::delta_v_angle(point, *angle))
            }
        }
    }

    fn delta_u_angle(point: &SurfacePoint<f64>, angle: Angle<f64>) -> f64 {
        let (du, _) = point.der1();
        let p = point.curvature_u().recip();
        (p * angle.radians()) / du.magnitude()
    }

    fn delta_v_angle(point: &SurfacePoint<f64>, angle: Angle<f64>) -> f64 {
        let (_, dv) = point.der1();
        let p = point.curvature_v().recip();
        (p * angle.radians()) / dv.magnitude()
    }

    fn delta_u_dist(point: &SurfacePoint<f64>, dist: f64) -> f64 {
        let du = point.der1().0;
        let p = point.curvature_u().recip();
        2.0 * (dist * (2.0 * (p) - dist)).sqrt() / du.magnitude()
    }

    fn delta_v_dist(point: &SurfacePoint<f64>, dist: f64) -> f64 {
        let dv = point.der1().1;
        let p = point.curvature_v().recip();
        2.0 * (dist * (2.0 * (p) - dist)).sqrt() / dv.magnitude()
    }
}
