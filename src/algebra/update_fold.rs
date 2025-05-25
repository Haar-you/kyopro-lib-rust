//! Range Update Range ~~~ 用の代数的構造
use crate::algebra::action::Action;
use crate::algebra::first_last::*;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Range Update Range ~~~ 用の代数的構造
///
/// `convert`は時間計算量が$O(\log n)$なので、
/// 遅延セグメント木に載せる場合は、更新・取得はともに$O(\log^2 n)$の計算量になることに注意。
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct UpdateFold<M: Monoid>(PhantomData<M>);

impl<M> Action for UpdateFold<M>
where
    M: Monoid + Clone,
{
    type Output = M;
    type Lazy = Last<M>;

    fn convert(value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output {
        match lazy.0 {
            Some(m) => m.times(len as u64),
            _ => value,
        }
    }
}
