use space::{rad, Angle, Mat44, Point3, Vec2, Vec3};

pub struct Camera {
    target: Point3,
    to_eye: Vec3,
    target_radius: f64,
    fov: f64,
    aspect_ratio: f64,
}
impl Camera {
    fn projection_matrix(&self) -> Mat44 {
        //
        todo!()
    }

    fn perspective_matrix(&self) -> Mat44 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use space::{point3, vec3, Mat44, Vec2};

    use super::*;

    #[test]
    fn projects_points() {
        let points = vec![
            // Front face, starting top left and going clockwise (relative to camera)
            point3(-2.0, 2.0, -2.0),
            point3(2.0, 2.0, -2.0),
            point3(2.0, -2.0, -2.0),
            point3(-2.0, -2.0, -2.0),
            // Back face, starting top left and going clockwise (relative to camera)
            point3(-2.0, 2.0, 2.0),
            point3(2.0, 2.0, 2.0),
            point3(2.0, -2.0, 2.0),
            point3(-2.0, -2.0, 2.0),
        ];

        let camera = Camera {
            target: point3(0.0, 0.0, 0.0),
            to_eye: vec3(0.0, 0.0, -1.0),
            target_radius: 6.0,
            aspect_ratio: 1.0,
            fov: 90.0,
        };

        let matrix = Mat44::IDENTITY;

        let projected = points
            .iter()
            .map(|p| p.transform(matrix))
            .collect::<Vec<_>>();

        for i in 0..points.len() {
            println!("{:?} => {:?}", points[i], projected[i]);
        }
    }
}
