use crate::{Arithmetic, HitData, Hittable, Vector};

use std::{cmp::PartialOrd, sync::Arc};

use num::{self, Num};

// lifetime here exists to tell the compiler
// if a type containing a reference implements Hittable
// then that reference has to live long enough,
// so as not to invalidate the correctness of the program.
#[derive(Debug, Default)]
pub struct List<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, obj: impl Hittable + 'a) {
        self.objects.push(Box::new(obj));
    }
}

impl<'a> Hittable for List<'a> {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        self.objects.iter().fold(HitData::Miss, |min_hit, obj| {
            let mut output = min_hit;
            let data = obj.hit(source, towards);
            if let HitData::Hit { t, .. } = data {
                match min_hit {
                    HitData::Hit { t: min_t, .. } if t < min_t => output = data,
                    HitData::Hit { .. } => (),
                    HitData::Miss => output = data,
                }
            }

            output
        })
    }

    fn bounds(&self) -> BoundingBox<f64> {
        self.objects
            .iter()
            .map(|obj| obj.bounds())
            .fold_first(|acc, val| BoundingBox::wraps(acc, val))
            .unwrap()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BoundingBox<T>
where
    T: Copy + Num + Send + Sync,
{
    x: (T, T),
    y: (T, T),
    z: (T, T),
}

impl<T> BoundingBox<T>
where
    T: Arithmetic + Num + PartialOrd + Send + Sync,
{
    fn ordered(arg: (T, T)) -> (T, T) {
        let (a, b) = arg;
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn new(x: (T, T), y: (T, T), z: (T, T)) -> Self {
        Self {
            x: Self::ordered(x),
            y: Self::ordered(y),
            z: Self::ordered(z),
        }
    }

    fn larger_bound(this: (T, T), other: (T, T)) -> (T, T) {
        (
            if this.0 < other.0 { this.0 } else { other.0 },
            if this.1 > other.1 { this.1 } else { other.1 },
        )
    }

    pub fn wraps(this: Self, other: Self) -> Self {
        Self {
            x: Self::larger_bound(this.x, other.x),
            y: Self::larger_bound(this.y, other.y),
            z: Self::larger_bound(this.z, other.z),
        }
    }

    pub fn min(&self) -> Vector<T> {
        debug_assert!(self.x.0 <= self.x.1);
        debug_assert!(self.y.0 <= self.y.1);
        debug_assert!(self.z.0 <= self.z.1);
        Vector::new(self.x.0, self.y.0, self.z.0)
    }

    pub fn max(&self) -> Vector<T> {
        debug_assert!(self.x.1 >= self.x.0);
        debug_assert!(self.y.1 >= self.y.0);
        debug_assert!(self.z.1 >= self.z.0);
        Vector::new(self.x.1, self.y.1, self.z.1)
    }
}

impl BoundingBox<f64> {
    pub fn center(&self) -> Vector<f64> {
        Vector::new(
            (self.x.0 + self.x.1) / 2_f64,
            (self.y.0 + self.y.1) / 2_f64,
            (self.z.0 + self.z.1) / 2_f64,
        )
    }

    pub fn through(&self, source: Vector<f64>, towards: Vector<f64>) -> bool {
        let (min, max) = (self.min(), self.max());

        let (mut t_min, mut t_max) = (f64::MIN, f64::MAX);
        for i in 0..3 {
            let inv_b = 1_f64 / towards[i];
            let t_small = (min[i] - source[i]) * inv_b;
            let t_large = (max[i] - source[i]) * inv_b;

            let (t_small, t_large) = if inv_b.is_sign_negative() {
                (t_large, t_small)
            } else {
                (t_small, t_large)
            };

            debug_assert!(t_small <= t_large);

            t_min = if t_small > t_min { t_small } else { t_min };
            t_max = if t_large < t_max { t_large } else { t_max };
        }

        t_min < t_max
    }
}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn maximum_variance<'a>(list: &[Arc<dyn Hittable + 'a>]) -> Self {
        let len = list.len() as f64;

        // ! workaround
        // using fold here because the iter::sum() does not work
        let center: Vector<f64> = list
            .iter()
            .map(|obj: &Arc<dyn Hittable>| obj.bounds().center())
            .fold(Vector::default(), |acc, val| acc + val)
            / len;

        let naive_var: Vector<f64> = list
            .iter()
            .map(|obj| (obj.bounds().center() - center).abs())
            .fold(Vector::default(), |acc, val| acc + val);

        // ! workaround
        // using if's here cause I've not thought of a better solution
        if naive_var.x() > naive_var.y() && naive_var.x() > naive_var.z() {
            Axis::X
        } else if naive_var.y() > naive_var.z() {
            Axis::Y
        } else {
            Axis::Z
        }
    }
}

#[derive(Debug)]
pub struct TreeNode<'a> {
    bounds: BoundingBox<f64>,

    left: Arc<dyn Hittable + 'a>,
    right: Arc<dyn Hittable + 'a>,
}

impl<'a> TreeNode<'a> {
    pub fn new(left: Arc<dyn Hittable + 'a>, right: Arc<dyn Hittable + 'a>) -> Self {
        let (left_bound, right_bound) = (left.bounds(), right.bounds());
        let bounds = BoundingBox::wraps(left_bound, right_bound);

        Self {
            bounds,
            left,
            right,
        }
    }
}

impl<'a> Hittable for TreeNode<'a> {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        if !self.bounds.through(source, towards) {
            return HitData::Miss;
        }

        match (
            self.left.hit(source, towards),
            self.right.hit(source, towards),
        ) {
            (HitData::Miss, HitData::Miss) => HitData::Miss,
            (hit @ HitData::Hit { .. }, HitData::Miss)
            | (HitData::Miss, hit @ HitData::Hit { .. }) => hit,
            (
                left_hit @ HitData::Hit { t: left_t, .. },
                right_hit @ HitData::Hit { t: right_t, .. },
            ) => {
                if left_t < right_t {
                    left_hit
                } else {
                    right_hit
                }
            }
        }
    }

    fn bounds(&self) -> BoundingBox<f64> {
        self.bounds
    }
}

#[derive(Debug)]
pub struct Tree<'a> {
    root: Arc<dyn Hittable + 'a>,
}

impl<'a> Tree<'a> {
    fn recursive_partition(mut list: Vec<Arc<dyn Hittable + 'a>>) -> Arc<dyn Hittable + 'a> {
        match list.len() {
            0 => unreachable!(),
            1 => return Arc::clone(list.get(0).unwrap()),
            2 => {
                let left = Arc::clone(list.get(0).unwrap());
                let right = Arc::clone(list.get(1).unwrap());

                Arc::new(TreeNode::new(left, right))
            }
            n => {
                let compare = match Axis::maximum_variance(&list) {
                    Axis::X => |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                        a.bounds()
                            .center()
                            .x()
                            .partial_cmp(&b.bounds().center().x())
                            .unwrap()
                    },
                    Axis::Y => |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                        a.bounds()
                            .center()
                            .y()
                            .partial_cmp(&b.bounds().center().y())
                            .unwrap()
                    },
                    Axis::Z => |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
                        a.bounds()
                            .center()
                            .z()
                            .partial_cmp(&b.bounds().center().z())
                            .unwrap()
                    },
                };

                list.sort_by(compare);

                let (left, right) = list.split_at(n / 2);

                let left_node = Self::recursive_partition(left.to_vec());
                let right_node = Self::recursive_partition(right.to_vec());

                Arc::new(TreeNode::new(left_node, right_node))
            }
        }
    }

    pub fn build(list: List<'a>) -> Self {
        let List { objects } = list;
        let shared: Vec<_> = objects.into_iter().map(Arc::from).collect();

        let root = Self::recursive_partition(shared);

        Self { root }
    }
}

impl<'a> Hittable for Tree<'a> {
    fn hit(&self, source: Vector<f64>, towards: Vector<f64>) -> HitData {
        self.root.hit(source, towards)
    }
    fn bounds(&self) -> BoundingBox<f64> {
        self.root.bounds()
    }
}
