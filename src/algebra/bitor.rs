//! 論理和を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 論理和を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct BitOr<T>(PhantomData<T>);
impl<T> BitOr<T> {
    /// `BitOr<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(BitOr<u8>, set: u8, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u16>, set: u16, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u32>, set: u32, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u64>, set: u64, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<u128>, set: u128, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
impl_algebra!(BitOr<usize>, set: usize, op: |_, a, b| a | b, id: |_| 0, commu: {}, assoc: {}, idem: {});
