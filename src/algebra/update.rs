pub use crate::algebra::traits::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Update<T>(PhantomData<T>);

impl<T> Update<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> AlgeStruct for Update<T> {
    type Output = Option<T>;
}

impl<T> BinaryOp for Update<T> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        match a {
            Some(_) => a,
            None => b,
        }
    }
}

impl<T> Identity for Update<T> {
    fn id(&self) -> Self::Output {
        None
    }
}
