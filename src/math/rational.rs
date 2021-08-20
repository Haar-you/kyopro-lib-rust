use crate::math::gcd_lcm::GcdLcm;

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

impl std::fmt::Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::ops::Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let l = self.denominator.lcm(other.denominator);
        Self::new(
            l / self.denominator * self.numerator + l / other.denominator * other.numerator,
            l,
        )
    }
}

impl std::ops::AddAssign for Rational {
    fn add_assign(&mut self, other: Rational) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let l = self.denominator.lcm(other.denominator);
        Self::new(
            l / self.denominator * self.numerator - l / other.denominator * other.numerator,
            l,
        )
    }
}

impl std::ops::SubAssign for Rational {
    fn sub_assign(&mut self, other: Rational) {
        *self = *self - other;
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl std::ops::MulAssign for Rational {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl std::ops::Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl std::ops::DivAssign for Rational {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl std::ops::Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.numerator, self.denominator)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.numerator * other.denominator).cmp(&(other.numerator * self.denominator)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
