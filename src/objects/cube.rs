use crate::logic::Material;
use crate::logic::Ray;
use crate::logic::Vec3;
use crate::logic::Intersectable;

#[derive(Debug)]
pub struct Cube {
    pub position: Vec3,
    pub dimensions: f64,
    pub material: Material,
    pub normal: Vec3,
}

impl Intersectable for Cube {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        // Calculate the half-size of the cube
        let half_size = self.dimensions / 2.0;

        // Calculate the minimum and maximum bounds of the cube
        let min_bound = self.position - Vec3::splat(half_size);
        let max_bound = self.position + Vec3::splat(half_size);

        // Calculate the t-values for the intersections with the X, Y, and Z faces
        let tx1 = (min_bound.x - ray.origin.x) / ray.direction.x;
        let tx2 = (max_bound.x - ray.origin.x) / ray.direction.x;
        let ty1 = (min_bound.y - ray.origin.y) / ray.direction.y;
        let ty2 = (max_bound.y - ray.origin.y) / ray.direction.y;
        let tz1 = (min_bound.z - ray.origin.z) / ray.direction.z;
        let tz2 = (max_bound.z - ray.origin.z) / ray.direction.z;

        // Find the minimum and maximum t-values for the intersections
        let t_min = tx1.min(tx2).max(ty1.min(ty2)).max(tz1.min(tz2));
        let t_max = tx1.max(tx2).min(ty1.max(ty2)).min(tz1.max(tz2));

        // Check if there's a valid intersection
        if t_max >= 0.0 && t_min <= t_max {
            // Intersection exists, return the minimum t-value
            Some(t_min)
        } else {
            // No intersection
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, point: Vec3) -> Vec3 {
        let differences = (point - self.position).abs();
        
        if differences.x >= differences.y.max(differences.z) {
            return Vec3::new((point.x - self.position.x).signum(), 0.0, 0.0);
        } else if differences.y >= differences.z {
            return Vec3::new(0.0, (point.y - self.position.y).signum(), 0.0);
        } else {
            return Vec3::new(0.0, 0.0, (point.z - self.position.z).signum());
        }
    }    
}