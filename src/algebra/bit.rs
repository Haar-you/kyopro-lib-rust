//! 論理積・論理和・排他的論理和
use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 論理積を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitAnd<T>(PhantomData<T>);
impl<T> BitAnd<T> {
    /// [`BitAnd`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// 論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitOr<T>(PhantomData<T>);
impl<T> BitOr<T> {
    /// [`BitOr`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

/// 排他的論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitXor<T>(PhantomData<T>);
impl<T> BitXor<T> {
    /// [`BitXor`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! implement {
    ($($t:tt),*) => {
        $(impl_algebra!(BitAnd<$t>; set: $t; op: |_, a: $t, b: $t| a & b;
                        id: |_| !0, |_, &a| a == !0; commu; assoc; idem;);)*
        $(impl_algebra!(BitOr<$t>; set: $t; op: |_, a: $t, b: $t| a | b;
                        id: |_| 0, |_, &a| a == 0; commu; assoc; idem;);)*
        $(impl_algebra!(BitXor<$t>; set: $t; op: |_, a: $t, b: $t| a ^ b;
                        id: |_| 0, |_, &a| a == 0; inv: |_, a| a; commu; assoc;);)*
    };
}

implement!(u8, u16, u32, u64, u128, usize);
