pub use crate::algebra::traits::*;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Trivial<T>(PhantomData<T>);

impl<T> Trivial<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Trivial<T> {
    type Output = ();
}

impl<T> BinaryOp for Trivial<T> {
    fn op(&self, _: Self::Output, _: Self::Output) -> Self::Output {
        ()
    }
}

impl<T> Identity for Trivial<T> {
    fn id(&self) -> Self::Output {
        ()
    }
}

impl<T> Inverse for Trivial<T> {
    fn inv(&self, _: Self::Output) -> Self::Output {
        ()
    }
}

impl<T> Associative for Trivial<T> {}
impl<T> Commutative for Trivial<T> {}
impl<T> Idempotence for Trivial<T> {}
