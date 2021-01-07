use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{self, Num, Signed};
use rand::{rngs::ThreadRng, Rng};

pub trait Arithmetic:
    Clone + Copy + Add + AddAssign + Div + DivAssign + Mul + MulAssign + Sub + SubAssign
{
}

/// `Vector` struct is `Copy` to avoid ownership issues
/// functions are inlined to avoid copy overhead
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector<T>
where
    T: Arithmetic + Num,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Vector<T>
where
    T: Arithmetic + Num,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self.z
    }

    pub fn array(&self) -> [T; 3] {
        [self.x, self.y, self.z]
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn l2(&self) -> T {
        self.dot(self)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn uniform(n: T) -> Self {
        Self { x: n, y: n, z: n }
    }
}

impl<T> Vector<T>
where
    T: Arithmetic + Num + Signed,
{
    pub fn abs(&self) -> Self {
        Self {
            x: num::abs(self.x),
            y: num::abs(self.y),
            z: num::abs(self.z),
        }
    }
}

impl<T> Add for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Div for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> AddAssign for Vector<T>
where
    T: Arithmetic + Num,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> SubAssign for Vector<T>
where
    T: Arithmetic + Num,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> MulAssign for Vector<T>
where
    T: Arithmetic + Num,
{
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T> DivAssign for Vector<T>
where
    T: Arithmetic + Num,
{
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T> Add<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T> Sub<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> AddAssign<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl<T> SubAssign<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T> DivAssign<T> for Vector<T>
where
    T: Arithmetic + Num,
{
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T> Neg for Vector<T>
where
    T: Arithmetic + Neg<Output = T> + Num,
{
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Index<usize> for Vector<T>
where
    T: Arithmetic + Num,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl Vector<f64> {
    pub fn length(&self) -> f64 {
        self.l2().sqrt()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn o() -> Self {
        Self::uniform(0_f64)
    }

    pub fn i() -> Self {
        Self {
            x: 1_f64,
            y: 0_f64,
            z: 0_f64,
        }
    }

    pub fn j() -> Self {
        Self {
            x: 0_f64,
            y: 1_f64,
            z: 0_f64,
        }
    }

    pub fn k() -> Self {
        Self {
            x: 0_f64,
            y: 0_f64,
            z: 1_f64,
        }
    }

    pub fn random(trng: &mut ThreadRng) -> Self {
        let (x, y, z) = trng.gen();
        Self { x, y, z }
    }

    pub fn random_ball(radius: f64, trng: &mut ThreadRng) -> Self {
        loop {
            let rand_vec = Self::random(trng);
            if rand_vec.l2() <= 1_f64 {
                return rand_vec * radius;
            }
        }
    }
}

impl<T> Arithmetic for Vector<T> where T: Arithmetic + Num {}

impl Arithmetic for isize {}
impl Arithmetic for i16 {}
impl Arithmetic for i32 {}
impl Arithmetic for i64 {}
impl Arithmetic for i128 {}

impl Arithmetic for usize {}
impl Arithmetic for u16 {}
impl Arithmetic for u32 {}
impl Arithmetic for u64 {}
impl Arithmetic for u128 {}

impl Arithmetic for f32 {}
impl Arithmetic for f64 {}
