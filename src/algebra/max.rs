//! 最大値を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 最大値を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Max<T>(PhantomData<T>);
impl<T> Max<T> {
    /// `Max<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(Max<i8>; set: i8; op: |_, a: i8, b| a.max(b); id: |_| i8::MIN; commu; assoc; idem;);
impl_algebra!(Max<i16>; set: i16; op: |_, a: i16, b| a.max(b); id: |_| i16::MIN; commu; assoc; idem;);
impl_algebra!(Max<i32>; set: i32; op: |_, a: i32, b| a.max(b); id: |_| i32::MIN; commu; assoc; idem;);
impl_algebra!(Max<i64>; set: i64; op: |_, a: i64, b| a.max(b); id: |_| i64::MIN; commu; assoc; idem;);
impl_algebra!(Max<i128>; set: i128; op: |_, a: i128, b| a.max(b); id: |_| i128::MIN; commu; assoc; idem;);
impl_algebra!(Max<isize>; set: isize; op: |_, a: isize, b| a.max(b); id: |_| isize::MIN; commu; assoc; idem;);

impl_algebra!(Max<u8>; set: u8; op: |_, a: u8, b| a.max(b); id: |_| u8::MIN; commu; assoc; idem;);
impl_algebra!(Max<u16>; set: u16; op: |_, a: u16, b| a.max(b); id: |_| u16::MIN; commu; assoc; idem;);
impl_algebra!(Max<u32>; set: u32; op: |_, a: u32, b| a.max(b); id: |_| u32::MIN; commu; assoc; idem;);
impl_algebra!(Max<u64>; set: u64; op: |_, a: u64, b| a.max(b); id: |_| u64::MIN; commu; assoc; idem;);
impl_algebra!(Max<u128>; set: u128; op: |_, a: u128, b| a.max(b); id: |_| u128::MIN; commu; assoc; idem;);
impl_algebra!(Max<usize>; set: usize; op: |_, a: usize, b| a.max(b); id: |_| usize::MIN; commu; assoc; idem;);
