use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

use image::{Pixel, Rgb};

use crate::random::Random;

#[derive(PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x_r: f64,
    pub y_g: f64,
    pub z_b: f64,
}

impl Vec3 {
    pub fn new(x_r: f64, y_g: f64, z_b: f64) -> Self {
        Self { x_r, y_g, z_b }
    }
    #[inline(always)]
    pub fn length(v: Self) -> f64 {
        v.x_r.hypot(v.y_g).hypot(v.z_b)
    }
    #[inline(always)]
    pub fn length_squared(v: Vec3) -> f64 {
        Vec3::length(v).powi(2)
    }

    pub fn cross(v: Self, rhs: Self) -> Self {
        Self {
            x_r: v.y_g * rhs.z_b - v.z_b * rhs.y_g,
            y_g: v.z_b * rhs.x_r - v.x_r * rhs.z_b,
            z_b: v.x_r * rhs.y_g - v.y_g * rhs.x_r,
        }
    }
    #[inline(always)]
    pub fn dot(v: Self, rhs: Self) -> f64 {
        v.x_r * rhs.x_r + v.y_g * rhs.y_g + v.z_b * rhs.z_b
    }
    #[inline(always)]
    pub fn unit_vector(v: Vec3) -> Self {
        v / Vec3::length(v)
    }

    pub fn random(rng: &mut Random<f64>, min: Option<f64>, max: Option<f64>) -> Vec3 {
        let rng_x = rng.random(min, max);
        let rng_y = rng.random(min, max);
        let rng_z = rng.random(min, max);
        Self {
            x_r: rng_x,
            y_g: rng_y,
            z_b: rng_z,
        }
    }

    pub fn random_in_unit_sphere(rng: &mut Random<f64>) -> Self {
        let min = -1.0;
        let max = 1.0;
        let rng_x = rng.random(Some(min), Some(max));
        let rng_y = rng.random(Some(min + rng_x.abs()), Some(max - rng_x.abs()));
        let rng_z = rng.random(
            Some(min + rng_x.abs() + rng_y.abs()),
            Some(max - rng_x.abs() - rng_y.abs()),
        );
        Self {
            x_r: rng_x,
            y_g: rng_y,
            z_b: rng_z,
        }
    }

    pub fn random_unit_vector(rng: &mut Random<f64>) -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere(rng))
    }

    pub fn random_in_hemisphere(rng: &mut Random<f64>, normal: Vec3) -> Vec3 {
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere(rng);
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk(rng: &mut Random<f64>) -> Vec3 {
        let min = -1.0;
        let max = 1.0;
        let rng_x = rng.random(Some(min), Some(max));
        let rng_y = rng.random(Some(min + rng_x.abs()), Some(max - rng_x.abs()));
        Self {
            x_r: rng_x,
            y_g: rng_y,
            z_b: 0.,
        }
    }
    #[inline(always)]
    pub fn near_zero(vec: Self) -> bool {
        vec.x_r.abs() < f64::MIN_POSITIVE
            && vec.y_g.abs() < f64::MIN_POSITIVE
            && vec.z_b.abs() < f64::MIN_POSITIVE
    }
    #[inline(always)]
    pub fn reflect(v: Self, n: Self) -> Vec3 {
        v - n * Vec3::dot(v, n) * 2.
    }

    pub fn reflact(uv: Self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = Vec3::dot(-uv, n).min(1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(1.0 - Vec3::length_squared(r_out_perp)).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl From<Rgb<u8>> for Vec3 {
    fn from(rgb: Rgb<u8>) -> Self {
        Self {
            x_r: f64::from(rgb.0[0]),
            y_g: f64::from(rgb.0[1]),
            z_b: f64::from(rgb.0[2]),
        }
    }
}

impl From<Vec3> for Rgb<u8> {
    fn from(vec: Vec3) -> Self {
        let mut r = vec.x_r;
        let mut g = vec.y_g;
        let mut b = vec.z_b;

        let scale = 1.0 / crate::SAMPLES_PER_PIXEL as f64;
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        let ir: u8 = (r.clamp(0.0, 1.0) * 256.) as u8;
        let ig: u8 = (g.clamp(0.0, 1.0) * 256.) as u8;
        let ib: u8 = (b.clamp(0.0, 1.0) * 256.) as u8;
        let arr: [u8; 3] = [ir, ig, ib];
        let rgb = Rgb::from_slice(&arr);
        *rgb
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x_r: self.x_r + rhs.x_r,
            y_g: self.y_g + rhs.y_g,
            z_b: self.z_b + rhs.z_b,
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x_r += rhs;
        self.y_g += rhs;
        self.z_b += rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x_r *= rhs;
        self.y_g *= rhs;
        self.z_b *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x_r: self.x_r / rhs,
            y_g: self.y_g / rhs,
            z_b: self.z_b / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x_r /= rhs;
        self.y_g /= rhs;
        self.z_b /= rhs;
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x_r: self.x_r * rhs.x_r,
            y_g: self.y_g * rhs.y_g,
            z_b: self.z_b * rhs.z_b,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x_r: self.x_r * rhs,
            y_g: self.y_g * rhs,
            z_b: self.z_b * rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x_r: self.x_r - rhs.x_r,
            y_g: self.y_g - rhs.y_g,
            z_b: self.z_b - rhs.z_b,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x_r += rhs.x_r;
        self.y_g += rhs.y_g;
        self.z_b += rhs.z_b;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x_r: -self.x_r,
            y_g: -self.y_g,
            z_b: -self.z_b,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0} {1} {2}", self.x_r, self.y_g, self.z_b))
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x_r,
            1 => &self.y_g,
            2 => &self.z_b,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x_r,
            1 => &mut self.y_g,
            2 => &mut self.z_b,
            _ => panic!("Index out of bounds"),
        }
    }
}
