mod bsp;

use std::collections::BTreeSet;

use geometry::{Curve3, Curve3Impl, Helix, Surface3, Surface3Impl};
use render::model::{CurveMesh, SurfaceMesh, SurfaceVertex};
use space::{lerp, point2, vec2, Coincidence, Point2, Point3, Vec2, Vec3};

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

pub struct SurfacePoint {
    pub u: f64,
    pub v: f64,
    pub pos: Point3,
    pub tangents: (Vec3, Vec3),
    pub normal: Vec3,
}

pub struct Surface3Tessellator<'a> {
    surface: &'a Surface3,
    points: Vec<SurfacePoint>,
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

        bsp.visit_spaces(&|n: f64, s: f64, w: f64, e: f64| {
            //
            let nw = point2(w, n);
            let ne = point2(e, n);
            let sw = point2(w, s);
            let se = point2(e, s);

            // U curvature
            if self.delta_u(nw.x, nw.y, tolerance) < (ne - nw).magnitude() {
                return TreeSplit::Ew;
            }

            if self.delta_u(ne.x, ne.y, tolerance) < (nw - ne).magnitude() {
                return TreeSplit::Ew;
            }

            if self.delta_u(sw.x, sw.y, tolerance) < (se - sw).magnitude() {
                return TreeSplit::Ew;
            }

            if self.delta_u(se.x, se.y, tolerance) < (sw - se).magnitude() {
                return TreeSplit::Ew;
            }

            // V curvature
            if self.delta_v(nw.x, nw.y, tolerance) < (nw - sw).magnitude() {
                return TreeSplit::Ns;
            }

            if self.delta_v(sw.x, sw.y, tolerance) < (sw - nw).magnitude() {
                return TreeSplit::Ns;
            }

            if self.delta_v(ne.x, ne.y, tolerance) < (ne - se).magnitude() {
                return TreeSplit::Ns;
            }

            if self.delta_v(se.x, se.y, tolerance) < (se - ne).magnitude() {
                return TreeSplit::Ns;
            }

            TreeSplit::None
        });

        println!("{:#?}", bsp);

        todo!()
    }

    pub fn tess_uvs2(&self, tolerance: f64) -> Vec<Point2> {
        let u_min = self.surface.u_min();
        let v_min = self.surface.v_min();
        let u_max = self.surface.u_max();
        let v_max = self.surface.v_max();

        let mut params = vec![
            delaunator::Point { x: u_min, y: v_min },
            delaunator::Point { x: u_min, y: v_max },
            delaunator::Point { x: u_max, y: v_min },
            delaunator::Point { x: u_max, y: v_max },
        ];

        for i in 0..1000 {
            let triangulation = delaunator::triangulate(&params);

            'tri: for i in 0..triangulation.triangles.len() / 3 {
                let t = i * 3;
                let edges = [
                    (triangulation.triangles[t], triangulation.triangles[t + 1]),
                    (
                        triangulation.triangles[t + 1],
                        triangulation.triangles[t + 2],
                    ),
                    (triangulation.triangles[t + 2], triangulation.triangles[t]),
                ];

                for edge in edges {
                    let p0 = &params[edge.0];
                    let p1 = &params[edge.1];

                    let p0 = point2(p0.x, p0.y);
                    let p1 = point2(p1.x, p1.y);

                    let edge_vec = p1 - p0;
                    let delta = self.delta(p0, edge_vec.normalize(), tolerance);

                    if !delta.x.is_nan()
                        && !delta.y.is_nan()
                        && delta.magnitude() < edge_vec.magnitude()
                    {
                        println!("edge_vec.magnitude() = {}", edge_vec.magnitude());
                        println!("delta.magnitude() = {}", delta.magnitude());
                        /*
                        let mut new_point = p0 + delta;

                        if new_point.x < u_min {
                            new_point.x = u_min;
                        }

                        if new_point.x > u_max {
                            new_point.x = u_max;
                        }

                        if new_point.y < v_min {
                            new_point.y = v_min;
                        }

                        if new_point.y > v_max {
                            new_point.y = v_max;
                        }
                         */

                        /*
                        if !params.iter().any(|p| point2(p.x, p.y).cc(new_point)) {
                            params.push(delaunator::Point {
                                x: new_point.x,
                                y: new_point.y,
                            });
                            println!("params.len() = {}", params.len());
                            break 'tri;
                        }
                          */

                        let np = (p0.into_vec() + p1.into_vec()) / 2.0;

                        params.push(delaunator::Point { x: np.x, y: np.y });
                        println!("params.len() = {}", params.len());
                        break 'tri;
                    }
                }
            }
        }

        params.iter().map(|p| point2(p.x, p.y)).collect()
    }

    pub fn tess_uvs(&self, tolerance: f64) -> Vec<Vec<Point2>> {
        let u_min = self.surface.u_min();
        let v_min = self.surface.v_min();
        let u_max = self.surface.u_max();
        let v_max = self.surface.v_max();
        let mut params: Vec<Vec<Point2>> = vec![vec![point2(u_min, v_min)]];

        loop {
            let mut complete = true;
            for row in params.iter_mut() {
                // Add one vertex in the +U direction on reach row
                let row_end = row[row.len() - 1];
                let u_next =
                    (row_end.u() + self.delta_u(row_end.u(), row_end.v(), tolerance)).min(u_max);

                let add_uv = match u_next == u_max {
                    true => row_end.u() < u_next,
                    false => true,
                };

                if add_uv {
                    row.push(point2(u_next, row_end.v()));
                    complete = false;
                }
            }

            // Add another row in the +V direction
            let mut new_row = vec![];
            let last_row = &params[params.len() - 1];
            for uv in last_row.iter() {
                let v_next = (uv.v() + self.delta_v(uv.u(), uv.v(), tolerance)).min(v_max);

                let add_uv = match v_next == v_max {
                    true => uv.v() < v_next,
                    false => true,
                };

                if add_uv {
                    new_row.push(point2(uv.u(), v_next));
                    complete = false;
                }
            }

            if new_row.len() > 0 {
                params.push(new_row);
            }

            if complete {
                break;
            }
        }

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

    pub fn tess(&mut self, tolerance: f64) {
        let params = self.tess_uvs(tolerance);

        /*
        for row in params.iter() {
            for uv in row.iter() {
                let (du, dv) = self.surface.der1(uv.u(), uv.v());

                println!("du.dv = {}", du.dot(dv));

                let normal = self.surface.normal(uv.u(), uv.v());
                //println!("{}, {}", normal, normal.magnitude());
                if normal.cc(Vec3::ZERO) {
                    //println!("zero normal @ {}", uv);
                }
            }
        }
         */

        // Flatten the UV points
        let uv_flat = params
            .into_iter()
            .flat_map(|row| {
                row.into_iter().map(|uv| delaunator::Point {
                    x: uv.u(),
                    y: uv.v(),
                })
            })
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
                    //let normal = du.cross(dv).normalize();
                    let normal = du.cross(dv).normalize();
                    Some(SurfacePoint {
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

    pub fn delta(&self, position: Point2, direction: Vec2, tolerance: f64) -> Vec2 {
        /*
        let c_num = self
            .surface
            .normal_curvature_num(position.u(), position.v(), direction);

        let c_den = self
            .surface
            .normal_curvature_den(position.u(), position.v(), direction);
        */
        let curvature = self
            .surface
            .normal_curvature(position.u(), position.v(), direction);

        let p = 1.0 / curvature;

        let (du, dv) = self.surface.der1(position.u(), position.v());
        let num = 2.0 * (tolerance * (2.0 * p - tolerance)).sqrt();

        let delta_u_den = (du + dv * (direction.v() / direction.u())).magnitude();
        let delta_v_den = (du * (direction.u() / direction.v()) + dv).magnitude();

        vec2(num / delta_u_den, num / delta_v_den)
    }

    /*
    pub fn delta_u(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        let (e, _f, _g) = self.surface.ff1(u, v);
        let (l, _m, _n) = self.surface.ff2(u, v);

        2.0 * (tolerance * (2.0 * (e / l) - tolerance)).sqrt() / e.sqrt()
    }
     */

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

    /*
    pub fn delta_v(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        let (_e, _f, g) = self.surface.ff1(u, v);
        let (_l, _m, n) = self.surface.ff2(u, v);

        // N may go to zero, causing division by zero!
        println!("u, v = {}, {}", u, v);
        println!("g = {}", g);
        println!("n = {}", n);
        println!("g / n = {}", g / n);
        println!("du = {}", self.surface.der1(u, v).0);
        println!("dv = {}", self.surface.der1(u, v).1);
        println!("duu = {}", self.surface.der2(u, v).0);
        println!("duv = {}", self.surface.der2(u, v).1);
        println!("dvv = {}", self.surface.der2(u, v).2);

        println!("ff1 = {:?}", self.surface.ff1(u, v));
        println!("ff2 = {:?}", self.surface.ff2(u, v));
        println!(
            "ff2 normal = {:?}",
            self.surface
                .der1(u, v)
                .1
                .cross(self.surface.der1(u, v).0)
                .normalize()
        );

        println!(
            "normal_curvature U = {:?}",
            self.surface.normal_curvature(u, v, vec2(1.0, 0.0))
        );
        println!(
            "normal_curvature V = {:?}",
            self.surface.normal_curvature(u, v, vec2(0.0, 1.0))
        );
        println!(
            "normal_curvature UV = {:?}",
            self.surface.normal_curvature(u, v, vec2(1.0, 1.0))
        );
        println!(
            "guassian_curvature = {:?}",
            self.surface.gaussian_curvature(u, v)
        );
        println!("mean_curvature = {:?}", self.surface.mean_curvature(u, v));
        println!(
            "principal_curvatures = {:?}",
            self.surface.principal_curvatures(u, v)
        );

        2.0 * (tolerance * (2.0 * (g / n) - tolerance)).sqrt() / g.sqrt()
    }
    */
}
