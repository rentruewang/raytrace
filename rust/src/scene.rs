use crate::{
    object::{HitData, Hittable},
    vector::Vector,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64;

pub struct Scene<'a> {
    source: Vector,
    corner: Vector,
    horizon: Vector,
    vertical: Vector,
    // lifetime here exists to tell the compiler
    // if a type containing a reference implements Hittable
    // then that reference has to live long enough,
    // so as not to invalidate the correctness of the program.
    objects: Vec<Box<dyn Hittable + 'a>>,
    aperture: f64,
}

impl<'a> Scene<'a> {
    pub fn new(
        source: Vector,
        corner: Vector,
        horizon: Vector,
        vertical: Vector,
        aperture: f64,
    ) -> Self {
        Self {
            source,
            corner,
            horizon,
            vertical,
            objects: Vec::new(),
            aperture,
        }
    }

    pub fn source(&self) -> Vector {
        self.source
    }
    pub fn corner(&self) -> Vector {
        self.corner
    }
    pub fn horizon(&self) -> Vector {
        self.horizon
    }
    pub fn vertical(&self) -> Vector {
        self.vertical
    }

    pub fn color_trace(
        &self,
        (starting, towards): (Vector, Vector),
        depth: usize,
        trng: &mut ThreadRng,
    ) -> Vector {
        let (mut starting, mut towards) = (starting, towards);
        let mut color: Vector = Vector::new(1.0, 1.0, 1.0);
        for _ in 0..depth {
            if let HitData::Hit {
                point,
                normal,
                ref matter,
                ..
            } = self.ray(starting, towards)
            {
                let reflected = matter.scatter(towards, normal.unit(), trng);
                color *= matter.albedo();
                starting = point;
                towards = reflected;
            } else {
                let t = 0.5 * (towards.unit().y() + 1.0);
                let background =
                    Vector::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector::new(0.5, 0.7, 1.0) * t;
                return color * background;
            }
        }
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn color(
        &self,
        (i, j): (usize, usize),
        (nx, ny, ns): (usize, usize, usize),
        depth: usize,
        trng: &mut ThreadRng,
    ) -> [u8; 3] {
        let (dx, dy) = Scene::random_disk(self.aperture, trng);
        let h: Vector = self.horizon.unit() * dx;
        let v: Vector = self.vertical.unit() * dy;
        let start = self.source + h + v;

        let mut color = Vector::new(0.0, 0.0, 0.0);
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

    pub fn ray(&self, starting: Vector, towards: Vector) -> HitData {
        let mut min_hit = HitData::Miss;
        let towards = towards.unit();
        for obj in self.objects.iter() {
            let data = obj.hit(starting, towards);
            if let HitData::Hit { t, .. } = data {
                match min_hit {
                    HitData::Hit { t: min_t, .. } if t < min_t => min_hit = data,
                    HitData::Hit { .. } => (),
                    HitData::Miss => min_hit = data,
                }
            }
        }
        min_hit
    }

    pub fn register(&mut self, obj: impl Hittable + 'a) {
        self.objects.push(Box::new(obj))
    }

    pub fn random_disk(radius: f64, trng: &mut ThreadRng) -> (f64, f64) {
        let (mut x, mut y): (f64, f64);
        loop {
            x = trng.gen();
            y = trng.gen();
            if x * x + y * y <= 1.0 {
                return (x * radius, y * radius);
            }
        }
    }
}
