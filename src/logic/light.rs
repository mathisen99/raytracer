use crate::logic::Color;
use crate::logic::Intersectable;
use crate::logic::Options;
use crate::logic::ray::{Intersection, Ray};
use crate::logic::Vec3;

// Define a struct for representing a light source.
#[derive(Debug)]
pub struct Light {
    pub light_type: LightType,
    pub position: Vec3,
    pub intensity: f64,
    pub color: Color,
}

// Define an enum to represent different types of lights.
#[derive(Debug)]
pub enum LightType {
    Ambient,
    Point,
    Spotlight { direction: Vec3, angle: f64 },
}

impl Light {
    // Function to calculate the shading of an object at an intersection point.
    pub fn shade(
        objects: &[Box<dyn Intersectable>],
        lights: &[Light],
        options: &Options,
        intersection: Intersection,
        direction: Vec3,
    ) -> Color {
        let mat = intersection.material;
        let mut diff_light = Color::black(); // Initialize diffuse light.
        let mut spec_light = Color::black(); // Initialize specular light.

        // Iterate through the lights.
        for light in lights {
            match &light.light_type {
                LightType::Ambient => {
                    // Ambient light contribution.
                    diff_light += light.color * light.intensity;
                }

                LightType::Point | LightType::Spotlight { .. } => {
                    let light_vec = light.position - intersection.hit_point;
                    let light_dir = light_vec.normalize();
                    let light_dis = light_vec.length();
                    let light_angle = light_dir.dot(intersection.normal);

                    // Create a ray from the intersection point to the light source.
                    let light_ray = Ray {
                        origin: if light_angle < 0.0 {
                            intersection.hit_point.correct(-intersection.normal)
                        } else {
                            intersection.hit_point.correct(intersection.normal)
                        },
                        direction: light_dir,
                    };

                    // Check if the intersection point is in shadow.
                    let hit_by_light = match Ray::intersect(light_ray, objects) {
                        None => true,
                        Some(intersect) => light_dis <= intersect.distance,
                    };

                    if hit_by_light || !options.shadows {
                        if let LightType::Spotlight { direction, angle } = &light.light_type {
                            let spotlight_dir = direction.normalize();
                            let cos_theta = light_dir.dot(spotlight_dir);

                            // Check if the light falls within the spotlight's angle.
                            if cos_theta < angle.cos() {
                                continue; // Skip this light source.
                            }
                        }

                        // Calculate the reflection of light.
                        let light_reflection = (-light_dir).reflect(intersection.normal);
                        let angle = -(light_reflection.dot(direction));

                        // Add to diffuse and specular light based on angles.
                        diff_light += light.color * (light.intensity * light_angle.max(0.0));
                        spec_light += light.color * angle.max(0.0).powf(mat.specular_exponent);
                    }
                }
            }
        }

        let mut factor = if !(options.diffuse | options.specular) {
            Color::white()
        } else {
            Color::black()
        };

        if options.diffuse {
            factor += diff_light * mat.diffuse; // Add diffuse reflection contribution.
        }

        if options.specular {
            factor += spec_light * mat.specular; // Add specular reflection contribution.
        }

        mat.color * factor // Return the final shaded color.
    }
}

// Function to create a spotlight.
pub fn create_spotlight(position: Vec3, direction: Vec3, intensity: f64, color: Color, angle: f64) -> Light {
    Light {
        light_type: LightType::Spotlight { 
            direction: (position - direction).normalize(), 
            angle: angle.to_radians() 
        },
        position: position,
        intensity: intensity,  
        color: color,
    }
}
