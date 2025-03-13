pub use crate::algebra::traits::*;

use crate::max;

use std::cmp::max;
use std::marker::PhantomData;
use std::ops::Add;

/// 連続する部分列の総和を管理する。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct MaxPartialSumValue<T> {
    sum: T,
    left_max: T,
    right_max: T,
    partial_max: T,
}

impl<T: Copy> MaxPartialSumValue<T> {
    /// 値`value`をもつ[`MaxPartialSumValue`]を生成する。
    pub fn new(value: T) -> Self {
        Self {
            sum: value,
            left_max: value,
            right_max: value,
            partial_max: value,
        }
    }
}

/// 連続する部分列の総和の最大値を扱う。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct MaxPartialSum<T>(PhantomData<T>);

impl<T> MaxPartialSum<T> {
    /// [`MaxPartialSum`]を生成する。
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Set for MaxPartialSum<T> {
    type Element = Option<MaxPartialSumValue<T>>;
}

impl<T> Identity for MaxPartialSum<T> {
    fn id(&self) -> Self::Element {
        None
    }
}

impl<T: Copy + Ord + Add<Output = T>> BinaryOp for MaxPartialSum<T> {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        match (a, b) {
            (None, None) => None,
            (Some(_), None) => a,
            (None, Some(_)) => b,
            (Some(a), Some(b)) => Some(MaxPartialSumValue {
                sum: a.sum + b.sum,
                left_max: a.left_max.max(a.sum + max(b.left_max, b.sum)),
                right_max: b.right_max.max(b.sum + max(a.right_max, a.sum)),
                partial_max: max!(a.partial_max, b.partial_max, a.right_max + b.left_max),
            }),
        }
    }
}

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
        let monoid = MaxPartialSum::<i64>::new();

        let (ans, _) = crate::algo::max_partial_sum::max_partial_sum(&a).unwrap();

        let res = a
            .iter()
            .map(|&x| Some(MaxPartialSumValue::new(x)))
            .fold(monoid.id(), |x, y| monoid.op(x, y))
            .unwrap();

        assert_eq!(ans, res.partial_max);
    }
}
