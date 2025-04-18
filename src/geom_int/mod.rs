pub mod arg_sort;
pub mod convex_hull;

use crate::impl_from;
use crate::impl_ops;
use std::f64::consts::PI;

/// 整数値をもつ二次元ベクトル
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VectorInt(pub i64, pub i64);

impl VectorInt {
    /// 絶対値を計算する
    pub fn abs(self) -> f64 {
        let x = self.0 as f64;
        let y = self.1 as f64;
        (x * x + y * y).sqrt()
    }
    /// 絶対値の2乗を計算する
    pub fn abs_sq(self) -> i64 {
        self.0 * self.0 + self.1 * self.1
    }
    /// 内積を計算する
    pub fn dot(self, other: Self) -> i64 {
        self.0 * other.0 + self.1 * other.1
    }
    /// 外積を計算する
    pub fn cross(self, other: Self) -> i64 {
        self.0 * other.1 - self.1 * other.0
    }
    /// 直交するベクトルを返す
    pub fn normal(self) -> Self {
        Self(-self.1, self.0)
    }
    /// ベクトルのなす角度を返す
    pub fn angle(self, other: Self) -> f64 {
        ((other.1 - self.1) as f64).atan2((other.0 - self.0) as f64)
    }
    /// `self`から`other`への角度($-\pi \le \theta \le \pi$)を返す。
    pub fn angle_diff(self, other: Self) -> f64 {
        let r = (other.1 as f64).atan2(other.0 as f64) - (self.1 as f64).atan2(self.0 as f64);

        if r < -PI {
            r + PI * 2.0
        } else if r > PI {
            r - PI * 2.0
        } else {
            r
        }
    }
}

impl_ops!(Add for VectorInt, |a: Self, b: Self| Self(a.0 + b.0, a.1 + b.1));
impl_ops!(Sub for VectorInt, |a: Self, b: Self| Self(a.0 - b.0, a.1 - b.1));
impl_ops!(Mul<i64> for VectorInt, |a: Self, k: i64| Self(a.0 * k, a.1 * k));
impl_ops!(Div<i64> for VectorInt, |a: Self, k: i64| Self(a.0 / k, a.1 / k));

impl_from!((i64, i64) => VectorInt, |value: (_, _)| Self(value.0,value.1));
impl_from!(VectorInt => (i64, i64), |value: VectorInt| (value.0, value.1));
