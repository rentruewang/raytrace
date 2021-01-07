use crate::{BoundingBox, Material, Vector};

use std::{fmt::Debug, sync::Arc};

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData;
    fn bounds(&self) -> BoundingBox<f64>;
}

#[derive(Clone, Debug)]
pub enum HitData {
    Miss,
    Hit {
        t: f64,
        point: Vector<f64>,
        normal: Vector<f64>,
        matter: Arc<dyn Material>,
    },
}
