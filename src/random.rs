use std::ops::{Add, Mul, Sub};

use rand::distributions::Uniform;

use rand::{rngs::ThreadRng, Rng};

pub struct Random<
    T: rand::distributions::uniform::SampleUniform
        + PartialOrd
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Copy,
> {
    pub rng: ThreadRng,
    pub uniform: Uniform<T>,
}

impl<
        T: rand::distributions::uniform::SampleUniform
            + PartialOrd
            + Sub<Output = T>
            + Mul<Output = T>
            + Add<Output = T>
            + Copy,
    > Random<T>
where
    T::Sampler: Copy,
{
    pub fn new(rng: ThreadRng, uniform: Uniform<T>) -> Self {
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
