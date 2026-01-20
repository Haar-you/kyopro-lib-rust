//! 最小値とその個数の総和
use crate::algebra::traits::*;
use crate::num::one_zero::Zero;
use std::{cmp::Ordering, marker::PhantomData, ops::Add};

/// 最小値とその個数の総和
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct MinCount<T, U>(PhantomData<(T, U)>);
impl<T, U> MinCount<T, U> {
    /// [`MinCount<T, U>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Set for MinCount<T, U> {
    type Element = (Option<T>, U);
}
impl<T: Ord, U: Add<Output = U>> BinaryOp for MinCount<T, U> {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        match (a, b) {
            ((None, ca), (None, cb)) => (None, ca + cb),
            ((None, _), a) => a,
            (b, (None, _)) => b,
            ((Some(a), ca), (Some(b), cb)) => match a.cmp(&b) {
                Ordering::Equal => (Some(a), ca + cb),
                Ordering::Less => (Some(a), ca),
                Ordering::Greater => (Some(b), cb),
            },
        }
    }
}
impl<T, U: Zero + PartialEq> Identity for MinCount<T, U> {
    fn id(&self) -> Self::Element {
        (None, U::zero())
    }
    fn is_id(&self, a: &Self::Element) -> bool {
        a.0.is_none() && a.1 == U::zero()
    }
}
impl<T, U> Associative for MinCount<T, U> {}
impl<T, U> Commutative for MinCount<T, U> {}
