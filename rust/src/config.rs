use raytrace_rs::{Material, Scene, Sphere, Vector};
use std::f64::consts;

pub const NX: usize = 800;
pub const NY: usize = 400;
pub const NS: usize = 100;
pub const DEP: usize = 50;
pub const DEG: f64 = 90.0;
pub const RATIO: f64 = NX as f64 / NY as f64;
pub const APERTURE: f64 = 0.01;

pub fn create<'a>() -> Scene<'a> {
    let eye = Vector::new(0.0, 0.0, -0.3);
    let lookat = Vector::new(0.0, 0.0, -1.0);
    let viewup = Vector::new(0.0, 1.0, 0.0);

    let vision = lookat - eye;
    let rad = (DEG / 2.0) / 180.0 * consts::PI;
    let height = rad.tan() * vision.length();
    let width = height * RATIO;

    let unit = vision.unit();
    let projection = unit * viewup.dot(&unit);
    let mut viewup = (viewup - projection).unit();
    let mut horizon = vision.cross(&viewup).unit();

    viewup *= height;
    horizon *= width;

    let mut scn = Scene::new(
        eye,
        lookat - viewup - horizon,
        horizon * 2.0,
        viewup * 2.0,
        APERTURE,
    );

    scn.register(Sphere::new(
        Vector::new(0.0, 0.0, -1.0),
        0.5,
        Material::Glass {
            albedo: Vector::new(1.0, 1.0, 1.0),
            blur: 0.0,
            refractive: 1.5,
        },
    ));
    scn.register(Sphere::new(
        Vector::new(0.0, -100.5, -1.0),
        100.0,
        Material::Matte {
            albedo: Vector::new(0.9, 0.9, 0.0),
        },
    ));
    scn.register(Sphere::new(
        Vector::new(-1.0, 0.0, -1.0),
        0.5,
        Material::Matte {
            albedo: Vector::new(0.8, 0.3, 0.3),
        },
    ));
    scn.register(Sphere::new(
        Vector::new(1.0, 0.0, -1.0),
        0.5,
        Material::Metal {
            albedo: Vector::new(0.95, 0.95, 0.95),
            blur: 0.1,
        },
    ));

    scn
}
