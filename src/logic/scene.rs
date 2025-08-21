use crate::logic::Camera;
use crate::logic::Color;
use crate::logic::Intersectable;
use crate::logic::Light;
use crate::logic::Options;
use crate::logic::Ray;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,

    pub camera: Camera,
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
    pub options: Options,
}

impl Scene {
    // Render the scene and save the result to an image file.
    pub fn render(&self, filename: String) {
        // Create an image buffer with the specified dimensions.
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        // Calculate gamma correction factor reciprocally.
        let gamma_correction = self.options.gamma.recip();

        // Convert width and height to floating-point numbers.
        let w = f64::from(self.width);
        let h = f64::from(self.height);

        // Create a ppm file to save the rendering result.
        let name = filename.clone().split_once(".").unwrap().0.to_owned() + ".ppm";
        let mut file = File::create(name).unwrap();
        file.write_all("P3\n".as_bytes()).unwrap();
        let scale = format!("{} {}\n", self.width, self.height);
        file.write_all(scale.as_bytes()).unwrap();
        file.write_all(b"255\n").unwrap();

        // Iterate over the coordinates and pixels of the image.
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // Calculate normalized coordinates within the image.
            let u = f64::from(x) / w;
            let v = f64::from(y) / h;

            // Generate a ray from the camera at the given coordinates.
            let ray = self.camera.get_ray(u, v);

            // Cast a ray into the scene and compute the resulting color.
            let color = Ray::cast_ray(ray, &self.objects, &self.lights, &self.options, 0)
                .unwrap_or(self.bg_color);

            // Apply gamma correction and set the pixel color.
            *pixel = color.gamma_rgb(gamma_correction);

            // Add the color data to the ppm file.
            let colors = color.gamma_rgb(gamma_correction);
            file.write_all(format!("{} {} {}\n", colors[0], colors[1], colors[2]).as_bytes())
                .unwrap();
            file.write_all(b"\n").unwrap();
        }

        // Save the rendered image to the specified file.
        imgbuf.save(filename).unwrap();
    }
}
