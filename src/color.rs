use super::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Color::new(r, g, b)
    }

    pub fn r(&self) -> f64 {
        self.x()
    }

    pub fn g(&self) -> f64 {
        self.y()
    }

    pub fn b(&self) -> f64 {
        self.z()
    }

    pub fn format(&self) -> String {
        let str = format!("{r} {g} {b}",
                          r = (255.999 * self.r()) as i32,
                          g = (255.999 * self.g()) as i32,
                          b = (255.999 * self.b()) as i32);
        str
    }
}
