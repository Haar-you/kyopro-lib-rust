pub use crate::algebra::traits::*;
use crate::impl_algebra;
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

impl_algebra!(T; Trivial<T>, op: |_, _, _| (), id: |_| (), inv: |_, _| (),
    assoc: {}, commu: {}, idem: {}
);
