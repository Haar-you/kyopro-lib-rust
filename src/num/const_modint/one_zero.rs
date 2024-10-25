//! [`One`]と[`Zero`]を実装する。

pub use crate::num::const_modint::*;
use crate::num::one_zero::*;

impl<const M: u32> Zero for ConstModInt<M> {
    fn zero() -> Self {
        Self(0)
    }
}

impl<const M: u32> One for ConstModInt<M> {
    fn one() -> Self {
        Self(1)
    }
}
