// Disable dead code warnings for the entire module.
#![allow(dead_code)]

mod camera;
mod light;
mod ray;
mod scene;
mod vector;
mod colors;
mod materials;
mod options;
mod intersectable;
mod helpers;

pub use camera::Camera;
pub use light::Light;
pub use light::LightType;
pub use light::create_spotlight;
pub use ray::Ray;
pub use scene::Scene;
pub use vector::Vec3;
pub use colors::Color;
pub use materials::Material;
pub use options::Options;
pub use intersectable::Intersectable;
pub use helpers::progress_bar;
