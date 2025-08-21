use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64, // Red component
    g: f64, // Green component
    b: f64, // Blue component
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0., 0., 0.)
    }

    pub fn white() -> Color {
        Color::new(1., 1., 1.)
    }

    pub fn purple() -> Color {
        Color::new(0.5, 0., 0.5)
    }

    pub fn red() -> Color {
        Color::new(1., 0., 0.)
    }

    pub fn green() -> Color {
        Color::new(0., 1., 0.)
    }

    pub fn blue() -> Color {
        Color::new(0.1, 0.1, 1.)
    }

    pub fn yellow() -> Color {
        Color::new(1., 1., 0.)
    }

    pub fn cyan() -> Color {
        Color::new(0., 1., 1.)
    }

    pub fn pink() -> Color {
        Color::new(1., 0., 1.)
    }

    pub fn orange() -> Color {
        Color::new(1., 0.5, 0.)
    }

    pub fn gray() -> Color {
        Color::new(0.5, 0.5, 0.5)
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Color {
        // Convert RGB values from u8 to the range [0.0, 1.0]
        Color {
            r: f64::from(r) / 255.0,
            g: f64::from(g) / 255.0,
            b: f64::from(b) / 255.0,
        }
    }

    pub fn gamma_rgb(&self, gamma_correction: f64) -> image::Rgb<u8> {
        // Apply gamma correction and convert the color to RGB format
        image::Rgb([
            (self.r.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
            (self.g.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
            (self.b.min(1.0).max(0.0).powf(gamma_correction) * 255.0) as u8,
        ])
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Color) -> Color {
        // Addition of two colors: Add corresponding components (r, g, b)
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        // In-place addition: Add and update the color components
        *self = Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, factor: f64) -> Color {
        // Scalar multiplication of a color: Multiply each component by the factor
        Color {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Color {
        // Component-wise multiplication of two colors
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
