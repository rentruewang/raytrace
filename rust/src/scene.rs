use crate::{
    container::{BoundingBox, NaiveList},
    object::{HitData, Hittable},
    vector::Vector,
};

use std::f64;

use rand::{rngs::ThreadRng, Rng};

pub struct Scene<'a> {
    source: Vector,
    corner: Vector,
    horizon: Vector,
    vertical: Vector,
    list: NaiveList<'a>,
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
            list: NaiveList::new(),
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
        let mut color: Vector = Vector::new(1_f64, 1_f64, 1_f64);
        for _ in 0..depth {
            if let HitData::Hit {
                point,
                normal,
                ref matter,
                ..
            } = self.hit(starting, towards)
            {
                let reflected = matter.scatter(towards, normal.unit(), trng);
                color *= matter.albedo();
                starting = point;
                towards = reflected;
            } else {
                let t = 0.5 * (towards.unit().y() + 1_f64);
                let background = Vector::new(1_f64, 1_f64, 1_f64) * (1_f64 - t)
                    + Vector::new(0.5, 0.7, 1_f64) * t;
                return color * background;
            }
        }
        Vector::new(0_f64, 0_f64, 0_f64)
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

        let mut color = Vector::new(0_f64, 0_f64, 0_f64);
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

    pub fn register(&mut self, obj: impl Hittable + 'a) {
        self.list.register(obj);
    }

    /// Build should be called before all hit operations
    pub fn build(mut self) -> Self {
        self.list = self.list.build();
        self
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
}

impl<'a> Hittable for Scene<'a> {
    fn hit(&self, source: Vector, target: Vector) -> HitData {
        self.list.hit(source, target)
    }

    fn bounds(&self) -> BoundingBox {
        self.list.bounds()
    }
}
