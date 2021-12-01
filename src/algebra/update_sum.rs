use crate::algebra::action::Action;
use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Clone, Default)]
pub struct UpdateSum<T, U>(PhantomData<T>, PhantomData<U>);

impl<T, U> UpdateSum<T, U> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<T, U> Action for UpdateSum<T, U>
where
    T: Add<Output = T> + Default + From<U>,
    U: Mul<Output = U> + Default + From<u64>,
{
    type FType = T;
    type UType = Option<U>;
    fn fold_id(&self) -> Self::FType {
        T::default()
    }
    fn fold(&self, x: Self::FType, y: Self::FType) -> Self::FType {
        x + y
    }
    fn update_id(&self) -> Self::UType {
        None
    }
    fn update(&self, x: Self::UType, y: Self::UType) -> Self::UType {
        match x {
            Some(_) => x,
            _ => y,
        }
    }
    fn convert(&self, x: Self::FType, y: Self::UType, l: usize) -> Self::FType {
        match y {
            Some(y) => T::from(y * U::from(l as u64)),
            _ => x,
        }
    }
}
