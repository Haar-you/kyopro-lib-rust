//! $\mathbb{Z} / m \mathbb{Z}$の環
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// $\mathbb{Z} / m \mathbb{Z}$の環
#[allow(clippy::wrong_self_convention)]
pub trait ZZ: Clone {
    /// 環の元の型
    type Element: ZZElem;
    /// `u64`から生成する。
    fn from_u64(&self, a: u64) -> Self::Element;
    /// `i64`から生成する。
    fn from_i64(&self, a: i64) -> Self::Element;
    /// 加法の単位元を返す。
    fn zero(&self) -> Self::Element {
        self.from_u64(0)
    }
    /// 乗法の単位元を返す。
    fn one(&self) -> Self::Element {
        self.from_u64(1)
    }
    /// 剰余の除数を返す。
    fn modulo(&self) -> u32;
}

/// $\mathbb{Z} / m \mathbb{Z}$の環の元
pub trait ZZElem:
    Sized
    + Copy
    + PartialEq
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
{
    /// 内部の値を取り出す。
    fn value(self) -> u32;
    /// 剰余の除数を返す。
    fn modulo(self) -> u32;
    /// `self`の`p`乗を返す。
    fn pow(self, p: u64) -> Self;
}
