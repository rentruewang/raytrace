use crate::{
    object::{HitData, Hittable},
    vector::Vector,
};

use std::cmp::PartialOrd;

// lifetime here exists to tell the compiler
// if a type containing a reference implements Hittable
// then that reference has to live long enough,
// so as not to invalidate the correctness of the program.
#[derive(Default)]
pub struct NaiveList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> NaiveList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, obj: impl Hittable + 'a) {
        self.objects.push(Box::new(obj));
    }

    // Dummy method to mirror `List`'s signature
    pub fn build(self) -> Self {
        self
    }
}

impl<'a> Hittable for NaiveList<'a> {
    fn hit(&self, source: Vector, target: Vector) -> HitData {
        let mut min_hit = HitData::Miss;
        let target = target.unit();
        for obj in self.objects.iter() {
            let data = obj.hit(source, target);
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

    fn bounds(&self) -> BoundingBox {
        todo!()
    }
}

#[derive(Default)]
pub struct BoundingBox {
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
}

impl BoundingBox {
    fn ordered<T>(arg: (T, T)) -> (T, T)
    where
        T: PartialOrd,
    {
        let (a, b) = arg;
        if a > b {
            (b, a)
        } else {
            (a, b)
        }
    }

    pub fn new(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Self {
        Self {
            x: Self::ordered(x),
            y: Self::ordered(y),
            z: Self::ordered(z),
        }
    }
}

/// Node represents the connection a tree node has
#[derive(Default)]
struct Node {
    this: usize,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct List<'a> {
    root: usize,
    nodes: Vec<Node>,
    // objects acts as both a placeholder and the list itself
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, obj: impl Hittable + 'a) {
        self.objects.push(Box::new(obj));
    }

    /// Build is required before any hit occurs
    pub fn build(self) -> Self {
        let Self { objects, .. } = self;

        // fn recursive_partition(list: Vec<Box<dyn Hittable +'a>>)

        todo!()
    }
}

impl<'a> Hittable for List<'a> {
    fn hit(&self, source: Vector, target: Vector) -> HitData {
        use HitData::{Hit, Miss};

        let root_node = self.nodes.get(self.root).unwrap();

        let left_hit = if let Some(left) = root_node.left {
            let obj = self.objects.get(left).unwrap();
            Some(obj.hit(source, target))
        } else {
            None
        };

        let right_hit = if let Some(right) = root_node.right {
            let obj = self.objects.get(right).unwrap();
            Some(obj.hit(source, target))
        } else {
            None
        };

        match (left_hit, right_hit) {
            (None, None) => Miss,
            (Some(data), None) | (None, Some(data)) => data,
            (Some(left_data), Some(right_data)) => match (left_data, right_data) {
                (Miss, Miss) => Miss,
                (data, Miss) | (Miss, data) => data,
                (hitl @ Hit { t: tl, .. }, hitr @ Hit { t: tr, .. }) => {
                    if tl > tr {
                        hitr
                    } else {
                        hitl
                    }
                }
            },
        }
    }

    fn bounds(&self) -> BoundingBox {
        todo!()
    }
}
