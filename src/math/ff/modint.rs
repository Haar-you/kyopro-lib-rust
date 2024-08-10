use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub struct ModIntBuilder {
    modulo: u32,
}

impl ModIntBuilder {
    pub fn new(modulo: u32) -> Self {
        Self { modulo }
    }

    pub fn value(&self, value: u32) -> ModInt {
        ModInt::new(value, self.modulo)
    }

    // pub fn frac(&self, numerator: i64, denominator: i64) -> ModInt {
    //     self.value(numerator) * self.value(denominator).inv()
    // }
}

#[derive(Copy, Clone, PartialEq)]
pub struct ModInt {
    value: u32,
    modulo: u32,
}

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
    pub fn value(self) -> u32 {
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
        Self::new_unchecked(
            if a < self.modulo as u64 {
                a as u32
            } else {
                (a % self.modulo as u64) as u32
            },
            self.modulo,
        )
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

    pub fn pow(self, p: u64) -> Self {
        self.pow_internal(p)
    }

    pub fn inv(self) -> Self {
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

// impl Sum for ModInt {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         iter.fold(Self::new_unchecked(0), |a, b| a + b)
//     }
// }

// impl<M: Modulo> Zero for ModInt<M> {
//     type Output = Self;
//     #[inline]
//     fn zero() -> Self::Output {
//         Self::new_unchecked(0)
//     }
// }

// impl<M: Modulo> One for ModInt<M> {
//     type Output = Self;
//     #[inline]
//     fn one() -> Self::Output {
//         Self::new_unchecked(1)
//     }
// }
