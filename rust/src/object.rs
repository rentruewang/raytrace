use crate::{BoundingBox, HitData, Hittable, Material, Vector};

use std::{fmt::Debug, sync::Arc};

impl<'a> Hittable for Box<dyn Hittable + 'a> {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        self.as_ref().hit(source, towards)
    }

    fn bounds(&self) -> BoundingBox<f64> {
        self.as_ref().bounds()
    }
}

impl<'a> Hittable for Arc<dyn Hittable + 'a> {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        self.as_ref().hit(source, towards)
    }

    fn bounds(&self) -> BoundingBox<f64> {
        self.as_ref().bounds()
    }
}

#[derive(Debug)]
pub struct Sphere {
    center: Vector<f64>,
    radius: f64,
    matter: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector<f64>, radius: f64, matter: Arc<dyn Material>) -> Self {
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

    pub fn matter(&self) -> Arc<dyn Material> {
        Arc::clone(&self.matter)
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
            (neg, _) if neg.is_normal() && neg.is_sign_positive() => {
                let point = source + towards * neg;
                debug_assert!(self.bounds().through(source, towards));
                HitData::Hit {
                    t: neg,
                    point,
                    normal: self.normal(&point),
                    matter: Arc::clone(&self.matter),
                }
            }
            (_, pos) if pos.is_normal() && pos.is_sign_positive() => {
                let point = source + towards * pos;
                debug_assert!(self.bounds().through(source, towards));
                HitData::Hit {
                    t: pos,
                    point,
                    normal: self.normal(&point),
                    matter: Arc::clone(&self.matter),
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
