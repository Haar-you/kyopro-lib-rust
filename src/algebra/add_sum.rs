use crate::algebra::action::Action;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Clone, Default)]
pub struct AddSum<T, U>(PhantomData<T>, PhantomData<U>);

impl<T, U> AddSum<T, U> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<T, U> Action for AddSum<T, U>
where
    T: Add<Output = T> + Default + From<U>,
    U: Add<Output = U> + Mul<Output = U> + Default + From<u64>,
{
    type FType = T;
    type UType = U;
    fn fold_id(&self) -> T {
        T::default()
    }
    fn fold(&self, x: T, y: T) -> T {
        x + y
    }
    fn update_id(&self) -> U {
        U::default()
    }
    fn update(&self, x: U, y: U) -> U {
        x + y
    }
    fn convert(&self, x: T, y: U, l: usize) -> T {
        x + T::from(y * U::from(l as u64))
    }
}
