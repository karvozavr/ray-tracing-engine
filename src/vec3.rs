use std::ops::{Index, Mul, Add, Neg, Sub, Div};
use std::fmt;

pub type Point3 = Vec3;

#[derive(Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

/// a 3D vector
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn one() -> Self {
        Vec3::new(1.0, 1.0, 1.0)
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> f64 {
        dot(self, rhs)
    }

    #[inline]
    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    #[inline]
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn unit(self) -> Self {
        self / self.length()
    }

    #[inline]
    pub fn x(self) -> f64 {
        self.x
    }

    #[inline]
    pub fn y(self) -> f64 {
        self.y
    }

    #[inline]
    pub fn z(self) -> f64 {
        self.z
    }

    #[inline]
    pub fn zero() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3 {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
       self.x = source.x.clone();
       self.y = source.y.clone();
       self.z = source.z.clone();
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid vector index. Should be 0, 1 or 2."),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}