mod logic;
mod objects;
pub use crate::logic::create_spotlight;
// Define a constant 'EPSILON' with a value of 1e-6, used for numeric tolerance.
const EPSILON: f64 = 1e-6;

use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn main() {
    //run external python script
    std::process::Command::new("pip3")
    .arg("install")
    .arg("-r")
    .arg("requirements.txt")
    .output()
    .expect("failed to execute process");

    std::process::Command::new("python3")
        .arg("./ai.py")
        .output()
        .expect("failed to execute process");

    let options = logic::Options {
        max_rays: 4,
        gamma: 1.0,
        diffuse: true,
        specular: true,
        shadows: true,
        reflections: true,
    };

    let width = 1920;
    let height = 1080;
    let aspect_ratio = width as f64 / height as f64;

    let scene = logic::Scene {
        width,
        height,
        camera: logic::Camera::new(
            logic::Vec3::new(0., 0., 0.), // Positioning the camera further back and centered
            logic::Vec3::new(10., 4., -20.), // Making the camera look at a point centered among the objects
            aspect_ratio,
            0.,
        ),
        objects: vec![
            Box::new(objects::Sphere {
                position: logic::Vec3::new(11.0, 4.0, -15.0),
                radius: 4.,
                material: logic::Material::plastic(logic::Color::orange()),
            }),
            Box::new(objects::Sphere {
                position: logic::Vec3::new(6.0, 0.0, -11.0),
                radius: 1.5,
                material: logic::Material::plastic(logic::Color::cyan()),
            }),
            Box::new(objects::Sphere {
                position: logic::Vec3::new(2.0, -1.0, -23.0),
                radius: 0.5,
                material: logic::Material::pearl(),
            }),
            Box::new(objects::Sphere {
                position: logic::Vec3::new(3.0, -1.5, -21.0),
                radius: 0.5,
                material: logic::Material::pearl(),
            }),
            Box::new(objects::Cylinder {
                center: logic::Vec3::new(2.0, -4.0, -19.0),
                radius: 4.,
                height: 2.,
                material: logic::Material::mirror(),
            }),
            Box::new(objects::Sphere {
                position: logic::Vec3::new(5.0, 4.0, -15.0),
                radius: 3.,
                material: logic::Material::mirror(),
            }),
            //walls
            Box::new(objects::Sphere {
                position: logic::Vec3::new(0.0, 0.0, 0.0),
                radius: 50.,
                material: logic::Material::matt(logic::Color::new(0.2, 0.2, 0.2)),
            }),
            Box::new(objects::Plane {
                position: logic::Vec3::new(0.0, -8.0, 0.0),
                normal: logic::Vec3::new(0.0, -1.0, 0.0),
                material: logic::Material::matt(logic::Color::from_u8(0x66, 0x33, 0x66)),
            }),
            Box::new(objects::Plane {
                position: logic::Vec3::new(0.0, 21.0, 0.0),
                normal: logic::Vec3::new(0.0, 1.0, 0.0),
                material: logic::Material::matt(logic::Color::from_u8(0x66, 0x33, 0x66)),
            }),
        ],
        lights: vec![
            logic::Light {
                light_type: logic::LightType::Ambient,
                position: logic::Vec3::zero(),
                intensity: 1.,
                color: logic::Color::white(),
            },
            logic::create_spotlight(
                logic::Vec3::new(-40.0, 20.0, 20.0), // Positioning the spotlight
                logic::Vec3::new(10., 4., -20.), // Making the spotlight look at a point centered among the objects
                1.,
                logic::Color::white(),
                10.,
            ),
            logic::create_spotlight(
                logic::Vec3::new(20.0, -5.0, 0.0),
                logic::Vec3::new(10.0, 4.0, -20.0),
                1.,
                logic::Color::purple(),
                40.,
            ),
            logic::create_spotlight(
                logic::Vec3::new(25.0, 6.0, 10.0),
                logic::Vec3::new(10.0, 4.0, -19.0),
                1.,
                logic::Color::yellow(),
                40.,
            ),
            logic::create_spotlight(
                logic::Vec3::new(-20.0, -2.0, 10.0),
                logic::Vec3::new(2.0, -4.0, -19.0),
                1.,
                logic::Color::yellow(),
                40.,
            ),
        ],
        bg_color: logic::Color::new(0.4, 0.4, 0.4),
        options,
    };

    let now = Instant::now();

    // Create a channel to communicate between threads
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread to run the progress_bar function
    thread::spawn(move || {
        logic::progress_bar(rx);
    });
    
    create_audit_pictures();
    scene.render("result.png".to_string());

    let duration = now.elapsed();
    tx.send(duration).unwrap(); // Send the final duration to the progress bar

    println!(

        "\nIt took {} milliseconds to render the image!.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
}

fn create_audit_pictures() {
    let options = logic::Options {
        max_rays: 4,
        gamma: 1.0,
        diffuse: true,
        specular: true,
        shadows: true,
        reflections: true,
    };

    let width = 1920;
    let height = 1080;
    let aspect_ratio = width as f64 / height as f64;

    let mut scene = logic::Scene {
        width,
        height,
        camera: logic::Camera::new(
            logic::Vec3::new(0., 0., 5.0), // Positioning the camera further back and centered
            logic::Vec3::new(10., 4., -20.), // Making the camera look at a point centered among the objects
            aspect_ratio,
            0.,
        ),
        objects: vec![Box::new(objects::Sphere {
            position: logic::Vec3::new(10.0, 4.0, -20.0),
            radius: 5.,
            material: logic::Material::plastic(logic::Color::orange()),
        })],
        lights: vec![
            logic::Light {
                light_type: logic::LightType::Ambient,
                position: logic::Vec3::zero(),
                intensity: 1.,
                color: logic::Color::white(),
            },
            logic::create_spotlight(
                logic::Vec3::new(10.0, 20.0, -20.0),
                logic::Vec3::new(10., 4., -20.),
                1.,
                logic::Color::white(),
                40.,
            ),
            logic::create_spotlight(
                logic::Vec3::new(-20.0, 15.0, 20.0),
                logic::Vec3::new(10.0, 4.0, -20.0),
                1.4,
                logic::Color::purple(),
                40.,
            ),
            logic::create_spotlight(
                logic::Vec3::new(20.0, -2.0, 10.0),
                logic::Vec3::new(10.0, 4.0, -20.0),
                1.,
                logic::Color::yellow(),
                40.,
            ),
        ],
        bg_color: logic::Color::black(),
        options,
    };
    scene.render("picture1.png".to_string());
    scene.objects = vec![
        Box::new(objects::Cube {
            position: logic::Vec3::new(10.0, 4.0, -20.0),
            dimensions: 6.,
            material: logic::Material::matt(logic::Color::cyan()),
            normal: logic::Vec3::new(0.0, 0.0, 0.0),
        }),
        Box::new(objects::Plane {
            position: logic::Vec3::new(0.0, -8.0, 0.0),
            normal: logic::Vec3::new(0.0, -1.0, 0.0),
            material: logic::Material::matt(logic::Color::from_u8(0x66, 0x33, 0x66)),
        }),
    ];
    scene.lights[0].intensity = 0.3;
    scene.render("picture2.png".to_string());
    scene.objects = vec![
        Box::new(objects::Cube {
            position: logic::Vec3::new(20.0, -2.5, -20.0),
            dimensions: 6.,
            material: logic::Material::matt(logic::Color::blue()),
            normal: logic::Vec3::new(0.0, 0.0, 0.0),
        }),
        Box::new(objects::Sphere {
            position: logic::Vec3::new(11.0, 4.0, -15.0),
            radius: 4.,
            material: logic::Material::plastic(logic::Color::orange()),
        }),
        Box::new(objects::Sphere {
            position: logic::Vec3::new(6.0, 0.0, -11.0),
            radius: 1.5,
            material: logic::Material::plastic(logic::Color::cyan()),
        }),
        Box::new(objects::Sphere {
            position: logic::Vec3::new(2.0, -1.0, -23.0),
            radius: 0.5,
            material: logic::Material::pearl(),
        }),
        Box::new(objects::Sphere {
            position: logic::Vec3::new(3.0, -1.5, -21.0),
            radius: 0.5,
            material: logic::Material::pearl(),
        }),
        Box::new(objects::Cylinder {
            center: logic::Vec3::new(2.0, -4.0, -19.0),
            radius: 4.,
            height: 2.,
            material: logic::Material::mirror(),
        }),
        Box::new(objects::Cylinder {
            center: logic::Vec3::new(8.0, -2.0, -7.0),
            radius: 1.5,
            height: 4.,
            material: logic::Material::plastic(logic::Color::green()),
        }),
        Box::new(objects::Sphere {
            position: logic::Vec3::new(5.0, 4.0, -15.0),
            radius: 3.,
            material: logic::Material::mirror(),
        }),
        //walls
        Box::new(objects::Sphere {
            position: logic::Vec3::new(0.0, 0.0, 0.0),
            radius: 50.,
            material: logic::Material::matt(logic::Color::new(0.2, 0.2, 0.2)),
        }),
        Box::new(objects::Plane {
            position: logic::Vec3::new(0.0, -8.0, 0.0),
            normal: logic::Vec3::new(0.0, -1.0, 0.0),
            material: logic::Material::matt(logic::Color::from_u8(0x66, 0x33, 0x66)),
        }),
        Box::new(objects::Plane {
            position: logic::Vec3::new(0.0, 21.0, 0.0),
            normal: logic::Vec3::new(0.0, 1.0, 0.0),
            material: logic::Material::matt(logic::Color::from_u8(0x66, 0x33, 0x66)),
        }),
    ];
    scene.lights[0].intensity = 1.;
    scene.render("picture3.png".to_string());
    scene.camera = logic::Camera::new(
        logic::Vec3::new(15., 3., -35.0),
        logic::Vec3::new(10., 4., -20.),
        aspect_ratio,
        0.,
    );
    scene.render("picture4.png".to_string());
}
