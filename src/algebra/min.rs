pub use crate::algebra::traits::*;
use std::{cmp::min, marker::PhantomData};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Min<T>(PhantomData<T>);
impl<T> Min<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Min<T> {
    type Output = Option<T>;
}

impl<T: Ord + Copy> BinaryOp for Min<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        match (a, b) {
            (None, _) => b,
            (_, None) => a,
            (Some(x), Some(y)) => Some(min(x, y)),
        }
    }
}

impl<T> Identity for Min<T> {
    fn id(&self) -> Self::Output {
        None
    }
}

impl<T> Commutative for Min<T> {}
impl<T> Associative for Min<T> {}
impl<T> Idempotence for Min<T> {}
