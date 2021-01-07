use crate::Vector;

use std::fmt::Debug;

use dyn_clone::DynClone;
use rand::{rngs::ThreadRng, Rng};

pub trait Material: Debug + DynClone + Send + Sync {
    fn scatter(&self, input: Vector<f64>, normal: Vector<f64>, trng: &mut ThreadRng)
        -> Vector<f64>;
    fn albedo(&self) -> Vector<f64>;
}

dyn_clone::clone_trait_object!(Material);

fn mirror(input: Vector<f64>, normal: Vector<f64>, blur: f64, trng: &mut ThreadRng) -> Vector<f64> {
    let random = Vector::random_ball(blur, trng);
    let casted = normal * (input.dot(&normal) * 2_f64);
    random + input - casted
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Matte {
    albedo: Vector<f64>,
}

impl Matte {
    pub fn new(albedo: Vector<f64>) -> Self {
        Self { albedo }
    }
}

impl Material for Matte {
    fn scatter(
        &self,
        _input: Vector<f64>,
        normal: Vector<f64>,
        trng: &mut ThreadRng,
    ) -> Vector<f64> {
        let normal = normal.unit();
        Vector::random_ball(1_f64, trng) + normal
    }

    fn albedo(&self) -> Vector<f64> {
        self.albedo
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Metal {
    albedo: Vector<f64>,
    blur: f64,
}

impl Metal {
    pub fn new(albedo: Vector<f64>, blur: f64) -> Self {
        Self { albedo, blur }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        input: Vector<f64>,
        normal: Vector<f64>,
        trng: &mut ThreadRng,
    ) -> Vector<f64> {
        let input = input.unit();
        let normal = normal.unit();
        mirror(input, normal, self.blur, trng)
    }

    fn albedo(&self) -> Vector<f64> {
        self.albedo
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Glass {
    albedo: Vector<f64>,
    blur: f64,
    refractive: f64,
}

impl Glass {
    pub fn new(albedo: Vector<f64>, blur: f64, refractive: f64) -> Self {
        Self {
            albedo,
            blur,
            refractive,
        }
    }
}

impl Glass {
    fn schilick(cosine: f64, ratio: f64) -> f64 {
        let r = (1_f64 - ratio) / (1_f64 + ratio);
        let sq = r * r;
        sq + (1_f64 - sq) * num::pow(1_f64 - cosine, 5)
    }
}

impl Material for Glass {
    fn scatter(
        &self,
        input: Vector<f64>,
        normal: Vector<f64>,
        trng: &mut ThreadRng,
    ) -> Vector<f64> {
        let input = input.unit();
        let normal = normal.unit();
        let blur = self.blur;
        let refractive = self.refractive;

        let cosine = input.dot(&normal);
        let ratio = if cosine.is_normal() && cosine.is_sign_negative() {
            1_f64 / refractive
        } else {
            refractive
        };
        let sine_squared = 1_f64 - cosine * cosine;
        let cosine_squared = 1_f64 - ratio * ratio * sine_squared;
        let refract =
            !cosine.is_nan() && (cosine.is_sign_negative() || cosine_squared.is_sign_positive());

        let random: f64 = trng.gen();
        // shilick approximates the *reflect* probability
        // hence here we take the *refract* probablility which is (1 - schilick)
        if refract && random > Self::schilick(num::abs(cosine), refractive) {
            let rand_blur = Vector::random_ball(blur, trng);
            (input + normal * cosine) * ratio - normal * cosine_squared.sqrt() + rand_blur
        } else {
            mirror(input, normal, blur, trng)
        }
    }
    fn albedo(&self) -> Vector<f64> {
        self.albedo
    }
}
