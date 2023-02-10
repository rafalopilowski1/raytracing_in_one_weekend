use crate::{degrees_to_radians, ray::Ray, vec3::Vec3, Random};
#[derive(Clone, Copy)]
pub struct Camera {
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,

    pub aspect_ratio: f64,

    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,

    pub time0: f64,
    pub time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focus_dist = Vec3::length(lookfrom - lookat);

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = lookfrom - (horizontal / 2.) - (vertical / 2.) - w * focus_dist;

        let lens_radius = aperture / 2.;
        Self {
            lens_radius,
            u,
            v,
            w,
            aspect_ratio,
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            time0,
            time1,
        }
    }

    pub fn get_ray(rng: &mut Random, camera: Camera, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk(rng) * camera.lens_radius;
        let offset = camera.u * rd.x_r + camera.v * rd.y_g;
        Ray::new(
            camera.origin + offset,
            camera.lower_left_corner + (camera.horizontal * s) + (camera.vertical * t)
                - camera.origin
                - offset,
            rng.random_float(Some(camera.time0), Some(camera.time1)),
        )
    }
}
impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            Vec3::new(13., 2., 3.),
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            20.,
            16.0 / 9.0,
            0.1,
            0.,
            1.,
        )
    }
}
