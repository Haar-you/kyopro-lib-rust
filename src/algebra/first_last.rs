//! First, Lastモノイド
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 最初に出現する`Some`を返す演算。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct First<T>(pub Option<T>);
/// 最後に出現する`Some`を返す演算。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Last<T>(pub Option<T>);

impl_algebra!(
    [T]; First<T>;
    op: |a: Self, b| match a.0 {
        Some(_) => a,
        None => b
    };
    id: Self(None);
    assoc;
    idem;
);
impl_algebra!(
    [T]; Last<T>;
    op: |a, b: Self| match b.0 {
        Some(_) => b,
        None => a
    };
    id: Self(None);
    assoc;
    idem;
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [None, None, Some(1), None, Some(3), Some(5)];

        let b: Vec<_> = a.into_iter().map(First).collect();
        dbg!(b.iter().fold(First::id(), |x, y| First::op(x, *y)));

        let b: Vec<_> = a.into_iter().map(Last).collect();
        dbg!(b.iter().fold(Last::id(), |x, y| Last::op(x, *y)));
    }
}
