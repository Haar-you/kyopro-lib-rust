pub use crate::algebra::traits::*;
pub use crate::algebra::one_zero::*;
use std::{marker::PhantomData, ops::Mul};

#[derive(Clone, Default)]
pub struct Prod<T>(PhantomData<T>);
impl<T> Prod<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Prod<T> {
    type Output = T;
}

impl<T: Mul<Output = T>> BinaryOp for Prod<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        a * b
    }
}

impl<T: One<Output = T>> Identity for Prod<T> {
    fn id(&self) -> Self::Output {
        T::one()
    }
}
