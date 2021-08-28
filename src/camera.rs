use crate::{degrees_to_radians, ray::Ray, vec3::Vec3};

pub struct Camera {
    pub vfov: f64,
    pub aspect_ratio: f64,

    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, vfov: f64, lookfrom: Vec3, lookat: Vec3, vup: Vec3) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        //let focal_length = 1.0;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = lookfrom - (horizontal / 2.) - (vertical / 2.) - w;

        Self {
            vfov,
            aspect_ratio,
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(camera: &Camera, s: f64, t: f64) -> Ray {
        Ray::new(
            camera.origin,
            camera.lower_left_corner + (camera.horizontal * s) + (camera.vertical * t)
                - camera.origin,
        )
    }
}
impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            16.0 / 9.0,
            20.,
            Vec3::new(-2.0, 2.0, 1.0),
            Vec3::new(0., 0., -1.0),
            Vec3::new(0., 1., 0.),
        )
    }
}
