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

const B: u32 = 32;
const R: u64 = 1 << B;
const MASK: u64 = R - 1;

struct Calc<const P: u32>;
impl<const P: u32> Calc<P> {
    const R2: u64 = R % P as u64 * R % P as u64;
    const M: u64 = {
        assert!(P % 2 == 1);

        let mut ret = 0;
        let mut r = R;
        let mut i = 1;
        let mut t = 0;
        while r > 1 {
            if t % 2 == 0 {
                t += P;
                ret += i;
            }
            t >>= 1;
            r >>= 1;
            i <<= 1;
        }
        ret
    };

    const fn reduce(value: u64) -> u32 {
        let mut ret = (((((value & MASK) * Self::M) & MASK) * P as u64 + value) >> B) as u32;
        if ret >= P {
            ret -= P;
        }
        ret
    }

    const fn make(value: u32) -> u32 {
        Self::reduce(value as u64 * Self::R2)
    }
}

/// [`ConstModInt<P>`]を生成するための構造体。
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ConstModIntBuilder<const P: u32>;

impl<const P: u32> FF for ConstModIntBuilder<P> {
    type Element = ConstModInt<P>;
    fn from_u64(&self, mut value: u64) -> Self::Element {
        if value >= P as u64 {
            value %= P as u64;
        }

        let value = Calc::<P>::make(value as u32);
        ConstModInt(value)
    }
    fn from_i64(&self, mut value: i64) -> Self::Element {
        value %= P as i64;
        if value < 0 {
            value += P as i64;
        }

        let value = Calc::<P>::make(value as u32);
        ConstModInt(value)
    }
    fn modulo(&self) -> u32 {
        P
    }
}

/// 奇素数`P`で剰余をとる構造体。
#[derive(Copy, Clone, PartialEq, Default)]
pub struct ConstModInt<const P: u32>(u32);

impl<const P: u32> FFElem for ConstModInt<P> {
    #[inline]
    fn value(self) -> u32 {
        Calc::<P>::reduce(self.0 as u64)
    }

    #[inline]
    fn modulo(self) -> u32 {
        P
    }

    fn pow(self, mut p: u64) -> Self {
        let mut ret = Self(Calc::<P>::make(1));
        let mut a = self;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
            }

            a *= a;
            p >>= 1;
        }

        ret
    }
}

impl<const P: u32> ConstModInt<P> {
    /// `ConstModInt<P>`を生成する。
    pub fn new(n: u32) -> Self {
        let value = if n < P { n } else { n % P };
        Self(Calc::<P>::make(value))
    }
}

impl<const P: u32> Display for ConstModInt<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<const P: u32> Debug for ConstModInt<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value(), P)
    }
}

impl_ops!([const P: u32]; Add for ConstModInt<P>, |x: Self, y: Self| {
    let mut a = x.0 + y.0;
    if a >= P{
        a -= P;
    }
    Self(a)
});
impl_ops!([const P: u32]; Sub for ConstModInt<P>, |x: Self, y: Self| {
    let a = if x.0 < y.0 {
        x.0 + P - y.0
    } else {
        x.0 - y.0
    };
    Self(a)
});
impl_ops!([const P: u32]; Mul for ConstModInt<P>, |x: Self, y: Self| {
    Self(Calc::<P>::reduce(x.0 as u64 * y.0 as u64))
});
impl_ops!([const P: u32]; Div for ConstModInt<P>, |x: Self, y: Self| x * y.inv());

impl_ops!([const P: u32]; AddAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x + y);
impl_ops!([const P: u32]; SubAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x - y);
impl_ops!([const P: u32]; MulAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x * y);
impl_ops!([const P: u32]; DivAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x / y);

impl_ops!([const P: u32]; Neg for ConstModInt<P>, |x: Self| Self(0) - x);

impl_from!([const P: u32]; ConstModInt<P> => u32, |value: ConstModInt<P>| value.value());

impl_from!([const P: u32]; usize => ConstModInt<P>, |value| ConstModIntBuilder.from_u64(value as u64));
impl_from!([const P: u32]; u64 => ConstModInt<P>, |value| ConstModIntBuilder.from_u64(value));
impl_from!([const P: u32]; u32 => ConstModInt<P>, |value| ConstModIntBuilder.from_u64(value as u64));

impl_from!([const P: u32]; isize => ConstModInt<P>, |value| ConstModIntBuilder.from_i64(value as i64));
impl_from!([const P: u32]; i64 => ConstModInt<P>, |value| ConstModIntBuilder.from_i64(value));
impl_from!([const P: u32]; i32 => ConstModInt<P>, |value| ConstModIntBuilder.from_i64(value as i64));

impl_one_zero!([const P: u32]; ConstModInt<P>; one: Self::new(1); zero: Self(0););
