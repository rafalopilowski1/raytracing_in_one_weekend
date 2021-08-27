use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use rand::Rng;

use crate::random_float;

#[derive(PartialEq, PartialOrd, Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn length(v: Vec3) -> f64 {
        f64::sqrt(Vec3::length_squared(v))
    }
    pub fn length_squared(v: Vec3) -> f64 {
        v.x.powi(2) + v.y.powi(2) + v.z.powi(2)
    }
    pub fn cross(v: Vec3, rhs: Self) -> Self {
        Self {
            x: v.y * rhs.z - v.z * rhs.y,
            y: v.z * rhs.x - v.x * rhs.z,
            z: v.x * rhs.y - v.y * rhs.x,
        }
    }
    pub fn dot(v: Vec3, rhs: Self) -> f64 {
        v.x * rhs.x + v.y * rhs.y + v.z * rhs.z
    }
    pub fn unit_vector(v: Vec3) -> Self {
        v / Vec3::length(v)
    }
    pub fn random<R: Rng + ?Sized>(rng: &mut R, min: Option<f64>, max: Option<f64>) -> Vec3 {
        let rng_x = random_float(rng, min, max);
        let rng_y = random_float(rng, min, max);
        let rng_z = random_float(rng, min, max);
        Self {
            x: rng_x,
            y: rng_y,
            z: rng_z,
        }
    }
    pub fn random_in_unit_sphere<R: Rng + ?Sized>(rng: &mut R) -> Self {
        loop {
            let p: Vec3 = Vec3::random(rng, Some(-1.0), Some(1.0));
            if Vec3::length_squared(p) >= 1. {
                continue;
            } else {
                return p;
            }
        }
    }
    pub fn random_unit_vector<R: Rng + ?Sized>(rng: &mut R) -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere(rng))
    }
    pub fn random_in_hemisphere<R: Rng + ?Sized>(rng: &mut R, normal: Vec3) -> Vec3 {
        let in_unit_sphere: Vec3 = Vec3::random_in_unit_sphere(rng);
        if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn near_zero(vec: Vec3) -> bool {
        let s = f64::MIN_POSITIVE;
        f64::abs(vec.x) < s && f64::abs(vec.y) < s && f64::abs(vec.z) < s
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * Vec3::dot(v, n) * 2.
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{0} {1} {2}", self.x, self.y, self.z))
    }
}
