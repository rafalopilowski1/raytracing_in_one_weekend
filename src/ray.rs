use crate::vec3::Vec3;

#[derive(Clone, Copy, Default, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    #[inline(always)]
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
    #[inline(always)]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}
