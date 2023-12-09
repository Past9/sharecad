use std::{
    cell::OnceCell,
    collections::{BTreeMap, LinkedList},
    iter::{self, Once},
};

use geometry::{Curve3, Curve3Impl};
use space::Point3;

#[derive(Debug, Clone)]
pub struct Vertex {
    u: f64,
    point: Point3,
}

#[derive(Debug, Clone)]
pub struct Segment<'a> {
    start: Vertex,
    end: Vertex,
    dist: OnceCell<f64>,
    curve: &'a Curve3,
}

pub struct TessellatedCurve3<'a> {
    curve: &'a Curve3,
    segments: Vec<Segment<'a>>,
}
impl<'a> TessellatedCurve3<'a> {
    pub fn new(curve: &'a Curve3) -> Self {
        let u_min = curve.u_min();
        let u_max = curve.u_max();

        let segment = Segment {
            start: Vertex {
                u: u_min,
                point: curve.eval(u_min),
            },
            end: Vertex {
                u: u_max,
                point: curve.eval(u_max),
            },
            dist: OnceCell::new(),
            curve,
        };

        Self {
            curve,
            segments: vec![segment],
        }
    }

    pub fn refine<F: Fn(&Segment) -> Vec<Vertex>>(&self, refiner: F) -> Self {
        Self {
            curve: self.curve,
            segments: self
                .segments
                .iter()
                .flat_map(|segment| {
                    let new_verts = refiner(segment);

                    let mut new_segments = Vec::with_capacity(new_verts.len() + 1);

                    new_segments.push(Segment {
                        start: segment.start.clone(),
                        dist: OnceCell::new(),
                        curve: self.curve,

                        // Will be overwritten by the next vert from new_verts
                        // or by segment.end() if there are none.
                        end: Vertex {
                            u: 0.0,
                            point: Point3::ZERO,
                        },
                    });

                    for new_vert in new_verts.into_iter() {
                        let ns_last_index = new_segments.len() - 1;
                        new_segments[ns_last_index].end = new_vert;
                        new_segments.push(Segment {
                            start: new_segments[ns_last_index].start.clone(),
                            dist: OnceCell::new(),
                            curve: self.curve,

                            // Will be overwritten by the next vert from new_verts
                            // or by segment.end() if there are no more.
                            end: Vertex {
                                u: 0.0,
                                point: Point3::ZERO,
                            },
                        });
                    }

                    let ns_last_index = new_segments.len() - 1;
                    new_segments[ns_last_index].end = segment.end.clone();

                    new_segments
                })
                .collect(),
        }
    }

    pub fn points(&self) -> Vec<Point3> {
        let mut points = Vec::with_capacity(self.segments.len() + 1);

        points.push(self.segments[0].start.point);
        points.extend(self.segments.iter().map(|segment| segment.end.point));

        points
    }
}
