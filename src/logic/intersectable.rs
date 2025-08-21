use crate::logic::Material;
use crate::logic::Ray;
use crate::logic::Vec3;
use std::fmt::Debug;

// Define a trait named 'Intersectable' that represents objects which can be
// intersected by rays in a 3D environment.
pub trait Intersectable: Debug {
    // This method calculates the intersection of the object with a ray
    // and returns the distance to the intersection point (if any).
    fn intersect(&self, ray: Ray) -> Option<f64>;

    // This method returns the material properties of the object.
    fn material(&self) -> Material;

    // This method computes the normal vector at a given intersection point
    // on the object, which is useful for lighting and shading calculations.
    fn normal(&self, hit_point: Vec3) -> Vec3;
}
