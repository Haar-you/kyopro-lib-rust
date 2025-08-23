//! 空ではない連続する部分列の総和の最大値
//!
//! # Problems
//! - <https://yukicoder.me/problems/no/776>
pub use crate::algebra::traits::*;

use crate::max;

use std::cmp::max;
use std::ops::Add;

/// 空ではない連続する部分列の総和を管理する。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
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

impl<T> Set for MaxPartialSum<T> {}

impl<T: Copy + Ord + Add<Output = T>> BinaryOp for MaxPartialSum<T> {
    fn op(self, b: Self) -> Self {
        let a = self;
        Self {
            sum: a.sum + b.sum,
            left_max: a.left_max.max(a.sum + max(b.left_max, b.sum)),
            right_max: b.right_max.max(b.sum + max(a.right_max, a.sum)),
            partial_max: max!(a.partial_max, b.partial_max, a.right_max + b.left_max),
        }
    }
}

impl<T> Associative for MaxPartialSum<T> {}

#[cfg(test)]
mod tests {
    use crate::iter::collect::CollectVec;

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 20;
        let a = (0..n).map(|_| rng.gen_range(-100..=100)).collect_vec();

        let (ans, _) = crate::algo::max_partial_sum::max_partial_sum(&a).unwrap();

        let res = a
            .iter()
            .map(|&x| Some(MaxPartialSum::new(x)))
            .fold(Option::id(), |x, y| x.op(y))
            .unwrap();

        assert_eq!(ans, res.partial_max);
    }
}
