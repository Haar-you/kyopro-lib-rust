pub mod one_zero;

use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

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

macro_rules! impl_ops {
    ($tr:tt, $f:tt, $ops:tt) => {
        impl $tr for Totalf64 {
            type Output = Self;
            fn $f(self, rhs: Self) -> Self::Output {
                Self(self.0 $ops rhs.0)
            }
        }
    };
}

impl_ops!(Add, add, +);
impl_ops!(Sub, sub, -);
impl_ops!(Mul, mul, *);
impl_ops!(Div, div, /);

macro_rules! impl_ops_assign {
    ($tr:tt, $f:tt, $ops:tt) => {
        impl $tr for Totalf64 {
            fn $f(&mut self, rhs: Self) {
                self.0 $ops rhs.0;
            }
        }
    };
}

impl_ops_assign!(AddAssign, add_assign, +=);
impl_ops_assign!(SubAssign, sub_assign, -=);
impl_ops_assign!(MulAssign, mul_assign, *=);
impl_ops_assign!(DivAssign, div_assign, /=);

impl Neg for Totalf64 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
