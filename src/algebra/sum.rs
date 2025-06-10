//! 加法を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 加法を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Sum<T>(pub T);

macro_rules! implement {
    (signed; $($t:ty),*) => {
        $(impl_algebra!(Sum<$t>; op: |a: Self, b: Self| Self(a.0 + b.0); id: Self(0 as $t); inv: |a: Self| Self(-a.0); commu; assoc;);)*
    };
    (unsigned; $($t:ty),*) => {
        $(impl_algebra!(Sum<$t>; op: |a: Self, b: Self| Self(a.0 + b.0); id: Self(0 as $t); commu; assoc;);)*
    };
}

implement!(signed; i8, i16, i32, i64, i128, isize, f32, f64);
implement!(unsigned; u8, u16, u32, u64, u128, usize);
