#![allow(clippy::suspicious_arithmetic_impl)]

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

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let l = self.denominator.lcm(other.denominator);
        Self::new(
            l / self.denominator * self.numerator + l / other.denominator * other.numerator,
            l,
        )
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Rational) {
        *self = *self + other;
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let l = self.denominator.lcm(other.denominator);
        Self::new(
            l / self.denominator * self.numerator - l / other.denominator * other.numerator,
            l,
        )
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, other: Rational) {
        *self = *self - other;
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
