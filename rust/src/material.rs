use crate::vector::Vector;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Matte {
        albedo: Vector,
    },
    Metal {
        albedo: Vector,
        blur: f64,
    },
    Glass {
        albedo: Vector,
        blur: f64,
        refractive: f64,
    },
}

impl Material {
    pub fn albedo(&self) -> Vector {
        match self {
            Material::Matte { albedo }
            | Material::Metal { albedo, .. }
            | Material::Glass { albedo, .. } => *albedo,
        }
    }
    pub fn scatter(&self, input: Vector, normal: Vector, trng: &mut ThreadRng) -> Vector {
        match self {
            Material::Matte { .. } => {
                // Mathematical Lambertian
                Vector::random(1.0, trng) + normal
            }
            // In reality, Rust translates
            // Material::Metal { blur, .. }
            // to this
            // &Material::Metal { ref blur, .. }
            // which is why we have to dereference blur
            Material::Metal { blur, .. } => {
                // Perfect reflection + little randomness
                Material::mirror(input, normal, *blur, trng)
            }
            Material::Glass {
                blur, refractive, ..
            } => {
                let input = input.unit();
                let cosine = input.dot(&normal);
                let ratio = if cosine < 0.0 {
                    1.0 / (*refractive)
                } else {
                    *refractive
                };
                let sine_squared = 1.0 - cosine * cosine;
                let cosine_squared = 1.0 - ratio * ratio * sine_squared;
                let refract = cosine <= 0.0 || cosine_squared >= 0.0;

                let random: f64 = trng.gen();
                // shilick approximates the *REFLECT* probability
                // hence here we take the refract probablility which is (1 - schilick)
                if refract && random > Material::schilick(num::abs(cosine), *refractive) {
                    let rand_blur = Vector::random(*blur, trng);
                    (input + normal * cosine) * ratio - normal * cosine_squared.sqrt() + rand_blur
                } else {
                    Material::mirror(input, normal, *blur, trng)
                }
            }
        }
    }

    fn mirror(input: Vector, normal: Vector, blur: f64, trng: &mut ThreadRng) -> Vector {
        let random = Vector::random(blur, trng);
        let casted = normal * (input.dot(&normal) * 2.0);
        random + input - casted
    }
    fn schilick(cosine: f64, ratio: f64) -> f64 {
        let r = (1.0 - ratio) / (1.0 + ratio);
        let sq = r * r;
        sq + (1.0 - sq) * num::pow(1.0 - cosine, 5)
    }
}
