//! 排他的論理和を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 排他的論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitXor<T>(pub T);

macro_rules! implement {
    ($($t:tt),*) => {
        $(impl_algebra!(BitXor<$t>; op: |a: Self, b: Self| Self(a.0 ^ b.0); id: Self(0); inv: |a| a; commu; assoc;);)*
    };
}

implement!(u8, u16, u32, u64, u128, usize);
