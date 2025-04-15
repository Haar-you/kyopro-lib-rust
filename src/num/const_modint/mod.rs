//! コンパイル時にmod Mが決まるModInt

pub mod algebra;
pub mod one_zero;

use crate::impl_from;
use crate::impl_ops;
pub use crate::num::ff::*;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

/// [`ConstModInt<M>`]を生成するための構造体。
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ConstModIntBuilder<const M: u32>;

impl<const M: u32> FF for ConstModIntBuilder<M> {
    type Element = ConstModInt<M>;
    fn from_u64(&self, a: u64) -> Self::Element {
        Self::Element::new_unchecked(if a < M as u64 {
            a as u32
        } else {
            (a % M as u64) as u32
        })
    }
    fn from_i64(&self, value: i64) -> Self::Element {
        let value = ((value % M as i64) + M as i64) as u32;
        Self::Element::new(value)
    }
}

/// `M`で剰余をとる構造体。
#[derive(Copy, Clone, PartialEq, Default)]
pub struct ConstModInt<const M: u32>(u32);

impl<const M: u32> FFElem for ConstModInt<M> {
    #[inline]
    fn value(self) -> u32 {
        self.0
    }
}

impl<const M: u32> ConstModInt<M> {
    /// `ConstModInt<M>`を生成する。
    pub fn new(n: u32) -> Self {
        Self(if n < M { n } else { n % M })
    }

    #[inline]
    fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    fn __add(self, other: Self) -> Self {
        let a = self.0 + other.0;
        Self::new_unchecked(if a < M { a } else { a - M })
    }

    #[inline]
    fn __sub(self, other: Self) -> Self {
        let a = if self.0 < other.0 {
            self.0 + M - other.0
        } else {
            self.0 - other.0
        };

        Self::new_unchecked(a)
    }

    #[inline]
    fn __mul(self, other: Self) -> Self {
        let a = self.0 as u64 * other.0 as u64;
        Self::new_unchecked(if a < M as u64 {
            a as u32
        } else {
            (a % M as u64) as u32
        })
    }

    #[inline]
    fn __div(self, other: Self) -> Self {
        self * other.__inv()
    }

    #[inline]
    fn __inv(self) -> Self {
        self.__pow(M as u64 - 2)
    }

    #[inline]
    fn __pow(self, mut p: u64) -> Self {
        let mut ret: u64 = 1;
        let mut a = self.0 as u64;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
                ret %= M as u64;
            }

            a *= a;
            a %= M as u64;

            p >>= 1;
        }

        Self::new_unchecked(ret as u32)
    }
}

impl<const M: u32> Pow for ConstModInt<M> {
    type Output = Self;
    fn pow(self, p: u64) -> Self {
        self.__pow(p)
    }
}

impl<const M: u32> Inv for ConstModInt<M> {
    type Output = Self;
    fn inv(self) -> Self {
        self.__inv()
    }
}

impl<const M: u32> Display for ConstModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const M: u32> Debug for ConstModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.0, M)
    }
}

impl_ops!([const M: u32]; Add for ConstModInt<M>, |x: Self, y| x.__add(y));
impl_ops!([const M: u32]; Sub for ConstModInt<M>, |x: Self, y| x.__sub(y));
impl_ops!([const M: u32]; Mul for ConstModInt<M>, |x: Self, y| x.__mul(y));
impl_ops!([const M: u32]; Div for ConstModInt<M>, |x: Self, y| x.__div(y));

impl_ops!([const M: u32]; AddAssign for ConstModInt<M>, |x: &mut Self, y| *x = *x + y);
impl_ops!([const M: u32]; SubAssign for ConstModInt<M>, |x: &mut Self, y| *x = *x - y);
impl_ops!([const M: u32]; MulAssign for ConstModInt<M>, |x: &mut Self, y| *x = *x * y);
impl_ops!([const M: u32]; DivAssign for ConstModInt<M>, |x: &mut Self, y| *x = *x / y);

impl_ops!([const M: u32]; Neg for ConstModInt<M>, |x: Self| Self::new_unchecked(if x.0 == 0 { 0 } else { M - x.0 }));

impl_from!([const M: u32]; ConstModInt<M> => u32, |value: ConstModInt<M>| value.0);

impl_from!([const M: u32]; usize => ConstModInt<M>, |value| ConstModIntBuilder.from_u64(value as u64));
impl_from!([const M: u32]; u64 => ConstModInt<M>, |value| ConstModIntBuilder.from_u64(value));
impl_from!([const M: u32]; u32 => ConstModInt<M>, |value| ConstModIntBuilder.from_u64(value as u64));

impl_from!([const M: u32]; isize => ConstModInt<M>, |value| ConstModIntBuilder.from_i64(value as i64));
impl_from!([const M: u32]; i64 => ConstModInt<M>, |value| ConstModIntBuilder.from_i64(value));
impl_from!([const M: u32]; i32 => ConstModInt<M>, |value| ConstModIntBuilder.from_i64(value as i64));
