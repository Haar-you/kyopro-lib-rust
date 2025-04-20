//! 全順序を実装した`f64`

use crate::impl_from;
use crate::impl_one_zero;
use crate::impl_ops;
use crate::num::one_zero::*;

use std::cmp::Ordering;

/// 全順序を実装した`f64`
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

impl_ops!(Add for Totalf64, |s: Self, rhs: Self| Self(s.0 + rhs.0));
impl_ops!(Sub for Totalf64, |s: Self, rhs: Self| Self(s.0 - rhs.0));
impl_ops!(Mul for Totalf64, |s: Self, rhs: Self| Self(s.0 * rhs.0));
impl_ops!(Div for Totalf64, |s: Self, rhs: Self| Self(s.0 / rhs.0));

impl_ops!(AddAssign for Totalf64, |s: &mut Self, rhs: Self| s.0 += rhs.0);
impl_ops!(SubAssign for Totalf64, |s: &mut Self, rhs: Self| s.0 -= rhs.0);
impl_ops!(MulAssign for Totalf64, |s: &mut Self, rhs: Self| s.0 *= rhs.0);
impl_ops!(DivAssign for Totalf64, |s: &mut Self, rhs: Self| s.0 /= rhs.0);

impl_ops!(Neg for Totalf64, |s: Self| Self(-s.0));

impl_from!(f64 => Totalf64, Self);
impl_from!(f32 => Totalf64, |value| Self(value as f64));
impl_from!(Totalf64 => f64, |value: Totalf64| value.0);

impl_one_zero!(Totalf64; one: Self(1.0); zero: Self(0.0););
