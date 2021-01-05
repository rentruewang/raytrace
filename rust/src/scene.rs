use crate::{BoundingBox, HitData, Hittable, Vector};

use std::f64;

use rand::{rngs::ThreadRng, Rng};

#[derive(Debug)]
pub struct Scene<'a> {
    source: Vector<f64>,
    corner: Vector<f64>,
    horizon: Vector<f64>,
    vertical: Vector<f64>,
    aperture: f64,

    object: Option<Box<dyn Hittable + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(
        source: Vector<f64>,
        corner: Vector<f64>,
        horizon: Vector<f64>,
        vertical: Vector<f64>,
        aperture: f64,
    ) -> Self {
        Self {
            source,
            corner,
            horizon,
            vertical,
            aperture,
            object: None,
        }
    }

    pub fn source(&self) -> Vector<f64> {
        self.source
    }

    pub fn corner(&self) -> Vector<f64> {
        self.corner
    }

    pub fn horizon(&self) -> Vector<f64> {
        self.horizon
    }

    pub fn vertical(&self) -> Vector<f64> {
        self.vertical
    }

    pub fn color_trace(
        &self,
        (starting, towards): (Vector<f64>, Vector<f64>),
        depth: usize,
        trng: &mut ThreadRng,
    ) -> Vector<f64> {
        let (mut starting, mut towards) = (starting, towards);
        let mut color = Vector::uniform(1_f64);
        for _ in 0..depth {
            if let HitData::Hit {
                point,
                normal,
                matter,
                ..
            } = self.hit(starting, towards)
            {
                let reflected = matter.scatter(towards, normal, trng);
                color *= matter.albedo();
                starting = point;
                towards = reflected;
            } else {
                let t = 0.5 * (towards.unit().y() + 1_f64);
                let background =
                    Vector::uniform(1_f64) * (1_f64 - t) + Vector::new(0.5, 0.7, 1_f64) * t;
                return color * background;
            }
        }
        Vector::uniform(0_f64)
    }

    pub fn color(
        &self,
        (i, j): (usize, usize),
        (nx, ny, ns): (usize, usize, usize),
        depth: usize,
        trng: &mut ThreadRng,
    ) -> [u8; 3] {
        let (dx, dy) = Scene::random_disk(self.aperture, trng);
        let h = self.horizon.unit() * dx;
        let v = self.vertical.unit() * dy;
        let start = self.source + h + v;

        let mut color = Vector::uniform(0_f64);
        for _ in 0..ns {
            let gens: (f64, f64) = (trng.gen(), trng.gen());
            let i = (i as f64 + gens.0) / nx as f64;
            let j = (j as f64 + gens.1) / ny as f64;
            let end = self.corner + self.horizon * i + self.vertical * j;
            let towards = end - start;

            color += self.color_trace((start, towards), depth, trng);
        }
        let array = (color / ns as f64 * 255.999).array();
        [array[0] as u8, array[1] as u8, array[2] as u8]
    }

    pub fn random_disk(radius: f64, trng: &mut ThreadRng) -> (f64, f64) {
        let (mut x, mut y): (f64, f64);
        loop {
            x = trng.gen();
            y = trng.gen();
            if x * x + y * y <= 1_f64 {
                return (x * radius, y * radius);
            }
        }
    }

    pub fn save(&mut self, object: Box<dyn Hittable + 'a>) {
        self.object = Some(object);
    }
}

impl<'a> Hittable for Scene<'a> {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        self.object.as_ref().unwrap().hit(source, towards)
    }

    fn bounds(&self) -> BoundingBox<f64> {
        self.object.as_ref().unwrap().bounds()
    }
}
