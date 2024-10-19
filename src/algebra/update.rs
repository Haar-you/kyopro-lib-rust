pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Update<T>(PhantomData<T>);

impl<T> Update<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(<T>; Update<T>,
    set: Option<T>,
    op: |_, a, b| match a {
        Some(_) => a,
        None => b
    },
    id: |_| None,
    assoc: {},
    idem: {}
);
