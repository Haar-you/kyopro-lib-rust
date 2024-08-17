pub use crate::algebra::traits::*;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Dual<S>(S);

impl<S: AlgeStruct> Dual<S> {
    pub fn new(s: S) -> Self {
        Self(s)
    }
}

impl<S: AlgeStruct> AlgeStruct for Dual<S> {
    type Output = S::Output;
}

impl<S: BinaryOp> BinaryOp for Dual<S> {
    fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
        self.0.op(b, a)
    }
}

impl<S: Identity> Identity for Dual<S> {
    fn id(&self) -> Self::Output {
        self.0.id()
    }
}
