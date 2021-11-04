pub use crate::{
    algebra::one_zero::{One, Zero},
    math::ff::traits::{Frac, Inv, Pow, FF},
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
    fn value() -> u32;
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct ModInt<M> {
    value: u32,
    phantom: PhantomData<M>,
}

impl<M: Modulo + Copy + PartialEq + Default> FF for ModInt<M> {}

impl<M: Modulo> ModInt<M> {
    pub fn new(n: u32) -> Self {
        Self {
            value: if n < M::value() { n } else { n % M::value() },
            phantom: PhantomData,
        }
    }

    #[inline]
    fn new_unchecked(value: u32) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn add_internal(self, other: Self) -> Self {
        let a = self.value + other.value;
        Self::new_unchecked(if a < M::value() { a } else { a - M::value() })
    }

    #[inline]
    fn sub_internal(self, other: Self) -> Self {
        let a = if self.value < other.value {
            self.value + M::value() - other.value
        } else {
            self.value - other.value
        };

        Self::new_unchecked(a)
    }

    #[inline]
    fn mul_internal(self, other: Self) -> Self {
        let a = self.value as u64 * other.value as u64;
        Self::new_unchecked(if a < M::value() as u64 {
            a as u32
        } else {
            (a % M::value() as u64) as u32
        })
    }

    #[inline]
    fn div_internal(self, other: Self) -> Self {
        self * other.inv_internal()
    }

    #[inline]
    fn inv_internal(self) -> Self {
        self.pow_internal(M::value() as u64 - 2)
    }

    #[inline]
    fn pow_internal(self, mut p: u64) -> Self {
        let mut ret: u64 = 1;
        let mut a = self.value as u64;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
                ret %= M::value() as u64;
            }

            a *= a;
            a %= M::value() as u64;

            p >>= 1;
        }

        Self::new_unchecked(ret as u32)
    }
}

impl<M: Modulo> Pow for ModInt<M> {
    type Output = Self;

    fn pow(self, p: u64) -> Self {
        self.pow_internal(p)
    }
}

impl<M: Modulo> Inv for ModInt<M> {
    type Output = Self;

    fn inv(self) -> Self {
        self.inv_internal()
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
                    let mut value = ((from % M::value() as $t) + M::value() as $t) as u32;
                    if value >= M::value() {
                        value -= M::value();
                    }

                    Self::new_unchecked(value)
                }
            }
        )*
    }
}

modint_from_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl<M> From<ModInt<M>> for u32 {
    fn from(from: ModInt<M>) -> Self {
        from.value
    }
}

macro_rules! impl_modint_arith {
    ($tr:ident, $f:ident, $fi:ident, $tr_a:ident, $f_a:ident, $op:tt) => {
        impl<M: Modulo> $tr for ModInt<M> {
            type Output = Self;
            #[inline]
            fn $f(self, other: Self) -> Self {
                self.$fi(other)
            }
        }

        impl<M: Modulo + Copy> $tr_a for ModInt<M> {
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

impl<M: Modulo> Neg for ModInt<M> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new_unchecked(if self.value == 0 {
            0
        } else {
            M::value() - self.value
        })
    }
}

impl<M: Modulo> FromStr for ModInt<M> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<u32>()?;
        Ok(Self::from(x))
    }
}

impl<M: Modulo> Sum for ModInt<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new_unchecked(0), |a, b| a + b)
    }
}

impl<M: Modulo> Zero for ModInt<M> {
    type Output = Self;
    #[inline]
    fn zero() -> Self::Output {
        Self::new_unchecked(0)
    }
}

impl<M: Modulo> One for ModInt<M> {
    type Output = Self;
    #[inline]
    fn one() -> Self::Output {
        Self::new_unchecked(1)
    }
}
