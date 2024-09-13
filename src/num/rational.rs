//! 有理数

use crate::math::gcd_lcm::GcdLcm;
use std::{
    cmp::Ordering,
    fmt,
    fmt::{Debug, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    pub fn new(mut numerator: i64, mut denominator: i64) -> Self {
        if denominator == 0 {
            panic!("denominator must not be 0.");
        }

        if denominator < 0 {
            denominator = -denominator;
            numerator = -numerator;
        }
        let g = numerator.abs().gcd(denominator);
        numerator /= g;
        denominator /= g;

        Self {
            numerator,
            denominator,
        }
    }

    /// 分子を返す
    pub fn numerator(self) -> i64 {
        self.numerator
    }

    /// 分母を返す
    pub fn denominator(self) -> i64 {
        self.denominator
    }

    #[inline]
    fn __add(self, other: Self) -> Self {
        let l = self.denominator.lcm(other.denominator);
        Self::new(
            l / self.denominator * self.numerator + l / other.denominator * other.numerator,
            l,
        )
    }

    #[inline]
    fn __sub(self, other: Self) -> Self {
        let l = self.denominator.lcm(other.denominator);
        Self::new(
            l / self.denominator * self.numerator - l / other.denominator * other.numerator,
            l,
        )
    }

    #[inline]
    fn __mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    #[inline]
    fn __div(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

impl From<Rational> for f64 {
    fn from(from: Rational) -> Self {
        (from.numerator as f64) / (from.denominator as f64)
    }
}

impl From<i64> for Rational {
    fn from(from: i64) -> Self {
        Self {
            numerator: from,
            denominator: 1,
        }
    }
}

macro_rules! impl_arith {
    ($tr:ident, $f:ident, $fi:ident, $tr_a:ident, $f_a:ident, $op:tt) => {
        impl $tr for Rational {
            type Output = Self;
            fn $f(self, other: Self) -> Self {
                self.$fi(other)
            }
        }

        impl $tr_a for Rational {
            fn $f_a(&mut self, other: Self) {
                *self = *self $op other;
            }
        }
    }
}

impl_arith!(Add, add, __add, AddAssign, add_assign, +);
impl_arith!(Sub, sub, __sub, SubAssign, sub_assign, -);
impl_arith!(Mul, mul, __mul, MulAssign, mul_assign, *);
impl_arith!(Div, div, __div, DivAssign, div_assign, /);

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.numerator, self.denominator)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.numerator * other.denominator).cmp(&(other.numerator * self.denominator)))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator * other.denominator).cmp(&(other.numerator * self.denominator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Rational::new(2, 3) + Rational::new(4, 5),
            Rational::new(22, 15)
        );

        assert_eq!(
            Rational::new(3, 4) - Rational::new(2, 3),
            Rational::new(1, 12)
        );

        assert_eq!(
            Rational::new(8, 9) * Rational::new(3, 4),
            Rational::new(2, 3)
        );

        assert_eq!(
            Rational::new(1, 3) / Rational::new(7, 6),
            Rational::new(2, 7)
        );
    }
}
