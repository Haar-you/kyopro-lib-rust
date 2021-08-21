use crate::chmin;

/// 価値の総和が小さいナップサック問題
///
/// Time complexity O(n sum(vs))
///
/// Space complexity O(sum(vs))
pub fn knapsack_small_value(n: usize, cap: usize, ws: &[usize], vs: &[usize]) -> usize {
    let max_v = vs.iter().sum::<usize>();
    let mut dp = vec![vec![usize::MAX; max_v + 1]; 2];

    dp[0][0] = 0;

    for i in 0..n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0..=max_v {
            chmin!(dp[next][j], dp[cur][j]);
            if j + vs[i] <= max_v && dp[cur][j] < usize::MAX {
                chmin!(dp[next][j + vs[i]], dp[cur][j] + ws[i]);
            }
        }
    }

    dp[n & 1]
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &x)| x <= cap)
        .unwrap()
        .0
}
