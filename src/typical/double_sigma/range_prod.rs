//! 区間積の総和
use std::ops::{Add, AddAssign, Mul};

use crate::num::one_zero::Zero;
/// 区間積の総和 $\sum_{i = 1}^N \sum_{j = i}^N a_i \times a_{i+1} \times \dots \times a_j$
///
/// **Time complexity** $O(|a|)$
pub fn sum_of_sum_of_range_prod<T>(a: Vec<T>) -> T
where
    T: Copy + Mul<Output = T> + Zero + Add<Output = T> + AddAssign,
{
    let mut ret = T::zero();
    let mut s = T::zero();

    for x in a {
        s = s * x + x;
        ret += s;
    }

    ret
}
