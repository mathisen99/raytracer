// Import necessary modules and types.
use crate::logic::Color;
use crate::logic::Intersectable;
use crate::logic::Light;
use crate::logic::Material;
use crate::logic::Options;
use crate::logic::Vec3;

// Define a struct representing a ray with an origin and direction.
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,       // Origin point of the ray
    pub direction: Vec3,    // Direction in which the ray travels
}

// Define a struct representing information about an intersection.
#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub distance: f64,      // Distance to the intersection point
    pub hit_point: Vec3,    // Coordinates of the intersection point
    pub normal: Vec3,       // Normal vector at the intersection point
    pub material: Material, // Material properties of the intersected object
}

impl Ray {
    // Calculate the intersection of a ray with a list of objects and return information
    // about the closest intersection, if any.
    pub fn intersect(ray: Ray, objects: &[Box<dyn Intersectable>]) -> Option<Intersection> {
        let mut distance = f64::INFINITY; // Initialize with positive infinity
        let mut material = Material::neutral(); // Neutral material properties
        let mut normal = Vec3::zero(); // Zero vector for normal
        let mut hit_point = Vec3::zero(); // Zero vector for hit point

        // Loop through the objects to find the closest intersection.
        for shape in objects {
            if let Some(dist) = shape.intersect(ray) {
                if dist < distance {
                    // Update the closest intersection information.
                    distance = dist;
                    material = shape.material();
                    hit_point = ray.origin + (ray.direction * distance);
                    normal = shape.normal(hit_point);
                }
            }
        }

        if distance < f64::INFINITY {
            Some(Intersection {
                distance,
                hit_point,
                normal,
                material,
            })
        } else {
            None // No intersection found
        }
    }

    // Cast a ray into the scene, compute shading, and handle reflections.
    pub fn cast_ray(
        ray: Ray,
        objects: &[Box<dyn Intersectable>],
        lights: &[Light],
        options: &Options,
        depth: u8,
    ) -> Option<Color> {
        if depth >= options.max_rays {
            return None; // Maximum recursion depth reached
        }

        let intersection = Ray::intersect(ray, objects)?;

        let mut shaded_color = Light::shade(objects, lights, options, intersection, ray.direction);

        if intersection.material.reflectiveness > 0.0 && options.reflections {
            let reflection = ray.direction.reflect(intersection.normal).normalize();

            let reflected_ray = Ray {
                origin: intersection.hit_point.correct(intersection.normal),
                direction: reflection,
            };

            if let Some(reflected_color) =
                Ray::cast_ray(reflected_ray, objects, lights, options, depth + 1)
            {
                shaded_color += reflected_color * intersection.material.reflectiveness;
            }
        }

        Some(shaded_color)
    }
}