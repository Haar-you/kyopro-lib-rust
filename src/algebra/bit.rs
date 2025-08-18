//! 論理積・論理和・排他的論理和
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 論理積を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitAnd<T>(pub T);

/// 論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitOr<T>(pub T);

/// 排他的論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitXor<T>(pub T);

macro_rules! implement {
    ($($t:tt),*) => {
        $(impl_algebra!(BitAnd<$t>; op: |a: Self, b: Self| Self(a.0 & b.0); id: Self(!0); commu; assoc; idem;);)*
        $(impl_algebra!(BitOr<$t>; op: |a: Self, b: Self| Self(a.0 | b.0); id: Self(0); commu; assoc; idem;);)*
        $(impl_algebra!(BitXor<$t>; op: |a: Self, b: Self| Self(a.0 ^ b.0); id: Self(0); inv: |a| a; commu; assoc;);)*
    };
}

implement!(u8, u16, u32, u64, u128, usize);
