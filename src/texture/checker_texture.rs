use crate::vec3::Vec3;

use crate::texture::Texture;

use std::sync::Arc;

pub struct CheckerTexture<T1: Texture + ?Sized, T2: Texture + ?Sized> {
    pub(crate) odd: Arc<T1>,
    pub(crate) even: Arc<T2>,
}

impl<T1: Texture + ?Sized, T2: Texture + ?Sized> CheckerTexture<T1, T2> {
    pub fn new(odd: Arc<T1>, even: Arc<T2>) -> Arc<Self> {
        Arc::from(Self { odd, even })
    }
}

impl<T1: Texture + ?Sized, T2: Texture + ?Sized> Texture for CheckerTexture<T1, T2> {
    fn color(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines = (10. * p.x_r).sin() * (10. * p.y_g).sin() * (10. * p.z_b).sin();
        if sines < 0. {
            self.odd.color(u, v, p)
        } else {
            self.even.color(u, v, p)
        }
    }
}
