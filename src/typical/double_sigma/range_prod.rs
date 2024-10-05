//! 区間積の総和
use std::ops::{Add, AddAssign, Mul};

use crate::num::one_zero::Zero;
/// 区間積の総和
///
/// Σ{i = 1 ~ N}Σ{j = i ~ N} aᵢ * aᵢ ₊ ₁ * ... * aⱼ
///
/// **Time complexity O(|a|)**
pub fn sum_of_sum_of_range_prod<T>(a: Vec<T>) -> T
where
    T: Copy + Mul<Output = T> + Zero<Output = T> + Add<Output = T> + AddAssign,
{
    let mut ret = T::zero();
    let mut s = T::zero();

    for x in a {
        s = s * x + x;
        ret += s;
    }

    ret
}
