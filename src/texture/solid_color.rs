use super::Texture;

use crate::vec3::Vec3;

pub struct SolidColor {
    pub(crate) color_value: Vec3,
}

impl SolidColor {
    pub fn new(color_value: Vec3) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
        self.color_value
    }
}
