//! 幾何

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

use std::{cmp::Ordering, f64::consts::PI};

#[derive(Copy, Clone)]
pub struct Eps {
    eps: f64,
}

impl Eps {
    pub fn new(eps: f64) -> Self {
        Self { eps }
    }
    pub fn eq(&self, a: f64, b: f64) -> bool {
        (a - b).abs() < self.eps
    }
    pub fn ne(&self, a: f64, b: f64) -> bool {
        !self.eq(a, b)
    }
    pub fn lt(&self, a: f64, b: f64) -> bool {
        a - b < -self.eps
    }
    pub fn gt(&self, a: f64, b: f64) -> bool {
        a - b > self.eps
    }
    pub fn le(&self, a: f64, b: f64) -> bool {
        self.lt(a, b) || self.eq(a, b)
    }
    pub fn ge(&self, a: f64, b: f64) -> bool {
        self.gt(a, b) || self.eq(a, b)
    }
    pub fn partial_cmp(&self, a: f64, b: f64) -> Option<Ordering> {
        if self.eq(a, b) {
            Some(Ordering::Equal)
        } else if self.lt(a, b) {
            Some(Ordering::Less)
        } else if self.gt(a, b) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector(pub f64, pub f64);

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other)
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self(self.0 / other, self.1 / other)
    }
}

impl Vector {
    pub fn abs(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    pub fn abs_sq(self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
    pub fn cross(self, other: Self) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }
    pub fn unit(self) -> Self {
        self / self.abs()
    }
    pub fn normal(self) -> Self {
        Self(-self.1, self.0)
    }
    pub fn angle(self, other: Self) -> f64 {
        (other.1 - self.1).atan2(other.0 - self.0)
    }
    pub fn polar(r: f64, ang: f64) -> Self {
        Vector(r * ang.cos(), r * ang.sin())
    }
    pub fn angle_diff(self, other: Self) -> f64 {
        let r = other.1.atan2(other.0) - self.1.atan2(self.0);

        if r < -PI {
            r + PI * 2.0
        } else if r > PI {
            r - PI * 2.0
        } else {
            r
        }
    }
    pub fn eq(self, other: Self, eps: Eps) -> bool {
        eps.eq(self.0, other.0) && eps.eq(self.1, other.1)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Line {
    pub from: Vector,
    pub to: Vector,
}

impl Line {
    pub fn new(from: Vector, to: Vector) -> Self {
        Self { from, to }
    }
    pub fn unit(self) -> Vector {
        (self.to - self.from).unit()
    }
    pub fn normal(self) -> Vector {
        (self.to - self.from).normal()
    }
    pub fn diff(self) -> Vector {
        self.to - self.from
    }
    pub fn abs(self) -> f64 {
        self.diff().abs()
    }
    pub fn dot(self, other: Self) -> f64 {
        self.diff().dot(other.diff())
    }
    pub fn cross(self, other: Self) -> f64 {
        self.diff().cross(other.diff())
    }
    pub fn eq(self, other: Self, eps: Eps) -> bool {
        self.from.eq(other.from, eps) && self.to.eq(other.to, eps)
    }

    pub fn projection(self, p: Vector) -> Vector {
        self.from + self.unit() * self.unit().dot(p - self.from)
    }

    pub fn reflection(self, p: Vector) -> Vector {
        p + (self.projection(p) - p) * 2.0
    }

    pub fn is_orthogonal(self, other: Self, eps: Eps) -> bool {
        eps.eq(self.dot(other).abs(), 0.0)
    }

    pub fn is_parallel(self, other: Self, eps: Eps) -> bool {
        eps.eq(self.cross(other).abs(), 0.0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
    pub center: Vector,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Vector, radius: f64) -> Self {
        Circle { center, radius }
    }

    pub fn eq(self, other: Self, eps: Eps) -> bool {
        self.center.eq(other.center, eps) && eps.eq(self.radius, other.radius)
    }
}
