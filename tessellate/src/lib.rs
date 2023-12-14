use std::collections::BTreeSet;

use geometry::{Curve3, Curve3Impl, Helix, Surface3, Surface3Impl};
use render::model::CurveMesh;
use space::{vec2, Coincidence, Point3, Vec2};

#[derive(Clone, Debug)]
pub struct CurveVertex {
    pub u: f64,
    pub pos: Point3,
}
impl PartialEq for CurveVertex {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u
    }
}
impl Eq for CurveVertex {}
impl PartialOrd for CurveVertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.u.partial_cmp(&other.u)
    }
}
impl Ord for CurveVertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.u.total_cmp(&other.u)
    }
}

pub struct Curve3Tesselator<'a> {
    curve: &'a Curve3,
    vertices: BTreeSet<CurveVertex>,
}
impl<'a> Curve3Tesselator<'a> {
    pub fn new(curve: &'a Curve3) -> Self {
        let vertices = BTreeSet::from_iter([
            CurveVertex {
                u: curve.u_min(),
                pos: curve.eval(curve.u_min()),
            },
            CurveVertex {
                pos: curve.eval(curve.u_max()),
                u: curve.u_max(),
            },
        ]);

        Self { curve, vertices }
    }

    pub fn mesh(&self) -> CurveMesh {
        CurveMesh::new(self.vertices.iter().map(|v| v.pos).collect())
    }

    pub fn curve(&self) -> &Curve3 {
        &self.curve
    }

    pub fn vertices(&self) -> &BTreeSet<CurveVertex> {
        &self.vertices
    }

    pub fn insert_with_pos(&mut self, u: f64, pos: Point3) {
        self.vertices.insert(CurveVertex { u, pos });
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

    pub fn tesselate_to_dist(&mut self, tolerance: f64) {
        match self.curve {
            Curve3::Helix(helix) => self.tessellate_helix_to_dist(helix, tolerance),
            Curve3::Line(line) => {
                // Line is already tesselated by the constructor, don't need
                // to do anything
            }
        }
    }

    fn tessellate_helix_to_dist(&mut self, helix: &Helix, tolerance: f64) {
        let param_inc = 2.0 * (1.0 - tolerance / helix.r()).acos();

        let num_points = ((helix.u_len() / param_inc).ceil() - 1.0) as usize;

        for i in 1..=num_points {
            let param = i as f64 * param_inc;
            self.insert(param);
        }
    }

    pub fn tessellate_to_tolerance(&mut self, tol: f64) {
        const MAX_ITER: u32 = 100;
        let mut remaining = MAX_ITER;
        while self.tessellate_to_tolerance_once(tol) && remaining > 0 {
            remaining -= 1;
        }
    }

    fn tessellate_to_tolerance_once(&mut self, tol: f64) -> bool {
        println!("\n\ntessellate_to_tolerance_once");

        let mut changed = false;
        let mut new_verts = self.vertices.clone();
        let mut vert_iter = self.vertices.iter();

        let mut last = vert_iter.next().unwrap();
        for cur in vert_iter {
            let deviation = self.curve.line_deviation(last.u, cur.u).unwrap();

            if deviation.distance.cc(0.0) {
                println!(
                    "deviation, last, cur {:#?}, {:#?}, {:#?}",
                    deviation, last, cur
                );
            }

            if deviation.distance > tol {
                new_verts.insert(CurveVertex {
                    u: deviation.uv.x,
                    pos: deviation.cu_pos,
                });
                changed = true;
            }
            last = cur;
        }

        self.vertices = new_verts;

        changed
    }
}

pub struct SurfaceVertex {
    pub u: f64,
    pub v: f64,
    pub pos: Point3,
}

pub struct Surface3Tessellator<'a> {
    surface: &'a Surface3,
    vertices: Vec<Vec<SurfaceVertex>>,
    points: Vec<Vec<Point3>>,
}
impl<'a> Surface3Tessellator<'a> {
    pub fn new(surface: &'a Surface3) -> Self {
        Self {
            surface,
            vertices: vec![],
            points: vec![],
        }
    }

    pub fn points(&self) -> &[Vec<Point3>] {
        &self.points
    }

    pub fn tessellate(&mut self, tolerance: f64) {
        let u_max = self.surface.u_max();
        let v_max = self.surface.v_max();
        let mut vertices = vec![];

        let mut u = self.surface.u_min();
        loop {
            //println!("\nu = {}", u);
            let mut row = vec![];

            let mut v = self.surface.v_min();
            loop {
                //println!("  v = {}", v);
                row.push(self.surface.eval(u, v));

                if v < v_max {
                    let delta_v = self.delta_v(u, v, tolerance);
                    //println!("rcv delta_v = {}", delta_v);
                    v = (v + delta_v).min(v_max);
                } else {
                    break;
                }
            }

            vertices.push(row);

            if u < u_max {
                let delta_u = self.delta_u(u, self.surface.v_min(), tolerance);
                u = (u + delta_u).min(u_max);
            } else {
                break;
            }
        }

        self.points = vertices;
    }

    pub fn tess(&mut self, tolerance: f64) {
        //
        let u_min = self.surface.u_min();
        let v_min = self.surface.v_min();
        let u_max = self.surface.u_max();
        let v_max = self.surface.v_max();
        let mut params: Vec<Vec<Vec2>> = vec![vec![vec2(u_min, v_min)]];

        loop {
            println!("params.len() = {}", params.len());
            let mut complete = true;
            for row in params.iter_mut() {
                // Add one vertex in the +U direction on reach row
                let row_end = row[row.len() - 1];
                let u_next =
                    (row_end.u() + self.delta_u(row_end.u(), row_end.v(), tolerance)).min(u_max);

                let add_uv = match u_next == u_max {
                    true => row[row.len() - 1].u() < u_next,
                    false => true,
                };

                if add_uv {
                    row.push(vec2(u_next, row_end.v()));
                    complete = false;
                }
            }

            // Add another row in the +V direction
            let mut new_row = vec![];
            for uv in params[params.len() - 1].iter() {
                let v_next = (uv.v() + self.delta_v(uv.u(), uv.v(), tolerance)).min(v_max);

                let add_uv = match v_next == v_max {
                    true => uv.v() < v_next,
                    false => true,
                };

                if add_uv {
                    new_row.push(vec2(uv.u(), v_next));
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

        println!("Params:");
        for row in params.iter() {
            println!("  {:?}", row);
        }
    }

    pub fn delta_u(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        let (der_u, der_v) = self.surface.der1(u, v);
        let (der_uu, _der_uv, _der_vv) = self.surface.der2(u, v);

        let f1a = der_u.magnitude2();
        let n = der_v.cross(der_u);
        let f2a = der_uu.dot(n);

        2.0 * (tolerance * (2.0 * (f1a / f2a) - tolerance)).sqrt() / f1a.sqrt()
    }

    pub fn delta_v(&mut self, u: f64, v: f64, tolerance: f64) -> f64 {
        let (der_u, der_v) = self.surface.der1(u, v);
        let (_der_uu, _der_uv, der_vv) = self.surface.der2(u, v);

        let f1c = der_v.magnitude2();
        let n = der_v.cross(der_u);
        let f2c = der_vv.dot(n);

        2.0 * (tolerance * (2.0 * (f1c / f2c) - tolerance)).sqrt() / f1c.sqrt()
    }
}
