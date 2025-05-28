//! 論理積を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 論理積を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitAnd<T>(pub T);

macro_rules! implement {
    ($($t:tt),*) => {
        $(impl_algebra!(BitAnd<$t>; op: |a: Self, b: Self| Self(a.0 & b.0); id: Self(!0); commu; assoc; idem;);)*
    };
}

implement!(u8, u16, u32, u64, u128, usize);
