//! First, Lastモノイド
use std::marker::PhantomData;

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 最初に出現する`Some`を返す演算。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct First<T>(PhantomData<T>);
/// 最後に出現する`Some`を返す演算。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Last<T>(PhantomData<T>);

impl<T> First<T> {
    /// [`First<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T> Last<T> {
    /// [`Last<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl_algebra!(
    [T]; First<T>;
    set: Option<T>;
    op: |_, a: Option<T>, b| a.or(b);
    id: |_| None, |_, a: &Option<T>| a.is_none();
    assoc;
    idem;
);
impl_algebra!(
    [T]; Last<T>;
    set: Option<T>;
    op: |_, a, b: Option<T>| b.or(a);
    id: |_| None, |_, a: &Option<T>| a.is_none();
    assoc;
    idem;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [None, None, Some(1), None, Some(3), Some(5)];

        assert_eq!(a.iter().cloned().fold_m(&First::new()), Some(1));
        assert_eq!(a.iter().cloned().fold_m(&Last::new()), Some(5));
    }
}
