use std::{cmp::max, ops::Add};

/// 個数制限無しナップサック問題
///
/// **Time complexity O(n cap)**
///
/// **Space complexity O(cap)**
pub fn knapsack_unlimited<T>(cap: usize, ws: &[usize], vs: &[T]) -> T
where
    T: Default + Copy + Ord + Add<Output = T>,
{
    let n = ws.len();
    assert_eq!(ws.len(), vs.len());

    let mut dp = vec![vec![T::default(); cap + 1]; 2];

    for i in 0..n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0..=cap {
            if j < ws[i] {
                dp[next][j] = dp[cur][j];
            } else {
                dp[next][j] = max(dp[cur][j], dp[next][j - ws[i]] + vs[i]);
            }
        }
    }

    dp[n & 1][cap]
}
