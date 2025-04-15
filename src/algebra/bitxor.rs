//! 排他的論理和を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 排他的論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitXor<T>(PhantomData<T>);
impl<T> BitXor<T> {
    /// `BitXor<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(BitXor<i8>; set: i8; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<i16>; set: i16; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<i32>; set: i32; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<i64>; set: i64; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<i128>; set: i128; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<isize>; set: isize; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);

impl_algebra!(BitXor<u8>; set: u8; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<u16>; set: u16; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<u32>; set: u32; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<u64>; set: u64; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<u128>; set: u128; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
impl_algebra!(BitXor<usize>; set: usize; op: |_, a, b| a ^ b; id: |_| 0; inv: |_, a| a; commu; assoc;);
