#![allow(clippy::suspicious_arithmetic_impl)]

pub use crate::{
    math::ff_traits::{Frac, Inv, Pow, FF},
    algebra::one_zero::{Zero, One},
};
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    iter::Sum,
    marker::PhantomData,
    num::ParseIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str::FromStr,
};

pub trait Modulo {
    fn value() -> u64;
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct ModInt<M> {
    value: u64,
    phantom: PhantomData<M>,
}

impl<M: Modulo + Copy + PartialEq + Default> FF for ModInt<M> {}

impl<M: Modulo> ModInt<M> {
    pub fn new() -> Self {
        ModInt {
            value: 0,
            phantom: PhantomData,
        }
    }
}

impl<M: Modulo> Pow for ModInt<M> {
    type Output = Self;

    fn pow(self, mut p: u64) -> Self {
        let mut ret = 1;
        let mut a = self.value;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
                ret %= M::value();
            }

            a *= a;
            a %= M::value();

            p >>= 1;
        }

        Self {
            value: ret,
            phantom: PhantomData,
        }
    }
}

impl<M: Modulo> Inv for ModInt<M> {
    type Output = Self;

    fn inv(self) -> Self {
        self.pow(M::value() - 2)
    }
}

impl<M: Modulo> Frac for ModInt<M> {
    type Output = Self;

    fn frac(numerator: i64, denominator: i64) -> Self {
        Self::from(numerator) * Self::from(denominator).inv()
    }
}

impl<M: Modulo> Display for ModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<M: Modulo> Debug for ModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value, M::value())
    }
}

macro_rules! modint_from_int {
    ( $($t:ty),* ) => {
        $(
            impl<M: Modulo> From<$t> for ModInt<M> {
                fn from(from: $t) -> Self {
                    let mut value = ((from % M::value() as $t) + M::value() as $t) as u64;
                    if value >= M::value() {
                        value -= M::value();
                    }

                    ModInt { value, phantom: PhantomData }
                }
            }
        )*
    }
}

modint_from_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl<M> From<ModInt<M>> for u64 {
    fn from(from: ModInt<M>) -> Self {
        from.value
    }
}

impl<M: Modulo> Add for ModInt<M> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: (u64::from(self) + u64::from(other)) % M::value(),
            phantom: PhantomData,
        }
    }
}

impl<M: Modulo + Copy> AddAssign for ModInt<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<M: Modulo> Sub for ModInt<M> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: (u64::from(self) + (M::value() - u64::from(other))) % M::value(),
            phantom: PhantomData,
        }
    }
}

impl<M: Modulo + Copy> SubAssign for ModInt<M> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<M: Modulo> Mul for ModInt<M> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: (u64::from(self) * u64::from(other)) % M::value(),
            phantom: PhantomData,
        }
    }
}

impl<M: Modulo + Copy> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<M: Modulo> Div for ModInt<M> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl<M: Modulo + Copy> DivAssign for ModInt<M> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl<M: Modulo> Neg for ModInt<M> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: (M::value() - u64::from(self)) % M::value(),
            phantom: PhantomData,
        }
    }
}

impl<M: Modulo> FromStr for ModInt<M> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<u64>()?;
        Ok(Self::from(x))
    }
}

impl<M: Modulo> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(0), |a, b| a + b)
    }
}

impl<M: Modulo> Zero for ModInt<M> {
    type Output = Self;
    fn zero() -> Self::Output { Self::from(0) }
}

impl<M: Modulo> One for ModInt<M> {
    type Output = Self;
    fn one() -> Self::Output { Self::from(1) }
}
