use crate::algebra::traits::*;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Dual<T, S> {
    s: S,
    phantom: PhantomData<T>,
}

impl<T, S: AlgeStruct<Output = T>> Dual<T, S> {
    pub fn new(s: S) -> Self {
        Self {
            s,
            phantom: PhantomData,
        }
    }
}

impl<T, S: AlgeStruct> AlgeStruct for Dual<T, S> {
    type Output = S::Output;
}

impl<T, S: BinaryOp> BinaryOp for Dual<T, S> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        self.s.op(b, a)
    }
}

impl<T, S: Identity> Identity for Dual<T, S> {
    fn id(&self) -> Self::Output {
        self.s.id()
    }
}
