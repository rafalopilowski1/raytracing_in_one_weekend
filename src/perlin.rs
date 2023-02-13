use rand::distributions::Uniform;

use crate::{random::Random, vec3::Vec3};

pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Perlin {
    pub fn new(random: &mut Random<f64>) -> Self {
        let mut ranvec = Vec::new();
        for _ in 0..256 {
            ranvec.push(Vec3::random(random, Some(-1.0), Some(1.0)));
        }
        let mut rng = Random::new(rand::thread_rng(), Uniform::new(0usize, 256));
        let perm_x = Perlin::perlin_generate_perm(&mut rng);
        let perm_y = Perlin::perlin_generate_perm(&mut rng);
        let perm_z = Perlin::perlin_generate_perm(&mut rng);
        Self {
            ran_vec: ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    fn perlin_generate_perm(rng: &mut Random<usize>) -> Vec<usize> {
        let mut p = Vec::new();
        for i in 0..256 {
            p.push(i);
        }
        Self::permute(rng, &mut p, 256);
        p
    }
    fn permute(rng: &mut Random<usize>, array: &mut [usize], n: usize) {
        for i in (n - 1)..0 {
            let target = rng.random(Some(0), Some(i));
            array.swap(i, target);
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x_r - p.x_r.floor();
        let v = p.y_g - p.y_g.floor();
        let w = p.z_b - p.z_b.floor();

        let i = p.x_r.floor() as i64;
        let j = p.y_g.floor() as i64;
        let k = p.z_b.floor() as i64;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ran_vec[self.perm_x
                        [(i + di) as usize & 255]
                        ^ self.perm_y[(j + dj) as usize & 255]
                        ^ self.perm_z[(k + dk) as usize & 255]];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, elem) in c.iter().enumerate() {
            for (j, elem2) in elem.iter().enumerate() {
                for (k, elem3) in elem2.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * Vec3::dot(*elem3, weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}
