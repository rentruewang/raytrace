use crate::{material::Material, vector::Vector};

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
    fn hit(&self, source: Vector, towards: Vector) -> HitData;
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
}

impl Hittable for Sphere {
    fn hit(&self, source: Vector, towards: Vector) -> HitData {
        let radius = self.radius;

        let oc = self.normal(&source);
        let a = towards.l2();
        let b = oc.dot(&towards);
        let c = oc.l2() - radius * radius;

        let base = (b * b - a * c).sqrt();
        let neg = (-b - base) / a;
        let pos = (-b + base) / a;

        match (neg, pos) {
            (neg, _) if neg > 0.0 => {
                let point = source + towards * neg;
                HitData::Hit {
                    t: neg,
                    point,
                    normal: self.normal(&point),
                    matter: self.matter,
                }
            }
            (_, pos) if pos > 0.0 => {
                let point = source + towards * pos;
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
}
