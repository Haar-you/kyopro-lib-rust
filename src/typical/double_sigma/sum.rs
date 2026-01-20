//! 2要素の和の総和

use std::{iter::repeat_n, ops::Add};

use crate::num::one_zero::Zero;

/// 2要素の和の総和 $\sum_{i = 1}^{N - 1} \sum_{j = i + 1}^N a_i + a_j$
///
/// **Time complexity** $O(|a|)$
pub fn sum_of_sum_of_sum<T>(a: Vec<T>) -> T
where
    T: Copy + Add<Output = T> + Zero,
{
    let n = a.len();
    let s = a.into_iter().fold(T::zero(), |x, y| x + y);
    repeat_n(s, n - 1).fold(T::zero(), |x, y| x + y)
}
