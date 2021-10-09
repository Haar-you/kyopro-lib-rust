#![allow(clippy::suspicious_arithmetic_impl)]

pub use crate::{
    math::ff_traits::{Frac, Inv, Pow, FF},
    misc::generics_int::GenericsInt,
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

#[derive(Copy, Clone, PartialEq, Default)]
pub struct ModInt<G> {
    value: u64,
    phantom: PhantomData<G>,
}

impl<G: GenericsInt<Output = u64> + Copy + PartialEq + Default> FF for ModInt<G> {}

impl<G: GenericsInt<Output = u64>> ModInt<G> {
    pub fn new() -> Self {
        ModInt {
            value: 0,
            phantom: PhantomData,
        }
    }
}

impl<G: GenericsInt<Output = u64>> Pow for ModInt<G> {
    type Output = Self;

    fn pow(self, mut p: u64) -> Self {
        let mut ret = 1;
        let mut a = self.value;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
                ret %= G::value();
            }

            a *= a;
            a %= G::value();

            p >>= 1;
        }

        Self {
            value: ret,
            phantom: PhantomData,
        }
    }
}

impl<G: GenericsInt<Output = u64>> Inv for ModInt<G> {
    type Output = Self;

    fn inv(self) -> Self {
        self.pow(G::value() - 2)
    }
}

impl<G: GenericsInt<Output = u64>> Frac for ModInt<G> {
    type Output = Self;

    fn frac(numerator: i64, denominator: i64) -> Self {
        Self::from(numerator) * Self::from(denominator).inv()
    }
}

impl<G> Display for ModInt<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<G: GenericsInt<Output = u64>> Debug for ModInt<G> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} (mod {})", self.value, G::value())
    }
}

macro_rules! modint_from_int {
    ( $($t:ty),* ) => {
        $(
            impl<G: GenericsInt<Output = u64>> From<$t> for ModInt<G> {
                fn from(from: $t) -> Self {
                    let mut value = ((from % G::value() as $t) + G::value() as $t) as u64;
                    if value >= G::value() {
                        value -= G::value();
                    }

                    ModInt { value, phantom: PhantomData }
                }
            }
        )*
    }
}

modint_from_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl<G> From<ModInt<G>> for u64 {
    fn from(from: ModInt<G>) -> Self {
        from.value
    }
}

impl<G: GenericsInt<Output = u64>> Add for ModInt<G> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: (u64::from(self) + u64::from(other)) % G::value(),
            phantom: PhantomData,
        }
    }
}

impl<G: GenericsInt<Output = u64> + Copy> AddAssign for ModInt<G> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<G: GenericsInt<Output = u64>> Sub for ModInt<G> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: (u64::from(self) + (G::value() - u64::from(other))) % G::value(),
            phantom: PhantomData,
        }
    }
}

impl<G: GenericsInt<Output = u64> + Copy> SubAssign for ModInt<G> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<G: GenericsInt<Output = u64>> Mul for ModInt<G> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: (u64::from(self) * u64::from(other)) % G::value(),
            phantom: PhantomData,
        }
    }
}

impl<G: GenericsInt<Output = u64> + Copy> MulAssign for ModInt<G> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<G: GenericsInt<Output = u64>> Div for ModInt<G> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl<G: GenericsInt<Output = u64> + Copy> DivAssign for ModInt<G> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl<G: GenericsInt<Output = u64>> Neg for ModInt<G> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: (G::value() - u64::from(self)) % G::value(),
            phantom: PhantomData,
        }
    }
}

impl<G: GenericsInt<Output = u64>> FromStr for ModInt<G> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<u64>()?;
        Ok(Self::from(x))
    }
}

impl<G: GenericsInt<Output = u64>> Sum for ModInt<G> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::from(0), |a, b| a + b)
    }
}

impl<G: GenericsInt<Output = u64>> Zero for ModInt<G> {
    type Output = Self;
    fn zero() -> Self::Output { Self::from(0) }
}

impl<G: GenericsInt<Output = u64>> One for ModInt<G> {
    type Output = Self;
    fn one() -> Self::Output { Self::from(1) }
}
