pub use crate::num::const_modint::*;
use crate::num::one_zero::*;

impl<const M: u32> Zero for ConstModInt<M> {
    type Output = Self;
    fn zero() -> Self::Output {
        Self(0)
    }
}

impl<const M: u32> One for ConstModInt<M> {
    type Output = Self;
    fn one() -> Self::Output {
        Self(1)
    }
}
