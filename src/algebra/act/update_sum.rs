//! Range Update Range Sum
pub use crate::algebra::{act::Act, first_last::Last, traits::*};

/// Range Update Range Sum 用のモノイド作用
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UpdateSum<T>(pub Last<T>);

impl<T, M> Act<M> for UpdateSum<T>
where
    M: Monoid<Element = T> + Additive,
{
    type Monoid = Last<T>;
    type Element = Option<T>;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &M, val: <M>::Element, a: Self::Element) -> <M>::Element {
        match a {
            Some(a) => a,
            _ => val,
        }
    }
    fn act_n(&self, m: &M, val: <M>::Element, a: Self::Element, len: usize) -> <M>::Element {
        match a {
            Some(a) => Additive::times(m, a, len as u64),
            _ => val,
        }
    }
}
