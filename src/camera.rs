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
    pub fn new(aspect_ratio: f64, vfov: f64, origin: Vec3) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h;
        let focal_length = 1.0;

        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - (horizontal / 2.) - (vertical / 2.) - Vec3::new(0., 0., focal_length);

        Self {
            vfov,
            aspect_ratio,

            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(camera: &Camera, u: f64, v: f64) -> Ray {
        Ray::new(
            camera.origin,
            camera.lower_left_corner + (camera.horizontal * u) + (camera.vertical * v)
                - camera.origin,
        )
    }
}
impl Default for Camera {
    fn default() -> Self {
        Camera::new(16.0 / 9.0, 90., Vec3::new(0.0, 0.0, 0.0))
    }
}
