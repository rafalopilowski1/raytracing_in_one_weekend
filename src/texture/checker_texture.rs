use crate::vec3::Vec3;

use super::Texture;

use std::sync::Arc;

pub struct CheckerTexture {
    pub(crate) odd: Arc<dyn Texture>,
    pub(crate) even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10. * p.x_r).sin() * (10. * p.y_g).sin() * (10. * p.z_b).sin();
        if sines < 0. {
            self.odd.color(u, v, p)
        } else {
            self.even.color(u, v, p)
        }
    }
}
