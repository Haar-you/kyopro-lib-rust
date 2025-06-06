//! First, Lastモノイド
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 最初に出現する`Some`を返す演算。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct First<T>(PhantomData<T>);
/// 最後に出現する`Some`を返す演算。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Last<T>(PhantomData<T>);

impl<T> First<T> {
    /// `First<T>`を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T> Last<T> {
    /// `Last<T>`を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(
    [T]; First<T>;
    set: Option<T>;
    op: |_, a, b| match a {
        Some(_) => a,
        None => b
    };
    id: |_| None;
    assoc;
    idem;
);
impl_algebra!(
    [T]; Last<T>;
    set: Option<T>;
    op: |_, a, b| match b {
        Some(_) => b,
        None => a
    };
    id: |_| None;
    assoc;
    idem;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [None, None, Some(1), None, Some(3), Some(5)];

        let monoid = First::new();
        dbg!(a.iter().fold(monoid.id(), |x, y| monoid.op(x, *y)));

        let monoid = Last::new();
        dbg!(a.iter().fold(monoid.id(), |x, y| monoid.op(x, *y)));
    }
}
