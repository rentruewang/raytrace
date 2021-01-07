#![feature(bindings_after_at)]
#![feature(iterator_fold_self)]

mod container;
mod hittable;
mod material;
mod object;
mod scene;
mod vector;

pub use container::{BoundingBox, List, Tree, TreeNode};
pub use hittable::{HitData, Hittable};
pub use material::{Glass, Material, Matte, Metal};
pub use object::Sphere;
pub use scene::Scene;
pub use vector::{Arithmetic, Vector};
