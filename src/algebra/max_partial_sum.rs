//! 空ではない連続する部分列の総和の最大値
//!
//! # Problems
//! - <https://yukicoder.me/problems/no/776>
pub use crate::algebra::traits::*;

use crate::{impl_algebra, max};

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

impl<T> MaxPartialSum<T>
where
    T: Copy + Ord + Add<Output = T>,
{
    /// 値`value`をもつ長さ`1`の列に対応する[`MaxPartialSum`]を生成する。
    pub fn new(value: T) -> Self {
        Self {
            sum: value,
            left_max: value,
            right_max: value,
            partial_max: value,
        }
    }

    /// `MaxPartialSum`を合成する。
    pub fn compose(self, b: Self) -> Self {
        Self {
            sum: self.sum + b.sum,
            left_max: self.left_max.max(self.sum + max(b.left_max, b.sum)),
            right_max: b.right_max.max(b.sum + max(self.right_max, self.sum)),
            partial_max: max!(self.partial_max, b.partial_max, self.right_max + b.left_max),
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

impl_algebra!({T: Copy + Ord + Add<Output = T>} Composition<T>; set: MaxPartialSum<T>;
              op: |_, a: Self::Element, b| a.compose(b); assoc;);

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
