use crate::algebra::action::Action;
use crate::num::one_zero::Zero;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct UpdateSum<T, U>(PhantomData<T>, PhantomData<U>);

impl<T, U> UpdateSum<T, U> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<T, U> Action for UpdateSum<T, U>
where
    T: Add<Output = T> + Zero + From<U>,
    U: Mul<Output = U> + From<u64>,
{
    type Output = T;
    type Lazy = Option<U>;
    fn fold_id(&self) -> Self::Output {
        T::zero()
    }
    fn fold(&self, left: Self::Output, right: Self::Output) -> Self::Output {
        left + right
    }
    fn update_id(&self) -> Self::Lazy {
        None
    }
    fn update(&self, next: Self::Lazy, cur: Self::Lazy) -> Self::Lazy {
        match next {
            Some(_) => next,
            _ => cur,
        }
    }
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy {
            Some(lazy) => T::from(lazy * U::from(len as u64)),
            _ => value,
        }
    }
}
