//! コンパイル時にmod Mが決まるModInt

pub mod algebra;

use crate::impl_from;
use crate::impl_one_zero;
use crate::impl_ops;
pub use crate::num::ff::*;
use crate::num::one_zero::*;
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

    #[inline]
    fn modulo(self) -> u32 {
        M
    }

    fn pow(self, mut p: u64) -> Self {
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

impl<const M: u32> ConstModInt<M> {
    /// `ConstModInt<M>`を生成する。
    pub fn new(n: u32) -> Self {
        Self(if n < M { n } else { n % M })
    }

    #[inline]
    fn new_unchecked(value: u32) -> Self {
        Self(value)
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

impl_ops!([const M: u32]; Add for ConstModInt<M>, |x: Self, y: Self| {
    let a = x.0 + y.0;
    Self::new_unchecked(if a < M { a } else { a - M })
});
impl_ops!([const M: u32]; Sub for ConstModInt<M>, |x: Self, y: Self| {
    let a = if x.0 < y.0 {
        x.0 + M - y.0
    } else {
        x.0 - y.0
    };
    Self::new_unchecked(a)
});
impl_ops!([const M: u32]; Mul for ConstModInt<M>, |x: Self, y: Self| {
    let a = x.0 as u64 * y.0 as u64;
    Self::new_unchecked(if a < M as u64 {
        a as u32
    } else {
        (a % M as u64) as u32
    })
});
impl_ops!([const M: u32]; Div for ConstModInt<M>, |x: Self, y: Self| x * y.inv());

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

impl_one_zero!([const M: u32]; ConstModInt<M>; one: Self(1); zero: Self(0););
