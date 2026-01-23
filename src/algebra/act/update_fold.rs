//! Range Update Range ~~~
pub use crate::algebra::{act::Act, first_last::Last, traits::*};

/// Range Update Range ~~~ 用のモノイド作用
///
/// `act_n`は時間計算量が$O(\log n)$なので、
/// 遅延セグメント木に載せる場合は、更新・取得はともに$O(\log^2 n)$の計算量になることに注意。
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct UpdateFold<T>(pub Last<T>);

impl<T, M> Act<M> for UpdateFold<T>
where
    M: Monoid<Element = T>,
    M::Element: Clone,
{
    type Monoid = Last<T>;
    type Element = Option<T>;

    fn monoid(&self) -> &Self::Monoid {
        &self.0
    }
    fn act(&self, _m: &M, val: M::Element, a: Self::Element) -> M::Element {
        match a {
            Some(a) => a,
            _ => val,
        }
    }
    fn act_n(&self, m: &M, val: <M>::Element, a: Self::Element, len: usize) -> <M>::Element {
        match a {
            Some(a) => m.times(a, len as u64),
            _ => val,
        }
    }
}
