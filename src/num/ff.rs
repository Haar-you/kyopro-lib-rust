//! 有限体
pub use crate::num::zz::*;
use std::ops::{Div, DivAssign};

/// 有限体
pub trait FF: ZZ<Element: FFElem> + Clone {
    /// `a/b`を生成する。
    fn frac(&self, a: i64, b: i64) -> Self::Element {
        self.from_i64(a) / self.from_i64(b)
    }
}

/// 有限体の元
pub trait FFElem: ZZElem + Div<Output = Self> + DivAssign {
    /// `self`の乗法の逆元を返す。
    fn inv(self) -> Self {
        self.pow(self.modulo() as u64 - 2)
    }
}
