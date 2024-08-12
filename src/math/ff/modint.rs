pub use crate::math::ff::traits::*;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModIntBuilder {
    modulo: u32,
}

impl ModIntBuilder {
    pub fn new(modulo: u32) -> Self {
        Self { modulo }
    }
}

impl FF for ModIntBuilder {
    type Output = ModInt;
    fn from_u64(&self, value: u64) -> Self::Output {
        ModInt::new((value % self.modulo as u64) as u32, self.modulo)
    }

    fn from_i64(&self, value: i64) -> Self::Output {
        let value = ((value % self.modulo as i64) + self.modulo as i64) as u32;
        ModInt::new(value, self.modulo)
    }

    fn frac(&self, numerator: i64, denominator: i64) -> Self::Output {
        self.from_i64(numerator) * self.from_i64(denominator).inv()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ModInt {
    value: u32,
    modulo: u32,
}

impl FFElem for ModInt {}

impl ModInt {
    pub fn new(value: u32, modulo: u32) -> Self {
        Self {
            value: if value < modulo {
                value
            } else {
                value % modulo
            },
            modulo,
        }
    }

    #[inline]
    pub fn to_u32(self) -> u32 {
        self.value
    }

    #[inline]
    fn new_unchecked(value: u32, modulo: u32) -> Self {
        Self { value, modulo }
    }

    #[inline]
    fn add_internal(self, other: Self) -> Self {
        assert_eq!(self.modulo, other.modulo);
        let a = self.value + other.value;
        Self::new_unchecked(
            if a < self.modulo { a } else { a - self.modulo },
            self.modulo,
        )
    }

    #[inline]
    fn sub_internal(self, other: Self) -> Self {
        assert_eq!(self.modulo, other.modulo);
        let a = if self.value < other.value {
            self.value + self.modulo - other.value
        } else {
            self.value - other.value
        };

        Self::new_unchecked(a, self.modulo)
    }

    #[inline]
    fn mul_internal(self, other: Self) -> Self {
        assert_eq!(self.modulo, other.modulo);
        let a = self.value as u64 * other.value as u64;
        let value = if a < self.modulo as u64 {
            a as u32
        } else {
            (a % self.modulo as u64) as u32
        };

        Self::new_unchecked(value, self.modulo)
    }

    #[inline]
    fn div_internal(self, other: Self) -> Self {
        self * other.inv_internal()
    }

    #[inline]
    fn inv_internal(self) -> Self {
        self.pow_internal(self.modulo as u64 - 2)
    }

    #[inline]
    fn pow_internal(self, mut p: u64) -> Self {
        let mut ret: u64 = 1;
        let mut a = self.value as u64;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
                ret %= self.modulo as u64;
            }

            a *= a;
            a %= self.modulo as u64;

            p >>= 1;
        }

        Self::new_unchecked(ret as u32, self.modulo)
    }
}

impl Pow for ModInt {
    type Output = Self;
    fn pow(self, p: u64) -> Self::Output {
        self.pow_internal(p)
    }
}

impl Inv for ModInt {
    type Output = Self;
    fn inv(self) -> Self::Output {
        self.inv_internal()
    }
}

impl Display for ModInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Debug for ModInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value, self.modulo)
    }
}

macro_rules! impl_modint_arith {
    ($tr:ident, $f:ident, $fi:ident, $tr_a:ident, $f_a:ident, $op:tt) => {
        impl $tr for ModInt {
            type Output = Self;
            #[inline]
            fn $f(self, other: Self) -> Self {
                self.$fi(other)
            }
        }

        impl $tr_a for ModInt {
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

impl Neg for ModInt {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_unchecked(
            if self.value == 0 {
                0
            } else {
                self.modulo - self.value
            },
            self.modulo,
        )
    }
}
