//! 有理数

use crate::math::gcd_lcm::GcdLcm;
use crate::{impl_from, impl_ops};
use std::{
    cmp::Ordering,
    fmt,
    fmt::{Debug, Formatter},
};

/// 有理数
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    /// `numerator / denominator`を表す有理数を生成する
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

impl_from!(Rational => f64, |value: Rational| (value.numerator as f64) / (value.denominator as f64));
impl_from!(i64 => Rational, |value| Self { numerator: value, denominator: 1 });

impl_ops!(Add, Rational, |s: Self, rhs| s.__add(rhs));
impl_ops!(Sub, Rational, |s: Self, rhs| s.__sub(rhs));
impl_ops!(Mul, Rational, |s: Self, rhs| s.__mul(rhs));
impl_ops!(Div, Rational, |s: Self, rhs| s.__div(rhs));

impl_ops!(AddAssign, Rational, |s: &mut Self, rhs| *s = *s + rhs);
impl_ops!(SubAssign, Rational, |s: &mut Self, rhs| *s = *s - rhs);
impl_ops!(MulAssign, Rational, |s: &mut Self, rhs| *s = *s * rhs);
impl_ops!(DivAssign, Rational, |s: &mut Self, rhs| *s = *s / rhs);

impl_ops!(Neg, Rational, |s: Self| Self::new(
    -s.numerator,
    s.denominator
));

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
