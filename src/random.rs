use std::ops::{Add, Mul, Sub};

use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

pub struct Random<
    'a,
    T: rand::distributions::uniform::SampleUniform
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Copy,
> {
    pub rng: &'a mut ThreadRng,
    pub uniform: Uniform<T>,
}

impl<
        'a,
        T: rand::distributions::uniform::SampleUniform
            + Sub<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Copy,
    > Random<'a, T>
where
    T::Sampler: Copy,
{
    pub fn new(rng: &'a mut ThreadRng, uniform: Uniform<T>) -> Self {
        Self { rng, uniform }
    }
    pub fn random(&mut self, min: Option<T>, max: Option<T>) -> T {
        let sample = self.rng.sample(self.uniform);
        match (min, max) {
            (Some(min), Some(max)) => min + sample * (max - min),
            _ => sample,
        }
    }
}
