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

/// `f64`の誤差を許容する演算を提供する。
#[derive(Copy, Clone)]
pub struct Eps {
    eps: f64,
}

impl Eps {
    /// 誤差`eps`を設定して[`Eps`]を生成する。
    pub fn new(eps: f64) -> Self {
        Self { eps }
    }
    /// 誤差を許容して`a == b`なら`true`を返す。
    pub fn eq(&self, a: f64, b: f64) -> bool {
        (a - b).abs() < self.eps
    }
    /// 誤差を許容して`a != b`なら`true`を返す。
    pub fn ne(&self, a: f64, b: f64) -> bool {
        !self.eq(a, b)
    }
    /// 誤差を許容して`a < b`なら`true`を返す。
    pub fn lt(&self, a: f64, b: f64) -> bool {
        a - b < -self.eps
    }
    /// 誤差を許容して`a > b`なら`true`を返す。
    pub fn gt(&self, a: f64, b: f64) -> bool {
        a - b > self.eps
    }
    /// 誤差を許容して`a <= b`なら`true`を返す。
    pub fn le(&self, a: f64, b: f64) -> bool {
        self.lt(a, b) || self.eq(a, b)
    }
    /// 誤差を許容して`a >= b`なら`true`を返す。
    pub fn ge(&self, a: f64, b: f64) -> bool {
        self.gt(a, b) || self.eq(a, b)
    }
    /// `a`と`b`の比較をする。
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

/// 二次元ベクトル
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
    /// 絶対値を計算する
    pub fn abs(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    /// 絶対値の2乗を計算する
    pub fn abs_sq(self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }
    /// 内積を計算する
    pub fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }
    /// 外積を計算する
    pub fn cross(self, other: Self) -> f64 {
        self.0 * other.1 - self.1 * other.0
    }
    /// 長さを`1`にしたベクトルを返す
    pub fn unit(self) -> Self {
        self / self.abs()
    }
    /// 直交するベクトルを返す
    pub fn normal(self) -> Self {
        Self(-self.1, self.0)
    }
    /// ベクトルのなす角度を返す
    pub fn angle(self, other: Self) -> f64 {
        (other.1 - self.1).atan2(other.0 - self.0)
    }
    /// 極座標形式で`Vector`を生成する
    pub fn polar(r: f64, ang: f64) -> Self {
        Vector(r * ang.cos(), r * ang.sin())
    }
    /// `self`から`other`への角度($-\pi \le \theta \le \pi$)を返す。
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
    /// 2つのベクトルが等しければ`true`を返す。
    pub fn eq(self, other: Self, eps: Eps) -> bool {
        eps.eq(self.0, other.0) && eps.eq(self.1, other.1)
    }
}

/// 直線 (線分)
#[derive(Copy, Clone, Debug, Default)]
pub struct Line {
    /// 線分の始点
    pub from: Vector,
    /// 線分の終点
    pub to: Vector,
}

impl Line {
    /// 始点と終点を設定した[`Line`]を返す。
    pub fn new(from: Vector, to: Vector) -> Self {
        Self { from, to }
    }
    /// 線分方向の単位ベクトルを返す。
    pub fn unit(self) -> Vector {
        (self.to - self.from).unit()
    }
    /// 線分と直交するベクトルを返す。
    pub fn normal(self) -> Vector {
        (self.to - self.from).normal()
    }
    /// 始点から終点に向かうベクトルを返す。
    pub fn diff(self) -> Vector {
        self.to - self.from
    }
    /// 線分の大きさを返す。
    pub fn abs(self) -> f64 {
        self.diff().abs()
    }
    /// 2つの線分の内積を求める。
    pub fn dot(self, other: Self) -> f64 {
        self.diff().dot(other.diff())
    }
    /// 2つの線分の外積を求める。
    pub fn cross(self, other: Self) -> f64 {
        self.diff().cross(other.diff())
    }
    /// 2つの線分が等しければ`true`を返す。
    pub fn eq(self, other: Self, eps: Eps) -> bool {
        self.from.eq(other.from, eps) && self.to.eq(other.to, eps)
    }

    /// 点`p`から直線に引いた垂線と直線の交点を求める。
    pub fn projection(self, p: Vector) -> Vector {
        self.from + self.unit() * self.unit().dot(p - self.from)
    }

    /// 直線を対象軸とした点`p`と線対称の位置の点を求める。
    pub fn reflection(self, p: Vector) -> Vector {
        p + (self.projection(p) - p) * 2.0
    }

    /// 2つの直線が直交していれば`true`を返す。
    pub fn is_orthogonal(self, other: Self, eps: Eps) -> bool {
        eps.eq(self.dot(other).abs(), 0.0)
    }

    /// 2つの直線が平行していれば`true`を返す。
    pub fn is_parallel(self, other: Self, eps: Eps) -> bool {
        eps.eq(self.cross(other).abs(), 0.0)
    }
}

/// 円
#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
    /// 円の中心
    pub center: Vector,
    /// 円の半径
    pub radius: f64,
}

impl Circle {
    /// 中心`center`、半径`radius`の`Circle`を生成する
    pub fn new(center: Vector, radius: f64) -> Self {
        Circle { center, radius }
    }

    /// 円が等しいかを判定する
    pub fn eq(self, other: Self, eps: Eps) -> bool {
        self.center.eq(other.center, eps) && eps.eq(self.radius, other.radius)
    }
}
