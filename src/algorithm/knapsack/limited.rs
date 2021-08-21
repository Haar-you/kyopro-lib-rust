use crate::chmax;

/// 個数制限付きナップサック問題
///
/// Time complexity O(n cap log(max(ms)))
///
/// Space complexity O(cap)
pub fn knapsack_limited<T>(n: usize, cap: usize, ws: &[usize], vs: &[T], ms: &[usize]) -> T
where
    T: Default + From<usize> + Copy + Ord + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    let mut dp = vec![T::default(); cap + 1];

    for i in 0..n {
        let mut a = 1;
        let mut x = ms[i];
        while x > 0 {
            let k = std::cmp::min(x, a);

            for j in (0..=cap).rev() {
                if j >= k * ws[i] {
                    chmax!(dp[j], dp[j - k * ws[i]] + T::from(k) * vs[i]);
                }
            }

            x -= k;
            a *= 2;
        }
    }

    dp[cap]
}
