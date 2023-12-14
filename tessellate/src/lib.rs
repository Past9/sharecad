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

    pub fn tess(&mut self, tolerance: f64) {
        //
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
                    true => row[row.len() - 1].u() < u_next,
                    false => true,
                };

                if add_uv {
                    row.push(point2(u_next, row_end.v()));
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
            .map(|uv| SurfacePoint {
                u: uv.x,
                v: uv.y,
                pos: self.surface.eval(uv.x, uv.y),
                tangents: self.surface.tangents(uv.x, uv.y),
                normal: self.surface.normal(uv.x, uv.y),
            })
            .collect();
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
