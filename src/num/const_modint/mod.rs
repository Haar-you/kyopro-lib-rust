//! コンパイル時にmod Mが決まるModInt

pub mod algebra;

use crate::impl_from;
use crate::impl_one_zero;
use crate::impl_ops;
use crate::math::prime_mod::PrimeMod;
pub use crate::num::ff::*;
use crate::num::one_zero::*;
use std::marker::PhantomData;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

const B: u32 = 32;
const R: u64 = 1 << B;
const MASK: u64 = R - 1;

struct Calc<P: PrimeMod>(PhantomData<P>);
impl<P: PrimeMod> Calc<P> {
    const R2: u64 = R % P::PRIME_NUM as u64 * R % P::PRIME_NUM as u64;
    const M: u64 = {
        assert!(P::PRIME_NUM % 2 == 1);

        let mut ret = 0;
        let mut r = R;
        let mut i = 1;
        let mut t = 0;
        while r > 1 {
            if t % 2 == 0 {
                t += P::PRIME_NUM;
                ret += i;
            }
            t >>= 1;
            r >>= 1;
            i <<= 1;
        }
        ret
    };

    const fn reduce(value: u64) -> u32 {
        let mut ret =
            (((((value & MASK) * Self::M) & MASK) * P::PRIME_NUM as u64 + value) >> B) as u32;
        if ret >= P::PRIME_NUM {
            ret -= P::PRIME_NUM;
        }
        ret
    }

    const fn make(value: u32) -> u32 {
        Self::reduce(value as u64 * Self::R2)
    }
}

/// [`ConstModInt<P>`]を生成するための構造体。
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ConstModIntBuilder<P: PrimeMod>(PhantomData<P>);

impl<P: PrimeMod> ConstModIntBuilder<P> {
    /// `ConstModIntBuilder`を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<P: PrimeMod> FF for ConstModIntBuilder<P> {
    type Element = ConstModInt<P>;
    fn from_u64(&self, mut value: u64) -> Self::Element {
        if value >= P::PRIME_NUM as u64 {
            value %= P::PRIME_NUM as u64;
        }

        let value = Calc::<P>::make(value as u32);
        ConstModInt(value, PhantomData)
    }
    fn from_i64(&self, mut value: i64) -> Self::Element {
        value %= P::PRIME_NUM as i64;
        if value < 0 {
            value += P::PRIME_NUM as i64;
        }

        let value = Calc::<P>::make(value as u32);
        ConstModInt(value, PhantomData)
    }
    fn modulo(&self) -> u32 {
        P::PRIME_NUM
    }
}

/// 奇素数`P`で剰余をとる構造体。
#[derive(Copy, Clone, PartialEq, Default)]
pub struct ConstModInt<P: PrimeMod>(u32, PhantomData<P>);

impl<P: PrimeMod> FFElem for ConstModInt<P> {
    #[inline]
    fn value(self) -> u32 {
        Calc::<P>::reduce(self.0 as u64)
    }

    #[inline]
    fn modulo(self) -> u32 {
        P::PRIME_NUM
    }

    fn pow(self, mut p: u64) -> Self {
        let mut ret = Self(Calc::<P>::make(1), PhantomData);
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

impl<P: PrimeMod> ConstModInt<P> {
    /// `ConstModInt<P>`を生成する。
    pub fn new(n: u32) -> Self {
        let value = if n < P::PRIME_NUM {
            n
        } else {
            n % P::PRIME_NUM
        };
        Self(Calc::<P>::make(value), PhantomData)
    }
}

impl<P: PrimeMod> Display for ConstModInt<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl<P: PrimeMod> Debug for ConstModInt<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value(), P::PRIME_NUM)
    }
}

impl_ops!([P: PrimeMod]; Add for ConstModInt<P>, |x: Self, y: Self| {
    let mut a = x.0 + y.0;
    if a >= P::PRIME_NUM {
        a -= P::PRIME_NUM;
    }
    Self(a, PhantomData)
});
impl_ops!([P: PrimeMod]; Sub for ConstModInt<P>, |x: Self, y: Self| {
    let a = if x.0 < y.0 {
        x.0 + P::PRIME_NUM - y.0
    } else {
        x.0 - y.0
    };
    Self(a, PhantomData)
});
impl_ops!([P: PrimeMod]; Mul for ConstModInt<P>, |x: Self, y: Self| {
    Self(Calc::<P>::reduce(x.0 as u64 * y.0 as u64), PhantomData)
});
impl_ops!([P: PrimeMod]; Div for ConstModInt<P>, |x: Self, y: Self| x * y.inv());

impl_ops!([P: PrimeMod]; AddAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x + y);
impl_ops!([P: PrimeMod]; SubAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x - y);
impl_ops!([P: PrimeMod]; MulAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x * y);
impl_ops!([P: PrimeMod]; DivAssign for ConstModInt<P>, |x: &mut Self, y| *x = *x / y);

impl_ops!([P: PrimeMod]; Neg for ConstModInt<P>, |x: Self| Self(0, PhantomData) - x);

impl_from!([P: PrimeMod]; ConstModInt<P> => u32, |value: ConstModInt<P>| value.value());

impl_from!([P: PrimeMod]; usize => ConstModInt<P>, |value| ConstModIntBuilder::new().from_u64(value as u64));
impl_from!([P: PrimeMod]; u64 => ConstModInt<P>, |value| ConstModIntBuilder::new().from_u64(value));
impl_from!([P: PrimeMod]; u32 => ConstModInt<P>, |value| ConstModIntBuilder::new().from_u64(value as u64));

impl_from!([P: PrimeMod]; isize => ConstModInt<P>, |value| ConstModIntBuilder::new().from_i64(value as i64));
impl_from!([P: PrimeMod]; i64 => ConstModInt<P>, |value| ConstModIntBuilder::new().from_i64(value));
impl_from!([P: PrimeMod]; i32 => ConstModInt<P>, |value| ConstModIntBuilder::new().from_i64(value as i64));

impl_one_zero!([P: PrimeMod]; ConstModInt<P>; one: Self::new(1); zero: Self(0, PhantomData););
