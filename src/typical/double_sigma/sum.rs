//! 2要素の和の総和

use std::{iter::repeat, ops::Add};

use crate::num::one_zero::Zero;

/// 2要素の和の総和
///
/// Σ{i = 1 ~ N - 1}Σ{j = i + 1 ~ N} aᵢ + aⱼ
///
/// **Time complexity** $O(|a|)$
pub fn sum_of_sum_of_sum<T>(a: Vec<T>) -> T
where
    T: Copy + Add<Output = T> + Zero,
{
    let n = a.len();
    let s = a.into_iter().fold(T::zero(), |x, y| x + y);
    repeat(s).take(n - 1).fold(T::zero(), |x, y| x + y)
}
