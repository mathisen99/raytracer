use crate::logic::Material;
use crate::logic::Ray;
use crate::logic::Vec3;
use crate::logic::Intersectable;

#[derive(Debug)]
pub struct Plane {
    pub position: Vec3,   // The position of the plane
    pub normal: Vec3,     // The normal vector to the plane
    pub material: Material, // The material of the plane
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        // Calculate the dot product between the plane's normal and the ray's direction
        let denom = self.normal.dot(ray.direction);

        if denom.abs() > crate::EPSILON {
            // Calculate the vector from the ray's origin to the plane's position
            let v = self.position - ray.origin;

            // Calculate the distance from the ray's origin to the point of intersection
            let distance = v.dot(self.normal) / denom;

            if distance >= 0.0 {
                Some(distance) // Return the distance if it's greater than or equal to zero
            } else {
                None // No intersection if the distance is negative
            }
        } else {
            None // No intersection if the denominator is close to zero (parallel or nearly parallel)
        }
    }

    fn material(&self) -> Material {
        self.material // Return the material of the plane
    }

    fn normal(&self, _point: Vec3) -> Vec3 {
        -self.normal // Return the negated normal vector
    }
}
