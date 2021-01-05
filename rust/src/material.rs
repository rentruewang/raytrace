use crate::Vector;

use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Matte {
        albedo: Vector<f64>,
    },
    Metal {
        albedo: Vector<f64>,
        blur: f64,
    },
    Glass {
        albedo: Vector<f64>,
        blur: f64,
        refractive: f64,
    },
}

impl Material {
    pub fn albedo(&self) -> Vector<f64> {
        match self {
            Material::Matte { albedo }
            | Material::Metal { albedo, .. }
            | Material::Glass { albedo, .. } => *albedo,
        }
    }
    pub fn scatter(
        &self,
        input: Vector<f64>,
        normal: Vector<f64>,
        trng: &mut ThreadRng,
    ) -> Vector<f64> {
        let input = input.unit();
        let normal = normal.unit();
        match self {
            Material::Matte { .. } => {
                // Mathematical Lambertian
                Vector::random_ball(1_f64, trng) + normal
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
                let cosine = input.dot(&normal);
                let ratio = if cosine.is_normal() && cosine.is_sign_negative() {
                    1_f64 / (*refractive)
                } else {
                    *refractive
                };
                let sine_squared = 1_f64 - cosine * cosine;
                let cosine_squared = 1_f64 - ratio * ratio * sine_squared;
                let refract = !cosine.is_nan()
                    && (cosine.is_sign_negative() || cosine_squared.is_sign_positive());

                let random: f64 = trng.gen();
                // shilick approximates the *reflect* probability
                // hence here we take the *refract* probablility which is (1 - schilick)
                if refract && random > Material::schilick(num::abs(cosine), *refractive) {
                    let rand_blur = Vector::random_ball(*blur, trng);
                    (input + normal * cosine) * ratio - normal * cosine_squared.sqrt() + rand_blur
                } else {
                    Material::mirror(input, normal, *blur, trng)
                }
            }
        }
    }

    fn mirror(
        input: Vector<f64>,
        normal: Vector<f64>,
        blur: f64,
        trng: &mut ThreadRng,
    ) -> Vector<f64> {
        let random = Vector::random_ball(blur, trng);
        let casted = normal * (input.dot(&normal) * 2_f64);
        random + input - casted
    }

    fn schilick(cosine: f64, ratio: f64) -> f64 {
        let r = (1_f64 - ratio) / (1_f64 + ratio);
        let sq = r * r;
        sq + (1_f64 - sq) * num::pow(1_f64 - cosine, 5)
    }
}
