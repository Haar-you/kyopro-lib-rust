pub mod arg_sort;
pub mod convex_hull;

use crate::impl_from;
use crate::impl_ops;
use std::f64::consts::PI;

/// 整数値をもつ二次元ベクトル
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VectorInt {
    /// x座標
    pub x: i64,
    /// y座標
    pub y: i64,
}

impl VectorInt {
    /// 二次元ベクトル(x, y)を返す。
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// 絶対値を計算する
    pub fn abs(self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        (x * x + y * y).sqrt()
    }
    /// 絶対値の2乗を計算する
    pub fn abs_sq(self) -> i64 {
        self.x * self.x + self.y * self.y
    }
    /// 内積を計算する
    pub fn dot(self, other: Self) -> i64 {
        self.x * other.x + self.y * other.y
    }
    /// 外積を計算する
    pub fn cross(self, other: Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
    /// 直交するベクトルを返す
    pub fn normal(self) -> Self {
        Self::new(-self.y, self.x)
    }
    /// ベクトルのなす角度を返す
    pub fn angle(self, other: Self) -> f64 {
        ((other.y - self.y) as f64).atan2((other.x - self.x) as f64)
    }
    /// `self`から`other`への角度($-\pi \le \theta \le \pi$)を返す。
    pub fn angle_diff(self, other: Self) -> f64 {
        let r = (other.y as f64).atan2(other.x as f64) - (self.y as f64).atan2(self.x as f64);

        if r < -PI {
            r + PI * 2.0
        } else if r > PI {
            r - PI * 2.0
        } else {
            r
        }
    }
}

impl_ops!(Add for VectorInt, |a: Self, b: Self| Self::new(a.x + b.x, a.y + b.y));
impl_ops!(Sub for VectorInt, |a: Self, b: Self| Self::new(a.x - b.x, a.y - b.y));
impl_ops!(Mul<i64> for VectorInt, |a: Self, k: i64| Self::new(a.x * k, a.y * k));
impl_ops!(Div<i64> for VectorInt, |a: Self, k: i64| Self::new(a.x / k, a.y / k));

impl_from!((i64, i64) => VectorInt, |value: (_, _)| Self::new(value.0, value.1));
impl_from!(VectorInt => (i64, i64), |value: VectorInt| (value.x, value.y));
