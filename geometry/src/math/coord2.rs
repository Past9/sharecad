use super::{Point2, Vec2};

pub fn coord2(o: Point2, x: Vec2, y: Vec2) -> Coord2 {
    Coord2::new(o, x, y)
}

#[derive(Debug, Clone)]
pub struct Coord2 {
    pub o: Point2,
    pub x: Vec2,
    pub y: Vec2,
}
impl Coord2 {
    pub fn global() -> Self {
        Self {
            o: Point2::ZERO,
            x: Vec2::UNIT_X,
            y: Vec2::UNIT_Y,
        }
    }

    pub fn new(o: Point2, x: Vec2, y: Vec2) -> Self {
        let x = x.normalize();
        let y = y.normalize();

        if x.dot(y) != 0.0 {
            panic!("2D axes X and Y are not orthogonal");
        }

        Self { o, x, y }
    }

    /*
    pub fn to_mat33(&self) -> Mat33 {
        // Arithmetic average of angle between local and global X-axes
        let x_axis_angle = (self.x.y.asin() + self.x.x.acos()) / 2.0;
        // Arithmetic average of angle between local and global Y-axes
        let y_axis_angle = (self.y.x.asin() + self.y.y.acos()) / 2.0;

        // Arithmetic average of X and Y angles
        let angle = (x_axis_angle + y_axis_angle) / 2.0;

        // Get individual transformation matrices
        let rotation = Mat33::rotation(rad(angle));
        let translation = Mat33::translation(self.o - Point2::ZERO);

        // Combined matrix
        translation * rotation
    }

    pub fn from_mat33(&self, m: Mat33) -> Self {
        // Zero the translation elements and apply to rotation to global axis vectors
        let rotation_only = Mat33::new(
            m[0][0], m[0][1], 0.0, //
            m[1][0], m[1][1], 0.0, //
            m[2][0], m[2][1], m[2][2], //
        );

        // Apply rotations to global axis vectors to get local coordinate system axis vectors
        let x = Vec2::UNIT_X.to_point().transform(&rotation_only) - Point2::ZERO;
        let y = Vec2::UNIT_Y.to_point().transform(&rotation_only) - Point2::ZERO;

        // The origin of the local system is just the translation part of the matrix
        let o = point2(m[0][2], m[1][2]);

        Self::new(o, x, y)
    }
    */
}
