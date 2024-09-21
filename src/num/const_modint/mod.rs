pub mod algebra;
pub mod one_zero;

pub use crate::num::ff::*;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct ConstModIntBuilder<const M: u32>;

impl<const M: u32> FF for ConstModIntBuilder<M> {
    type Output = ConstModInt<M>;
    fn from_u64(&self, a: u64) -> Self::Output {
        Self::Output::new_unchecked(if a < M as u64 {
            a as u32
        } else {
            (a % M as u64) as u32
        })
    }
    fn from_i64(&self, value: i64) -> Self::Output {
        let value = ((value % M as i64) + M as i64) as u32;
        Self::Output::new(value)
    }
    fn frac(&self, numerator: i64, denominator: i64) -> Self::Output {
        self.from_i64(numerator) * self.from_i64(denominator).inv()
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct ConstModInt<const M: u32>(u32);

impl<const M: u32> FFElem for ConstModInt<M> {}

impl<const M: u32> ConstModInt<M> {
    pub fn new(n: u32) -> Self {
        Self(if n < M { n } else { n % M })
    }

    pub fn to_u32(self) -> u32 {
        self.0
    }

    #[inline]
    fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    #[inline]
    fn add_internal(self, other: Self) -> Self {
        let a = self.0 + other.0;
        Self::new_unchecked(if a < M { a } else { a - M })
    }

    #[inline]
    fn sub_internal(self, other: Self) -> Self {
        let a = if self.0 < other.0 {
            self.0 + M - other.0
        } else {
            self.0 - other.0
        };

        Self::new_unchecked(a)
    }

    #[inline]
    fn mul_internal(self, other: Self) -> Self {
        let a = self.0 as u64 * other.0 as u64;
        Self::new_unchecked(if a < M as u64 {
            a as u32
        } else {
            (a % M as u64) as u32
        })
    }

    #[inline]
    fn div_internal(self, other: Self) -> Self {
        self * other.inv_internal()
    }

    #[inline]
    fn inv_internal(self) -> Self {
        self.pow_internal(M as u64 - 2)
    }

    #[inline]
    fn pow_internal(self, mut p: u64) -> Self {
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
        self.pow_internal(p)
    }
}

impl<const M: u32> Inv for ConstModInt<M> {
    type Output = Self;

    fn inv(self) -> Self {
        self.inv_internal()
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

impl<const M: u32> From<ConstModInt<M>> for u32 {
    fn from(from: ConstModInt<M>) -> Self {
        from.0
    }
}

macro_rules! impl_modint_arith {
    ($tr:ident, $f:ident, $fi:ident, $tr_a:ident, $f_a:ident, $op:tt) => {
        impl<const M: u32> $tr for ConstModInt<M> {
            type Output = Self;
            #[inline]
            fn $f(self, other: Self) -> Self {
                self.$fi(other)
            }
        }

        impl<const M: u32> $tr_a for ConstModInt<M> {
            #[inline]
            fn $f_a(&mut self, other: Self) {
                *self = *self $op other;
            }
        }
    }
}

impl_modint_arith!(Add, add, add_internal, AddAssign, add_assign, +);
impl_modint_arith!(Sub, sub, sub_internal, SubAssign, sub_assign, -);
impl_modint_arith!(Mul, mul, mul_internal, MulAssign, mul_assign, *);
impl_modint_arith!(Div, div, div_internal, DivAssign, div_assign, /);

impl<const M: u32> Neg for ConstModInt<M> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_unchecked(if self.0 == 0 { 0 } else { M - self.0 })
    }
}
