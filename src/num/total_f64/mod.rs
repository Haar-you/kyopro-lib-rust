pub mod one_zero;

use crate::impl_ops;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Totalf64(pub f64);

impl PartialOrd for Totalf64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Totalf64 {}
impl Ord for Totalf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl_ops!(Add, Totalf64, |s: Self, rhs: Self| Self(s.0 + rhs.0));
impl_ops!(Sub, Totalf64, |s: Self, rhs: Self| Self(s.0 - rhs.0));
impl_ops!(Mul, Totalf64, |s: Self, rhs: Self| Self(s.0 * rhs.0));
impl_ops!(Div, Totalf64, |s: Self, rhs: Self| Self(s.0 / rhs.0));

impl_ops!(AddAssign, Totalf64, |s: &mut Self, rhs: Self| s.0 += rhs.0);
impl_ops!(SubAssign, Totalf64, |s: &mut Self, rhs: Self| s.0 -= rhs.0);
impl_ops!(MulAssign, Totalf64, |s: &mut Self, rhs: Self| s.0 *= rhs.0);
impl_ops!(DivAssign, Totalf64, |s: &mut Self, rhs: Self| s.0 /= rhs.0);

impl_ops!(Neg, Totalf64, |s: Self| Self(-s.0));
