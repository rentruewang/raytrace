use crate::{BoundingBox, Material, Vector};

use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub enum HitData {
    Miss,
    Hit {
        t: f64,
        point: Vector<f64>,
        normal: Vector<f64>,
        matter: Material,
    },
}

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData;
    fn bounds(&self) -> BoundingBox<f64>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vector<f64>,
    radius: f64,
    matter: Material,
}

impl Sphere {
    pub fn new(center: Vector<f64>, radius: f64, matter: Material) -> Self {
        Self {
            center,
            radius,
            matter,
        }
    }

    pub fn center(&self) -> Vector<f64> {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn matter(&self) -> Material {
        self.matter
    }

    pub fn normal(&self, point: &Vector<f64>) -> Vector<f64> {
        *point - self.center
    }
}

impl Hittable for Sphere {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        let radius = self.radius;

        let oc = self.normal(&source);
        let a = towards.l2();
        let b = oc.dot(&towards);
        let c = oc.l2() - radius * radius;

        let base = (b.powi(2) - a * c).sqrt();
        let neg = (-b - base) / a;
        let pos = (-b + base) / a;

        match (neg, pos) {
            (neg, _) if !neg.is_nan() && neg.is_sign_positive() => {
                let point = source + towards * neg;
                debug_assert!(self.bounds().through(source, towards));
                HitData::Hit {
                    t: neg,
                    point,
                    normal: self.normal(&point),
                    matter: self.matter,
                }
            }
            (_, pos) if !pos.is_nan() && pos.is_sign_positive() => {
                let point = source + towards * pos;
                debug_assert!(self.bounds().through(source, towards));
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

    fn bounds(&self) -> BoundingBox<f64> {
        let min = self.center - self.radius;
        let max = self.center + self.radius;
        BoundingBox::new((min.x(), max.x()), (min.y(), max.y()), (min.z(), max.z()))
    }
}
