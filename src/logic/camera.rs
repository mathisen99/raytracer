use super::ray::Ray;
use super::vector::Vec3;

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,     // The camera's origin (position)
    corner: Vec3,     // The corner point of the view plane
    horizontal: Vec3, // The horizontal vector of the view plane
    vertical: Vec3,   // The vertical vector of the view plane
    u: Vec3,           // The u vector (used for camera rotation)
    v: Vec3,           // The v vector (used for camera rotation)
    w: Vec3,           // The w vector (used for camera rotation)
}

impl Camera {
    pub fn new(origin: Vec3, look_at: Vec3, aspect_ratio: f64, roll: f64) -> Camera {
        let fov: f64 = 60.;
        let roll_angle = roll.to_radians();
        let rotated_up = Vec3::new(-roll_angle.sin(), roll_angle.cos(), 0.0);

        let w = (origin - look_at).normalize(); // Calculate the w vector (forward)
        let u = rotated_up.cross(w).normalize(); // Calculate the u vector (right)
        let v = w.cross(u).normalize();          // Calculate the v vector (up)

        let half_height = (fov.to_radians() / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let corner = origin - (u * half_width) + (v * half_height) - w; // Calculate the corner of the view plane
        let horizontal = u * (2.0 * half_width);                       // Calculate the horizontal vector
        let vertical = -v * (2.0 * half_height);                       // Calculate the vertical vector

        Camera {
            origin,
            corner,
            horizontal,
            vertical,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction = self.corner + (self.horizontal * s) + (self.vertical * t) - self.origin;

        Ray {
            origin: self.origin,
            direction: direction.normalize(),
        }
    }
}

