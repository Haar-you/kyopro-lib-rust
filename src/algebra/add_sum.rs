use crate::algebra::action::Action;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Clone)]
pub struct AddSum<T, U>(PhantomData<T>, PhantomData<U>);

impl<T, U> AddSum<T, U> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<T, U> Action<T, U> for AddSum<T, U>
where
    T: Clone + Add<Output = T> + Default + From<U>,
    U: Clone + Add<Output = U> + Mul<Output = U> + Default + From<u64>,
{
    fn fold_id(&self) -> T {
        T::default()
    }
    fn fold(&self, x: T, y: T) -> T {
        x.clone() + y.clone()
    }
    fn update_id(&self) -> U {
        U::default()
    }
    fn update(&self, x: U, y: U) -> U {
        x.clone() + y.clone()
    }
    fn convert(&self, x: T, y: U, l: usize) -> T {
        x.clone() + T::from(y.clone() * U::from(l as u64))
    }
}
