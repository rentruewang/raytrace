#![feature(bindings_after_at)]
#![feature(iterator_fold_self)]

mod container;
mod material;
mod object;
mod scene;
mod vector;

pub use container::{BoundingBox, List, Tree, TreeNode};
pub use material::Material;
pub use object::{HitData, Hittable, Sphere};
pub use scene::Scene;
pub use vector::{Arithmetic, Vector};
