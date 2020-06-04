use crate::geometry::vec3::{Vec3, Point3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[inline]
    fn origin(self) -> Point3 {
        return self.origin
    }

    #[inline]
    fn direction(self) -> Vec3 {
        return self.direction
    }
}
