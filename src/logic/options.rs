// Define a struct to represent rendering options.
#[derive(Debug)]
pub struct Options {
    pub max_rays: u8,      // Maximum number of rays for rendering
    pub gamma: f64,        // Gamma correction factor for final color
    pub diffuse: bool,     // Enable/disable diffuse reflection
    pub specular: bool,    // Enable/disable specular reflection
    pub shadows: bool,     // Enable/disable shadows
    pub reflections: bool, // Enable/disable reflections
}
