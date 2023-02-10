use crate::vec3::Vec3;

use super::Texture;

use crate::random::Random;

use crate::perlin::Perlin;

pub struct NoiseTexture {
    pub(crate) noise: Perlin,
    pub(crate) scale: f64,
}

impl NoiseTexture {
    pub fn new(rng: &mut Random<f64>, scale: f64) -> Self {
        Self {
            noise: Perlin::new(rng),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn color(&self, _u: f64, _v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z_b + 10.0 * self.noise.turb(&p, 7)))
    }
}
