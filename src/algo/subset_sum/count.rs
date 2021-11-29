use std::ops::Add;

/// 部分和問題 (和を達成する組み合わせ数を返す)
///
/// # Complexity
/// Time complexity $O(nk)$
///
/// Space complexity $O(k)$
pub fn subset_sum<T>(n: usize, k: usize, a: &[usize]) -> Vec<T>
where
    T: Copy + From<usize> + Add<Output = T>,
{
    assert!(a.len() == n);

    let mut dp = vec![vec![T::from(0); k + 1]; 2];

    dp[0][0] = T::from(1);

    for (i, &x) in a.iter().enumerate() {
        let cur = i & 1;
        let next = (i + 1) & 1;
        for j in 0..=k {
            if j >= x {
                dp[next][j] = dp[cur][j - x] + dp[cur][j];
            } else {
                dp[next][j] = dp[cur][j];
            }
        }
    }

    dp[n & 1].clone()
}
