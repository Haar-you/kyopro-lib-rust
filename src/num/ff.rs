//! 有限体
use crate::num::arithmetic::Arithmetic;
use std::ops::Neg;

/// 有限体
#[allow(clippy::wrong_self_convention)]
pub trait FF: Clone {
    /// 有限体の元の型
    type Element: FFElem;
    /// `u64`から生成する。
    fn from_u64(&self, a: u64) -> Self::Element;
    /// `i64`から生成する。
    fn from_i64(&self, a: i64) -> Self::Element;
    /// `a/b`を生成する。
    fn frac(&self, a: i64, b: i64) -> Self::Element {
        self.from_i64(a) / self.from_i64(b)
    }
    fn modulo(&self) -> u32;
}

/// 有限体の元
pub trait FFElem: Sized + Copy + Neg<Output = Self> + PartialEq + Arithmetic {
    /// 内部の値を取り出す。
    fn value(self) -> u32;
    /// 剰余の除数を返す。
    fn modulo(self) -> u32;
    /// `self`の`p`乗を返す。
    fn pow(self, p: u64) -> Self;
    /// `self`の乗法の逆元を返す。
    fn inv(self) -> Self {
        self.pow(self.modulo() as u64 - 2)
    }
}
