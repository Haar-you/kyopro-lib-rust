pub mod ccw;

pub mod intersect_circle_line;
pub mod intersect_circle_segment;
pub mod intersect_circles;
pub mod intersect_line_segment;
pub mod intersect_segments;

pub mod dist_line_point;
pub mod dist_segment_point;
pub mod dist_segments;

pub mod area_intersection_circle_polygon;
pub mod area_intersection_circles;
pub mod area_polygon;

pub mod convex;
pub mod convex_cut;
pub mod convex_diameter;
pub mod convex_hull;

pub mod point_in_polygon;

pub mod circumcircle;
pub mod incircle;

pub mod common_tangent_circles;
pub mod tangent_circle;

pub mod closest_pair;

use std::{f64::consts::PI, marker::PhantomData};

pub trait EpsValue {
    fn eps() -> f64;
}

pub trait Eps:
    Copy
    + PartialEq
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Neg<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + Sized
    + From<f64>
{
    fn eps() -> f64;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn acos(self) -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn sq(self) -> Self;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct EpsFloat<E>(pub f64, PhantomData<E>);

impl<E> EpsFloat<E> {
    pub fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
}

macro_rules! eps_float_impl_one_arg {
    ($($f:tt),*) => { $(fn $f(self) -> Self { Self::new((self.0).$f()) })* }
}
macro_rules! eps_float_impl_two_arg {
    ($($f:tt),*) => { $(fn $f(self, other: Self) -> Self { Self::new((self.0).$f(other.0)) })* }
}

impl<E: EpsValue + Copy> Eps for EpsFloat<E> {
    fn eps() -> f64 {
        E::eps()
    }
    eps_float_impl_one_arg!(sin, cos, tan, acos, abs);
    eps_float_impl_two_arg!(atan2, max, min);

    fn sqrt(self) -> Self {
        Self::new(self.0.max(0.0).sqrt())
    }
    fn sq(self) -> Self {
        self * self
    }
}

impl<E: EpsValue> PartialEq for EpsFloat<E> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < E::eps()
    }
}

impl<E: EpsValue> PartialOrd for EpsFloat<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.0 - other.0 < -E::eps() {
            Some(std::cmp::Ordering::Less)
        } else if self.0 - other.0 > E::eps() {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

macro_rules! eps_ops {
    ( $($tr:ident, $fn:ident, $op:tt), * ) => {
        $(
            impl<E: EpsValue> std::ops::$tr for EpsFloat<E> {
                type Output = Self;
                fn $fn(self, other: Self) -> Self {
                    Self::new(self.0 $op other.0)
                }
            }
        )*
    }
}

eps_ops!(Add, add, +, Sub, sub, -, Mul, mul, *, Div, div, /);

macro_rules! eps_ops_assign {
    ( $($tr:ident, $fn:ident, $op:tt), * ) => {
        $(
            impl<E: EpsValue> std::ops::$tr for EpsFloat<E> {
                fn $fn(&mut self, other: Self) {
                    self.0 = self.0 $op other.0
                }
            }
        )*
    }
}

eps_ops_assign!(AddAssign, add_assign, +, SubAssign, sub_assign, -, MulAssign, mul_assign, *, DivAssign, div_assign, /);

impl<E> std::ops::Neg for EpsFloat<E> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0)
    }
}

impl<E> From<f64> for EpsFloat<E> {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl<E> std::fmt::Display for EpsFloat<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E> std::str::FromStr for EpsFloat<E> {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<f64>()?;
        Ok(Self::new(x))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector<T>(pub T, pub T);

impl<T: Eps> std::ops::Add for Vector<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<T: Eps> std::ops::Sub for Vector<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T: Eps> std::ops::Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl<T: Eps> std::ops::Div<T> for Vector<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl<T: Eps + std::fmt::Display> std::fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl<T: Eps> Vector<T> {
    pub fn abs(self) -> T {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    pub fn abs_sq(self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn dot(self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1
    }
    pub fn cross(self, other: Self) -> T {
        self.0 * other.1 - self.1 * other.0
    }
    pub fn unit(self) -> Self {
        self / self.abs()
    }
    pub fn normal(self) -> Self {
        Self(-self.1, self.0)
    }
    pub fn angle(self, other: Self) -> T {
        (other.1 - self.1).atan2(other.0 - self.0)
    }
    pub fn polar(r: T, ang: T) -> Self {
        Vector(r * ang.cos(), r * ang.sin())
    }
    pub fn angle_diff(self, other: Self) -> T {
        let r = other.1.atan2(other.0) - self.1.atan2(self.0);

        if r < T::from(-PI) {
            r + T::from(PI * 2.0)
        } else if r > T::from(PI) {
            r - T::from(PI * 2.0)
        } else {
            r
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Line<T> {
    pub from: Vector<T>,
    pub to: Vector<T>,
}

impl<T: Eps> Line<T> {
    pub fn new(from: Vector<T>, to: Vector<T>) -> Self {
        Self { from, to }
    }
    pub fn unit(self) -> Vector<T> {
        (self.to - self.from).unit()
    }
    pub fn normal(self) -> Vector<T> {
        (self.to - self.from).normal()
    }
    pub fn diff(self) -> Vector<T> {
        self.to - self.from
    }
    pub fn abs(self) -> T {
        self.diff().abs()
    }
    pub fn dot(self, other: Self) -> T {
        self.diff().dot(other.diff())
    }
    pub fn cross(self, other: Self) -> T {
        self.diff().cross(other.diff())
    }
}

impl<T: Eps> Line<T> {
    pub fn projection(self, p: Vector<T>) -> Vector<T> {
        self.from + self.unit() * self.unit().dot(p - self.from)
    }

    pub fn reflection(self, p: Vector<T>) -> Vector<T> {
        p + (self.projection(p) - p) * T::from(2.0)
    }

    pub fn is_orthogonal(self, other: Self) -> bool {
        self.dot(other).abs() == T::from(0.0)
    }

    pub fn is_parallel(self, other: Self) -> bool {
        self.cross(other).abs() == T::from(0.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Circle<T> {
    pub center: Vector<T>,
    pub radius: T,
}

impl<T> Circle<T> {
    pub fn new(center: Vector<T>, radius: T) -> Self {
        Circle { center, radius }
    }
}
