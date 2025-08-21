use crate::logic::Intersectable;
use crate::logic::Material;
use crate::logic::Ray;
use crate::logic::Vec3;

#[derive(Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        // oc represents the vector from the spheres center to the light source origin
        let oc = ray.origin - self.position;
        // a represents the length of the ray squared. Is 1 when ray.direction is normalized
        let a = ray.direction.dot(ray.direction);
        // b represents the rays direction in relation to oc
        let b = oc.dot(ray.direction);
        // c represents the radius of the sphere in relation to the line oc
        let c = oc.dot(oc) - self.radius * self.radius;
        // the discriminant is the result of the mathematical operations under the square root in a quadratic equation
        let discriminant = b*b - a * c;
        let mut t: f64;

        // if the discriminant is positive, evaluate the solution for the smaller root first
        if discriminant > 0.0 {
            t = (-b - discriminant.sqrt()) / a;

            if t < f64::INFINITY && t > crate::EPSILON {
                // if the solution to the equation falls within our decided span, return the result temp
                // temp is the distance of the ray to the intersection point of the sphere
                return Some(t);
            }
        }

        // if the smaller root falls outside EPSILON and INFINITY, try the bigger root
        t = (-b + discriminant.sqrt()) / a;

        if t < f64::INFINITY && t > crate::EPSILON {
            // if the solution to the equation falls within our decided span, return the result temp
            // temp is the distance of the ray to the intersection point of the sphere
            return Some(t);
        }

        // if no solution is acceptable return None. Ray doesn't hit the sphere
        None
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalize()
    }
}
