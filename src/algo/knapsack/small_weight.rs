//! 容量が小さいナップサック問題
use crate::{chmax, num::one_zero::Zero};
use std::ops::Add;

/// 容量が小さいナップサック問題
///
/// **Time complexity** $O(n \cdot cap)$
///
/// **Space complexity** $O(cap)$
pub fn knapsack_small_weight<T>(cap: usize, ws: &[usize], vs: &[T]) -> T
where
    T: Copy + Ord + Add<Output = T> + Zero,
{
    let n = ws.len();
    assert_eq!(ws.len(), vs.len());

    let mut dp = vec![vec![T::zero(); cap + 1]; 2];

    for i in 0..n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0..=cap {
            chmax!(dp[next][j], dp[cur][j]);
            if j + ws[i] <= cap {
                chmax!(dp[next][j + ws[i]], dp[cur][j] + vs[i]);
            }
        }
    }

    dp[n & 1][cap]
}
