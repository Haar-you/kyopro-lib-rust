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
        assert!(modulo > 0);
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
    fn modulo(&self) -> u32 {
        self.modulo
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

    #[inline]
    fn modulo(self) -> u32 {
        self.modulo
    }

    fn pow(self, mut p: u64) -> Self {
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

impl ModInt {
    /// `value`を値にもち、`modulo`を法とする`ModInt`を生成する。
    pub fn new(value: u32, modulo: u32) -> Self {
        assert!(modulo > 0);
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

impl_ops!(Add for ModInt, |x: Self, y: Self| {
    assert_eq!(x.modulo, y.modulo);
    let a = x.value + y.value;
    Self::new_unchecked(
        if a < x.modulo { a } else { a - x.modulo },
        x.modulo,
    )
});
impl_ops!(Sub for ModInt, |x: Self, y: Self| {
    assert_eq!(x.modulo, y.modulo);
    let a = if x.value < y.value {
        x.value + x.modulo - y.value
    } else {
        x.value - y.value
    };
    Self::new_unchecked(a, x.modulo)
});
impl_ops!(Mul for ModInt, |x: Self, y: Self| {
    assert_eq!(x.modulo, y.modulo);
    let a = x.value as u64 * y.value as u64;
    let value = if a < x.modulo as u64 {
        a as u32
    } else {
        (a % x.modulo as u64) as u32
    };

    Self::new_unchecked(value, x.modulo)
});
impl_ops!(Div for ModInt, |x: Self, y: Self| x * y.inv());

impl_ops!(AddAssign for ModInt, |x: &mut Self, y| *x = *x + y);
impl_ops!(SubAssign for ModInt, |x: &mut Self, y| *x = *x - y);
impl_ops!(MulAssign for ModInt, |x: &mut Self, y| *x = *x * y);
impl_ops!(DivAssign for ModInt, |x: &mut Self, y| *x = *x / y);

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
