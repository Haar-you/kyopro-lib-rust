use std::ops::Add;

/// 部分和問題
///
/// Time complexity O(nk)
///
/// Space complexity O(k)
pub fn subset_sum<T>(n: usize, k: usize, a: &[usize]) -> Vec<T>
where
    T: Copy + Clone + From<usize> + Add<Output = T>,
{
    assert!(a.len() == n);

    let mut dp = vec![vec![T::from(0); k + 1]; 2];

    dp[0][0] = T::from(1);

    for i in 0..n {
        for j in 0..=k {
            if j >= a[i] {
                dp[(i + 1) & 1][j] = dp[i & 1][j - a[i]] + dp[i & 1][j];
            } else {
                dp[(i + 1) & 1][j] = dp[i & 1][j];
            }
        }
    }

    dp[n & 1].clone()
}

/// 各要素に最大使用回数が定められている部分和問題
///
/// Time complexity O(nk)
///
/// Space complexity O(k)
///
/// # Arguments
///
/// * `n` - 要素数 (`== a.len()`, `== m.len()`)
/// * `k` - 判定したい値の最大値 (0以上k以下の整数について部分和が構成できるかを判定する。)
/// * `a` - 総和をとる数列。
/// * `m` - `m[i]`は`a[i]`を使用できる最大回数。
pub fn subset_sum_limited(n: usize, k: usize, a: &[usize], m: &[usize]) -> Vec<bool> {
    assert!(a.len() == n);
    assert!(m.len() == n);

    let mut dp: Vec<isize> = vec![-1; k + 1];

    dp[0] = 0;
    for i in 0..n {
        for j in 0..=k {
            if dp[j] >= 0 {
                dp[j] = m[i] as isize;
            } else if j < a[i] || dp[j - a[i]] <= 0 {
                dp[j] = -1;
            } else {
                dp[j] = dp[j - a[i]] - 1;
            }
        }
    }

    dp.into_iter().map(|x| x >= 0).collect()
}
