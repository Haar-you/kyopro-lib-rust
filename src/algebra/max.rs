pub use crate::algebra::traits::*;
use std::{cmp::max, marker::PhantomData};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Max<T>(PhantomData<T>);
impl<T> Max<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Max<T> {
    type Output = Option<T>;
}

impl<T: Ord + Copy> BinaryOp for Max<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        match (a, b) {
            (None, _) => b,
            (_, None) => a,
            (Some(x), Some(y)) => Some(max(x, y)),
        }
    }
}

impl<T> Identity for Max<T> {
    fn id(&self) -> Self::Output {
        None
    }
}

impl<T> Commutative for Max<T> {}
impl<T> Associative for Max<T> {}
impl<T> Idempotence for Max<T> {}
