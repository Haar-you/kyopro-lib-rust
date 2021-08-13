use std::marker::PhantomData;
use crate::algebra::traits::*;

#[derive(Clone)]
pub struct Affine<T>(T, T, PhantomData<T>);

impl<T> Affine<T> {
    pub fn new(zero: T, one: T) -> Self { Self (zero, one, PhantomData) }
}

impl<T> AlgeStruct for Affine<T> {
    type Output = (T, T);
}

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy> BinaryOp for Affine<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        (a.0 * b.0, a.0 * b.1 + a.1)
    }
}

impl<T: Copy> Identity for Affine<T> {
    fn id(&self) -> Self::Output {
        (self.1, self.0)
    }
}
