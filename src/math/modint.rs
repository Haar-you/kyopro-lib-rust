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

    #[inline]
    fn add_internal(self, other: Self) -> Self {
        Self {
            value: (self.value + other.value) % M::value(),
            phantom: PhantomData,
        }
    }

    #[inline]
    fn sub_internal(self, other: Self) -> Self {
        Self {
            value: (self.value + (M::value() - other.value)) % M::value(),
            phantom: PhantomData,
        }
    }

    #[inline]
    fn mul_internal(self, other: Self) -> Self {
        Self {
            value: (self.value * other.value) % M::value(),
            phantom: PhantomData,
        }
    }

    #[inline]
    fn div_internal(self, other: Self) -> Self {
        self * other.inv_internal()
    }

    #[inline]
    fn inv_internal(self) -> Self {
        self.pow_internal(M::value() - 2)
    }

    #[inline]
    fn pow_internal(self, mut p: u64) -> Self {
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

macro_rules! impl_modint_arith {
    ($tr:ident, $f:ident, $fi:ident, $tr_a:ident, $f_a:ident, $op:tt) => {
        impl<M: Modulo> $tr for ModInt<M> {
            type Output = Self;
            fn $f(self, other: Self) -> Self {
                self.$fi(other)
            }
        }

        impl<M: Modulo + Copy> $tr_a for ModInt<M> {
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
        Self {
            value: (M::value() - self.value) % M::value(),
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
