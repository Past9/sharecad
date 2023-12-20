use std::collections::BTreeSet;

use geometry::{Curve3, Curve3Impl, Helix, Surface3, Surface3Impl};
use render::model::{CurveMesh, SurfaceMesh, SurfaceVertex};
use space::{point2, vec2, Coincidence, Point2, Point3, Vec2, Vec3};

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

    pub fn tess_uvs(&mut self, tolerance: f64) -> Vec<Vec<Point2>> {
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
                //

                //let normal = self.surface.normal(uv.x, uv.y).normalize();
                let (du, dv) = self.surface.der1(uv.x, uv.y);

                let dv = if dv.cc(Vec3::ZERO) {
                    /*
                    let func = |u: f64, v: f64| -> Vec3 {
                        let (du, dv) = self.surface.der1(u, v);
                        du.cross(dv).normalize()
                    };
                    */
                    let func = |u: f64, v: f64| -> Vec3 {
                        let (_, dv) = self.surface.der1(u, v);
                        dv.normalize()
                    };

                    let u = uv.x;
                    let v = uv.y;

                    let u_max = self.surface.u_max();
                    let u_min = self.surface.u_min();
                    const START_DIST: f64 = 0.1;

                    let max_rows = 40;

                    let end_u = u;
                    let start_u = {
                        let dist_to_max = (self.surface.u_max() - end_u).abs();
                        let dist_to_min = (self.surface.u_min() - end_u).abs();

                        if u_max < u_min {
                            // If closer to top of U range, start from below
                            end_u - START_DIST
                        } else {
                            // Otherwise start from above
                            end_u + START_DIST
                        }
                    };

                    let initial_h = start_u - end_u;
                    let mut found_solution = false;

                    let mut h = initial_h;

                    let mut a = vec![vec![Vec3::ZERO; max_rows]; max_rows];

                    //a[0][0] = self.surface.der1(h, v).1;
                    a[0][0] = func(h, v);

                    let mut solution = None;

                    for i in 0..max_rows - 1 {
                        h = h / 2.0;

                        //a[i + 1][0] = self.surface.der1(h, v).1;
                        a[i + 1][0] = func(h, v);

                        for j in 0..=i {
                            let num = 4f64.powi(j as i32 + 1) * a[i + 1][j] - a[i][j];
                            let den = 4f64.powi(j as i32 + 1) - 1.0;
                            a[i + 1][j + 1] = num / den;
                        }

                        let latest = a[i + 1][i + 1];
                        let previous = a[i][i];

                        /*
                        println!("");
                        println!("i = {}", i);
                        println!("latest = {}", latest);
                        println!("previous = {}", previous);
                        println!("latest - previous = {}", latest - previous);
                        println!("magnitude = {}", (latest - previous).magnitude());
                        */

                        if (latest - previous).magnitude() < 0.001 {
                            //println!("solved dv = {}", a[i + 1][i + 1]);
                            solution = Some(latest);
                        } else {
                            //println!("searching dv = {}", a[i + 1][i + 1]);
                        }
                    }

                    //panic!("a = {:#?}", a);

                    /*
                    if !found_solution {
                        panic!("NO SOLUTION");
                    }
                     */

                    solution
                } else {
                    Some(dv)
                };

                if let Some(dv) = dv {
                    let tangent = du.normalize();
                    let bitangent = dv.normalize();
                    let normal = du.cross(dv).normalize();
                    Some(SurfacePoint {
                        u: uv.x,
                        v: uv.y,
                        pos: self.surface.eval(uv.x, uv.y),
                        tangents: (tangent, bitangent),
                        normal: normal, //tangents: self.surface.tangents(uv.x, uv.y),
                                        //normal: self.surface.normal(uv.x, uv.y),
                    })
                } else {
                    None
                }

                //let tangent = du.normalize();
                //let bitangent = normal.cross(tangent).normalize();

                //let normal = Vec3::UNIT_Z;
                /*
                let tangent = Vec3::UNIT_X;
                let bitangent = Vec3::UNIT_Z;

                Some(SurfacePoint {
                    u: uv.x,
                    v: uv.y,
                    pos: self.surface.eval(uv.x, uv.y),
                    tangents: (tangent, bitangent),
                    normal: normal, //tangents: self.surface.tangents(uv.x, uv.y),
                                    //normal: self.surface.normal(uv.x, uv.y),
                })
                 */
            })
            .collect();
    }

    /*
    pub fn delta_u(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        let (e, _f, _g) = self.surface.ff1(u, v);
        let (l, _m, _n) = self.surface.ff2(u, v);

        2.0 * (tolerance * (2.0 * (e / l) - tolerance)).sqrt() / e.sqrt()
    }
     */

    pub fn delta_u(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        return 0.5;

        let du = self.surface.der1(u, v).0;
        let duu = self.surface.der2(u, v).0;

        let k = du.cross(duu).magnitude() / du.magnitude().powi(3);
        let p = k.recip();

        2.0 * (tolerance * (2.0 * (p) - tolerance)).sqrt() / du.magnitude()
    }

    pub fn delta_v(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        return 0.5;

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
