//! `Vec`結合のモノイド
use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// `Vec`の結合演算のモノイド
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct ConcatList<T>(PhantomData<T>);
impl<T> ConcatList<T> {
    /// `ConcatList`を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(
    {T} ConcatList<T>;
    set: Vec<T>;
    id: |_| vec![];
    op: |_, mut a: Vec<T>, mut b: Vec<T>| {a.append(&mut b); a};
    assoc;
);
