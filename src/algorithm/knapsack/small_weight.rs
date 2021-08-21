use std::{cmp::max, ops::Add};

/// 容量が小さいナップサック問題
///
/// Time complexity O(n cap)
///
/// Space complexity O(cap)
pub fn knapsack_small_weight<T>(n: usize, cap: usize, ws: &[usize], vs: &[T]) -> T
where
    T: Default + Copy + Ord + Add<Output = T>,
{
    let mut dp = vec![vec![T::default(); cap + 1]; 2];

    for i in 0..n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0..=cap {
            dp[next][j] = max(dp[next][j], dp[cur][j]);
            if j + ws[i] <= cap {
                dp[next][j + ws[i]] = max(dp[next][j + ws[i]], dp[cur][j] + vs[i]);
            }
        }
    }

    dp[n & 1][cap]
}
