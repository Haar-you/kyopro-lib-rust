//! 最大値を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 最大値を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Max<T>(pub T);

macro_rules! implement {
    ($($t:tt),*) => {
        $(impl_algebra!(Max<$t>; op: |a: Self, b: Self| Self(a.0.max(b.0)); id: Self($t::MIN); commu; assoc; idem;);)*
    };
}

implement!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
