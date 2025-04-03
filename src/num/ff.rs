//! 有限体
use std::ops::Neg;

use crate::num::arithmetic::Arithmetic;
pub use crate::num::ops::{Inv, Pow};

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
}

/// 有限体の元
pub trait FFElem:
    Sized + Neg<Output = Self> + PartialEq + Arithmetic + Pow<Output = Self> + Inv<Output = Self>
{
}
