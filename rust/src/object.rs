use crate::{container::BoundingBox, material::Material, vector::Vector};

use num::Num;

#[derive(Clone, Copy, Debug)]
pub enum HitData {
    Hit {
        t: f64,
        point: Vector,
        normal: Vector,
        matter: Material,
    },
    Miss,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, source: Vector, target: Vector) -> HitData;
    fn bounds(&self) -> BoundingBox;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vector,
    radius: f64,
    matter: Material,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, matter: Material) -> Self {
        Self {
            center,
            radius,
            matter,
        }
    }

    pub fn center(&self) -> Vector {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn matter(&self) -> Material {
        self.matter
    }

    pub fn normal(&self, point: &Vector) -> Vector {
        *point - self.center
    }

    fn square<T>(n: T) -> T
    where
        T: Copy + Num,
    {
        n * n
    }
}

impl Hittable for Sphere {
    fn hit(&self, source: Vector, target: Vector) -> HitData {
        let radius = self.radius;

        let oc = self.normal(&source);
        let a = target.l2();
        let b = oc.dot(&target);
        let c = oc.l2() - radius * radius;

        let base = (Self::square(b) - a * c).sqrt();
        let neg = (-b - base) / a;
        let pos = (-b + base) / a;

        match (neg, pos) {
            (neg, _) if neg > 0.0 => {
                let point = source + target * neg;
                HitData::Hit {
                    t: neg,
                    point,
                    normal: self.normal(&point),
                    matter: self.matter,
                }
            }
            (_, pos) if pos > 0.0 => {
                let point = source + target * pos;
                HitData::Hit {
                    t: pos,
                    point,
                    normal: self.normal(&point),
                    matter: self.matter,
                }
            }
            _ => HitData::Miss,
        }
    }

    fn bounds(&self) -> BoundingBox {
        let center = self.center;
        let (x, y, z) = (center.x(), center.y(), center.z());
        let r = self.radius;
        BoundingBox::new((x - r, x + r), (y - r, y + r), (z - r, z + r))
    }
}
