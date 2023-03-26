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
        let ranvec = (0..256)
            .map(|_| Vec3::random(random, Some(-1.0), Some(1.0)))
            .collect();
        let mut rand = rand::thread_rng();
        let mut rng = Random::new(&mut rand, Uniform::new(0usize, 256));
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
        let mut p = (0..256).collect::<Vec<usize>>();
        Self::permute(rng, &mut p);
        p
    }
    fn permute(rng: &mut Random<usize>, array: &mut [usize]) {
        ((array.len() - 1)..0).for_each(|i| array.swap(i, rng.random(Some(0), Some(i))))
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x_r - p.x_r.floor();
        let v = p.y_g - p.y_g.floor();
        let w = p.z_b - p.z_b.floor();

        let i = p.x_r.floor() as i64;
        let j = p.y_g.floor() as i64;
        let k = p.z_b.floor() as i64;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for (di, elem) in c.iter_mut().enumerate() {
            for (dj, elem2) in elem.iter_mut().enumerate() {
                for (dk, elem3) in elem2.iter_mut().enumerate() {
                    let perm_x_index = ((i + di as i64) & 255) as usize;
                    let perm_y_index = ((j + dj as i64) & 255) as usize;
                    let perm_z_index = ((k + dk as i64) & 255) as usize;
                    *elem3 = self.ran_vec[self.perm_x[perm_x_index]
                        ^ self.perm_y[perm_y_index]
                        ^ self.perm_z[perm_z_index]];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u.powi(2) * (3.0 - 2.0 * u);
        let vv = v.powi(2) * (3.0 - 2.0 * v);
        let ww = w.powi(2) * (3.0 - 2.0 * w);
        c.iter()
            .enumerate()
            .map(|(i, elem)| {
                elem.iter()
                    .enumerate()
                    .map(|(j, elem2)| {
                        elem2
                            .iter()
                            .enumerate()
                            .map(|(k, elem3)| {
                                let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                                (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                                    * Vec3::dot(*elem3, weight_v)
                            })
                            .fold(0., |a, b| a + b)
                    })
                    .fold(0., |a, b| a + b)
            })
            .fold(0., |a, b| a + b)
    }
    pub fn turb(&self, p: &Vec3, depth: i32) -> f64 {
        let mut temp_p = *p;
        let mut weight = 1.0;

        (0..depth)
            .map(|_| {
                let output = weight * self.noise(&temp_p);
                weight *= 0.5;
                temp_p *= 2.0;
                output
            })
            .fold(0., |a, b| a + b)
            .abs()
    }
}
