//! 空ではない連続する部分列の総和の最大値
//!
//! # Problems
//! - <https://yukicoder.me/problems/no/776>
pub use crate::algebra::traits::*;

use crate::max;

use std::cmp::max;
use std::marker::PhantomData;
use std::ops::Add;

/// 空ではない連続する部分列の総和を管理する。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct MaxPartialSum<T> {
    /// 列の総和
    pub sum: T,
    /// 列の左端から連続する空でない部分列の総和の最大値
    pub left_max: T,
    /// 列の右端から連続する空でない部分列の総和の最大値
    pub right_max: T,
    /// 連続する空でない部分列の総和の最大値
    pub partial_max: T,
}

impl<T: Copy> MaxPartialSum<T> {
    /// 値`value`をもつ長さ`1`の列に対応する[`MaxPartialSum`]を生成する。
    pub fn new(value: T) -> Self {
        Self {
            sum: value,
            left_max: value,
            right_max: value,
            partial_max: value,
        }
    }
}

/// [`MaxPartialSum`]の合成
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Composition<T>(PhantomData<T>);
impl<T> Composition<T> {
    /// [`Composition<T>`]を返す。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Set for Composition<T> {
    type Element = MaxPartialSum<T>;
}

impl<T: Copy + Ord + Add<Output = T>> BinaryOp for Composition<T> {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        MaxPartialSum {
            sum: a.sum + b.sum,
            left_max: a.left_max.max(a.sum + max(b.left_max, b.sum)),
            right_max: b.right_max.max(b.sum + max(a.right_max, a.sum)),
            partial_max: max!(a.partial_max, b.partial_max, a.right_max + b.left_max),
        }
    }
}

impl<T> Associative for Composition<T> {}

#[cfg(test)]
mod tests {
    use crate::{algebra::option::AppendId, iter::collect::CollectVec};

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 20;
        let a = std::iter::repeat_with(|| rng.gen_range(-100..=100))
            .take(n)
            .collect_vec();

        let (ans, _) = crate::algo::max_partial_sum::max_partial_sum(&a).unwrap();

        let m = AppendId(Composition::new());

        let res = a
            .iter()
            .map(|&x| Some(MaxPartialSum::new(x)))
            .fold(m.id(), |x, y| m.op(x, y))
            .unwrap();

        assert_eq!(ans, res.partial_max);
    }
}
