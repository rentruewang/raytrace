use raytrace_rs::{Hittable, List, Material, Scene, Sphere, Tree, Vector};

use rand::{self, Rng};

// pub const NX: usize = 1200;
// pub const NY: usize = 675;

pub const NX: usize = 400;
pub const NY: usize = 225;

pub const NS: usize = 100;
pub const DEP: usize = 50;

pub const DEG: f64 = 30_f64;
pub const APERTURE: f64 = 0_f64;

pub const TREE: bool = true;
pub const RATIO: f64 = NX as f64 / NY as f64;

fn material(material_code: f64, albedo: Vector<f64>, blur: f64, refractive: f64) -> Material {
    let mat = (material_code * 3_f64) as usize;
    let albedo = albedo / 2_f64 + 0.5;
    let blur = blur / 2_f64;
    let refractive = refractive + 1_f64;
    match mat {
        0 => Material::Matte { albedo },
        1 => Material::Metal { albedo, blur },
        2 => Material::Glass {
            albedo,
            blur,
            refractive,
        },
        _ => unreachable!(),
    }
}

pub fn create<'a>() -> Scene<'a> {
    use std::f64::consts::PI;

    let eye = Vector::new(13_f64, 2_f64, 3_f64);
    let lookat = Vector::new(0_f64, 0_f64, 0_f64);
    let viewup = Vector::new(0_f64, 1_f64, 0_f64);

    let vision = lookat - eye;
    let rad = PI * DEG / 360_f64;
    let height = rad.tan() * vision.length();
    let width = height * RATIO;

    let unit = vision.unit();
    let projection = unit * viewup.dot(&unit);
    let mut viewup = (viewup - projection).unit();
    let mut horizon = vision.cross(&viewup).unit();

    viewup *= height;
    horizon *= width;

    let mut list = List::new();

    let trng = &mut rand::thread_rng();

    for i in -11..=11 {
        for j in -11..=11 {
            let i = i as f64;
            let j = j as f64;

            let center = Vector::new(
                i + 0.9 * trng.gen::<f64>(),
                0.2,
                j + 0.9 * trng.gen::<f64>(),
            );

            list.register(Sphere::new(
                center,
                0.2,
                material(trng.gen(), Vector::random(trng), trng.gen(), trng.gen()),
            ));
        }
    }

    list.register(Sphere::new(
        Vector::new(0_f64, -1000_f64, 0_f64),
        1000_f64,
        Material::Matte {
            albedo: Vector::new(0.9, 0.9, 0_f64),
        },
    ));

    list.register(Sphere::new(
        Vector::j(),
        1_f64,
        Material::Glass {
            albedo: Vector::uniform(1_f64),
            blur: 0_f64,
            refractive: 1.5,
        },
    ));

    list.register(Sphere::new(
        Vector::new(-4_f64, 1_f64, 0_f64),
        1_f64,
        Material::Matte {
            albedo: Vector::new(0.4, 0.2, 0.1),
        },
    ));

    list.register(Sphere::new(
        Vector::new(4_f64, 1_f64, 0_f64),
        1_f64,
        Material::Metal {
            albedo: Vector::new(0.7, 0.6, 0.5),
            blur: 0_f64,
        },
    ));

    let mut scn = Scene::new(
        eye,
        lookat - viewup - horizon,
        horizon * 2_f64,
        viewup * 2_f64,
        APERTURE,
    );

    let list: Box<dyn Hittable> = if TREE {
        Box::new(Tree::build(list))
    } else {
        Box::new(list)
    };

    scn.save(list);

    scn
}
