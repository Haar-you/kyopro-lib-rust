//! GCD・LCM
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use crate::math::gcd_lcm::*;
use std::marker::PhantomData;

/// GCDモノイド
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GCD<T>(PhantomData<T>);
/// LCMモノイド
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LCM<T>(PhantomData<T>);

impl<T> GCD<T> {
    /// `GCD`を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T> LCM<T> {
    /// `LCM`を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! implement {
    ($($t:ty),*) => {
        $(
            impl_algebra!(GCD<$t>; set: $t; op: |_, a: $t, b: $t| a.gcd(b);
                          id: |_| 0 as $t; commu; assoc; idem;);
            impl_algebra!(LCM<$t>; set: $t; op: |_, a: $t, b: $t| a.lcm(b);
                          id: |_| 1 as $t; commu; assoc; idem;);
        )*
    };
}

implement!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
