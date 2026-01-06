//! 2要素の積の総和

use std::ops::{AddAssign, Mul};

use crate::num::one_zero::Zero;

/// 2要素の積の総和 $\sum_{i = 1}^{N-1} \sum_{j = i+1}^N a_i \times a_j$
///
/// **Time complexity** $O(|a|)$
pub fn sum_of_sum_of_prod<T>(a: Vec<T>) -> T
where
    T: Copy + Mul<Output = T> + AddAssign + Zero,
{
    let mut ret = T::zero();
    let mut acc = T::zero();

    for x in a {
        ret += x * acc;
        acc += x;
    }

    ret
}
