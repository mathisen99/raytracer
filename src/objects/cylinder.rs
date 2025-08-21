use crate::logic::Intersectable;
use crate::logic::Material;
use crate::logic::Ray;
use crate::logic::Vec3;

// Define a struct for representing a cylinder.
#[derive(Debug)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
    pub material: Material,
}

// Implement the Intersectable trait for Cylinder.
impl Intersectable for Cylinder {
    // Function to check for ray-cylinder intersection.
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let b = 2.0 * (oc.x * ray.direction.x + oc.z * ray.direction.z);
        let c = oc.x * oc.x + oc.z * oc.z - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            let y1 = ray.origin.y + t1 * ray.direction.y;
            let y2 = ray.origin.y + t2 * ray.direction.y;
            let half_height = self.height / 2.0;

            // Initialize a variable to keep track of the closest intersection.
            let mut closest_t = f64::INFINITY;

            // Check for intersection with the sides of the cylinder.
            if t1 >= 0.0 && y1 >= self.center.y - half_height && y1 <= self.center.y + half_height {
                closest_t = t1;
            }
            if t2 >= 0.0 && y2 >= self.center.y - half_height && y2 <= self.center.y + half_height {
                if t2 < closest_t {
                    closest_t = t2;
                }
            }

            // Check for intersection with the top and bottom surfaces.
            let t3 = (self.center.y - half_height - ray.origin.y) / ray.direction.y;
            let t4 = (self.center.y + half_height - ray.origin.y) / ray.direction.y;

            if t3 >= 0.0 {
                let x = ray.origin.x + t3 * ray.direction.x - self.center.x;
                let z = ray.origin.z + t3 * ray.direction.z - self.center.z;
                if x * x + z * z <= self.radius * self.radius {
                    if t3 < closest_t {
                        closest_t = t3;
                    }
                }
            }

            if t4 >= 0.0 {
                let x = ray.origin.x + t4 * ray.direction.x - self.center.x;
                let z = ray.origin.z + t4 * ray.direction.z - self.center.z;
                if x * x + z * z <= self.radius * self.radius {
                    if t4 < closest_t {
                        closest_t = t4;
                    }
                }
            }

            if closest_t < f64::INFINITY {
                return Some(closest_t);
            }
        }

        None
    }

    // Function to get the material of the cylinder.
    fn material(&self) -> Material {
        self.material.clone()
    }

    // Function to get the normal vector at a given hit point on the cylinder.
    fn normal(&self, hit_point: Vec3) -> Vec3 {
        if (hit_point.y - self.center.y - self.height / 2.0).abs() < 1e-5
            || (hit_point.y - self.center.y + self.height / 2.0).abs() < 1e-5
        {
            return Vec3::new(0.0, (hit_point.y - self.center.y).signum(), 0.0);
        } else {
            let normal = Vec3::new(
                hit_point.x - self.center.x,
                0.0,
                hit_point.z - self.center.z,
            );
            return normal.normalize();
        }
    }
}

