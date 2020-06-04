use super::vec3::{Vec3, Point3};

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    #[inline]
    pub fn origin(self) -> Point3 {
        self.origin
    }

    #[inline]
    pub fn direction(self) -> Vec3 {
        self.direction
    }

    #[inline]
    pub fn at(self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
