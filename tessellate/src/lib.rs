use std::collections::BTreeSet;

use geometry::{Curve3, Curve3Impl, Helix};
use render::model::CurveMesh;
use space::{Coincidence, Point3};

#[derive(Clone, Debug)]
pub struct Vertex {
    pub u: f64,
    pub pos: Point3,
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u
    }
}
impl Eq for Vertex {}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.u.partial_cmp(&other.u)
    }
}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.u.total_cmp(&other.u)
    }
}

pub struct Curve3Tesselator<'a> {
    curve: &'a Curve3,
    vertices: BTreeSet<Vertex>,
}
impl<'a> Curve3Tesselator<'a> {
    pub fn new(curve: &'a Curve3) -> Self {
        let vertices = BTreeSet::from_iter([
            Vertex {
                u: curve.u_min(),
                pos: curve.eval(curve.u_min()),
            },
            Vertex {
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

    pub fn vertices(&self) -> &BTreeSet<Vertex> {
        &self.vertices
    }

    pub fn insert_with_pos(&mut self, u: f64, pos: Point3) {
        self.vertices.insert(Vertex { u, pos });
    }

    pub fn insert(&mut self, u: f64) {
        self.insert_with_pos(u, self.curve.eval(u));
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
                new_verts.insert(Vertex {
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
