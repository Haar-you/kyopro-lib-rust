pub use crate::algebra::traits::*;
use std::{
    marker::PhantomData,
    ops::{Add, Neg},
};

#[derive(Clone, Default)]
pub struct Sum<T>(PhantomData<T>);
impl<T> Sum<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Sum<T> {
    type Output = T;
}

impl<T: Add<Output = T>> BinaryOp for Sum<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        a + b
    }
}

impl<T: Zero<Output = T>> Identity for Sum<T> {
    fn id(&self) -> Self::Output {
        T::zero()
    }
}

impl<T: Neg<Output = T>> Inverse for Sum<T> {
    fn inv(&self, a: Self::Output) -> Self::Output {
        -a
    }
}
