//! 実行時にmod Mが決まるModInt

pub mod algebra;

use crate::impl_ops;
pub use crate::num::ff::*;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::Neg,
};

/// [`ModInt`]を生成するための構造体。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModIntBuilder {
    modulo: u32,
}

impl ModIntBuilder {
    /// `modulo`を法とする`ModIntBuilder`を生成する。
    pub fn new(modulo: u32) -> Self {
        Self { modulo }
    }
}

impl FF for ModIntBuilder {
    type Element = ModInt;
    fn from_u64(&self, value: u64) -> Self::Element {
        ModInt::new((value % self.modulo as u64) as u32, self.modulo)
    }

    fn from_i64(&self, value: i64) -> Self::Element {
        let value = ((value % self.modulo as i64) + self.modulo as i64) as u32;
        ModInt::new(value, self.modulo)
    }
}

/// `modulo`を法として剰余をとる構造体。
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ModInt {
    value: u32,
    modulo: u32,
}

impl FFElem for ModInt {
    #[inline]
    fn value(self) -> u32 {
        self.value
    }
}

impl ModInt {
    /// `value`を値にもち、`modulo`を法とする`ModInt`を生成する。
    pub fn new(value: u32, modulo: u32) -> Self {
        let value = if value < modulo {
            value
        } else {
            value % modulo
        };
        Self { value, modulo }
    }

    #[inline]
    fn new_unchecked(value: u32, modulo: u32) -> Self {
        Self { value, modulo }
    }

    #[inline]
    fn __add(self, other: Self) -> Self {
        assert_eq!(self.modulo, other.modulo);
        let a = self.value + other.value;
        Self::new_unchecked(
            if a < self.modulo { a } else { a - self.modulo },
            self.modulo,
        )
    }

    #[inline]
    fn __sub(self, other: Self) -> Self {
        assert_eq!(self.modulo, other.modulo);
        let a = if self.value < other.value {
            self.value + self.modulo - other.value
        } else {
            self.value - other.value
        };

        Self::new_unchecked(a, self.modulo)
    }

    #[inline]
    fn __mul(self, other: Self) -> Self {
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
    fn __div(self, other: Self) -> Self {
        self * other.__inv()
    }

    #[inline]
    fn __inv(self) -> Self {
        self.__pow(self.modulo as u64 - 2)
    }

    #[inline]
    fn __pow(self, mut p: u64) -> Self {
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
        self.__pow(p)
    }
}

impl Inv for ModInt {
    type Output = Self;
    fn inv(self) -> Self::Output {
        self.__inv()
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

impl_ops!(Add, ModInt, |x: Self, y| x.__add(y));
impl_ops!(Sub, ModInt, |x: Self, y| x.__sub(y));
impl_ops!(Mul, ModInt, |x: Self, y| x.__mul(y));
impl_ops!(Div, ModInt, |x: Self, y| x.__div(y));

impl_ops!(AddAssign, ModInt, |x: &mut Self, y| *x = *x + y);
impl_ops!(SubAssign, ModInt, |x: &mut Self, y| *x = *x - y);
impl_ops!(MulAssign, ModInt, |x: &mut Self, y| *x = *x * y);
impl_ops!(DivAssign, ModInt, |x: &mut Self, y| *x = *x / y);

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

impl From<ModInt> for u32 {
    fn from(value: ModInt) -> Self {
        value.value
    }
}
