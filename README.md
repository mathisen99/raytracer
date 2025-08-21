# Documentation for Ray Tracer

## Table of Contents

1. [Setting up the Scene](#setting-up-the-scene)
2. [Creating Objects](#creating-objects)
3. [Defining Materials](#defining-materials)
4. [Setting up Lights](#setting-up-lights)
5. [Configuring the Camera](#configuring-the-camera)
6. [Rendering Options](#rendering-options)
7. [Rendering the Scene](#rendering-the-scene)
8. [Examples to Creating Objects](#examples-to-creating-objects)

## Setting up the Scene
   - **Width & Height:** Define the resolution of the output image.
   - **Aspect Ratio:** Calculated based on width and height.
   - **Background Color:** Set the background color of the scene.

## Creating Objects
Objects can be created and added to the scene. Currently, spheres and planes are supported.
   - **Sphere:** Defined by a position, radius, and material.
   - **Plane:** Defined by a position, normal vector, and material.
   - **Cube:** Defined by a position, dimensions, and material.
   - **Cylinder:** Defined by a position, radius, and material.

## Defining Materials
Each object has a material defined by the following properties:
   - **Color:** RGB color of the material.
   - **Diffuse:** The amount of diffuseness. (0 for no diffuse, 1 for maximum diffuse)
   - **Specular:** The sharpness of the specular highlight.
   - **Specular Exponent:** Influences the spread of the specular highlight.
   - **Reflectiveness:** The amount of reflectiveness. (0 for no reflection, 1 for maximum reflection)
   
   You can also define materials in a simplyfied way:
   - Example: 
   ```rust
   material: logic::Material::plastic(logic::Color::cyan()),
   ```
   The materials that currently exists are:
   - mirror (Full reflection no colors)
   - plastic (plastic looking surface)
   - pearl (looks like a pearl lol...)
   - matt (non reflective plain looking.)

## Colors
We have simplyfied hardcoded colors to use instead of writing exact color code you can use constants.

- **white**
- **black**
- **purple**
- **blue**
- **green**
- **red**
- **yellow**
- **cyan**
- **pink**
- **gray**
- **orange**

Example:
```rust
logic::Color::purple()
```
   

## Setting up Lights
Lights can be added to the scene with various types and properties.
   - **Spotlight Light:** Defined by a angle, position, intensity, and color.
   - **Ambient Light:** Defined by intensity and color. (Position is irrelevant for ambient light)

## Configuring the Camera
   - **Position:** The position of the camera in the scene.
   - **Target Position:** Where the camera is looking at.
   - **Field of View:** The camera's field of view angle.
   - **Roll:** Rotation around the camera's forward axis.

## Rendering Options
Various options affect the rendering process:
   - **Max Rays:** Maximum number of rays to be traced per pixel.
   - **Gamma:** Gamma correction value.
   - **Diffuse, Specular, Shadows, Reflections:** Boolean flags to enable or disable certain rendering features.
   ```
   Diffuse is the spread of the light.
   Specular is the amount of light reflection
   Shadows is the shadows intensity
   Reflections is the amount of reflectivenes.
   ```

## Rendering the Scene
Call the `render()` function with a filename as the argument to render the scene and save it as an image file.

## Examples to Creating Objects

### Sphere

Creating a sphere involves defining its position, radius, and material properties.

```rust
Box::new(objects::Sphere {
    position: logic::Vec3::new(0.0, -5.0, -13.0),
    radius: 2.0,
    material: logic::Material {
        color: logic::Color::from_u8(0x40, 0xe0, 0xd0),
        diffuse: 0.6,
        specular: 5.0,
        specular_exponent: 500.0,
        reflectiveness: 0.0,
    },
})
```

## Setting up Lights
Point Light
Defining a point light requires specifying its position, intensity, and color.

```rust
logic::create_spotlight(
   logic::Vec3::new(-40.0, 20.0, 20.0), // Positioning the spotlight
   logic::Vec3::new(10., 4., -20.), // Making the spotlight look at a point centered among the objects
   1.,
   logic::Color::white(),
   10.,
   ),
```

## Configuring the Camera
Configuring the camera involves setting its position, target position, field of view, and roll.

```rust
logic::Camera::new(
    logic::Vec3::new(0., -3., 5.),
    logic::Vec3::new(0., 0., -20.),
    60.,
    aspect_ratio,
    0.,
)
```# raytracer
# raytracer
# raytracer
