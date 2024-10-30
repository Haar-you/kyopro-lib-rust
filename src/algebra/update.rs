//! 値の更新を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 値の更新を演算とする代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Update<T>(PhantomData<T>);

impl<T> Update<T> {
    /// `Update<T>`を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(<T>; Update<T>,
    set: Option<T>,
    op: |_, a, b| match a {
        Some(_) => a,
        None => b
    },
    id: |_| None,
    assoc: {},
    idem: {}
);
