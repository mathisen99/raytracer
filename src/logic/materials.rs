// Import the 'Color' type from the 'super::color' module.
use crate::logic::Color;

// Define a struct representing material properties.
#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,           // The color of the material
    pub diffuse: f64,           // Diffuse reflectivity (0.0 to 1.0)
    pub specular: f64,          // Specular reflectivity (0.0 to 1.0)
    pub specular_exponent: f64, // Specular exponent for shininess
    pub reflectiveness: f64,    // Reflectiveness (0.0 to 1.0)
}

impl Material {
    // Create a new Material with neutral properties (all zeros).
    pub fn neutral() -> Material {
        Material {
            color: Color::black(),  // Black color
            diffuse: 0.0,           // No diffuse reflection
            specular: 0.0,          // No specular reflection
            specular_exponent: 0.0, // Specular exponent is zero
            reflectiveness: 0.0,    // No reflectiveness
        }
    }
    // create velvet material
    pub fn matt(color: Color) -> Material {
        Material {
            color,
            diffuse: 0.8,
            specular: 0.2,
            specular_exponent: 5.0,
            reflectiveness: 0.0,
        }
    }
    // create mirror material
    pub fn mirror() -> Material {
        Material {
            color: Color::white(),
            diffuse: 0.0,
            specular: 10.0,
            specular_exponent: 100.0,
            reflectiveness: 1.0,
        }
    }
    // create plastic material
    pub fn plastic(color: Color) -> Material {
        Material {
            color,
            diffuse: 0.5,
            specular: 30.0,
            specular_exponent: 50.0,
            reflectiveness: 0.4,
        }
    }

    pub fn pearl() -> Material {
        Material {
            color: Color::white(),
            diffuse: 0.5,
            specular: 80.0,
            specular_exponent: 80.0,
            reflectiveness: 0.3,
        }
    }
}
