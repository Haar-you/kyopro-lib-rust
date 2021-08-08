use crate::{ chmin, chmax };

/// 容量が小さいナップサック問題
///
/// Time complexity O(n cap)
///
/// Space complexity O(cap)
pub fn knapsack_small_weight<T>(n: usize, cap: usize, ws: &[usize], vs: &[T]) -> T
where
    T: From<usize> + Copy + Ord + std::ops::Add<Output = T>
{
    let mut dp = vec![vec![T::from(0); cap + 1]; 2];

    for i in 0 .. n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0 ..= cap {
            chmax!(dp[next][j], dp[cur][j]);
            if j + ws[i] <= cap {
                chmax!(dp[next][j + ws[i]], dp[cur][j] + vs[i]);
            }
        }
    }

    dp[n & 1][cap]
}

/// 価値の総和が小さいナップサック問題
///
/// Time complexity O(n sum(vs))
///
/// Space complexity O(sum(vs))
pub fn knapsack_small_value(n: usize, cap: usize, ws: &[usize], vs: &[usize]) -> usize {
    let max_v = vs.iter().sum::<usize>();
    let mut dp = vec![vec![usize::MAX; max_v + 1]; 2];

    dp[0][0] = 0;

    for i in 0 .. n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0 ..= max_v {
            chmin!(dp[next][j], dp[cur][j]);
            if j + vs[i] <= max_v && dp[cur][j] < usize::MAX {
                chmin!(dp[next][j + vs[i]], dp[cur][j] + ws[i]);
            }
        }
    }

    dp[n & 1].iter().enumerate().rev().find(|(_, &x)| x <= cap).unwrap().0
}

/// 個数制限付きナップサック問題
///
/// Time complexity O(n cap log(max(ms)))
///
/// Space complexity O(cap)
pub fn knapsack_limited<T>(n: usize, cap: usize, ws: &[usize], vs: &[T], ms: &[usize]) -> T
where
    T: From<usize> + Copy + Ord + std::ops::Add<Output = T> + std::ops::Mul<Output = T>
{
    let mut dp = vec![T::from(0); cap + 1];

    for i in 0 .. n {
        let mut a = 1;
        let mut x = ms[i];
        while x > 0 {
            let k = std::cmp::min(x, a);

            for j in (0 ..= cap).rev() {
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


/// 個数制限無しナップサック問題
///
/// Time complexity O(n cap)
///
/// Space complexity O(cap)
pub fn knapsack_unlimited<T>(n: usize, cap: usize, ws: &[usize], vs: &[T]) -> T
where
    T: From<usize> + Copy + Ord + std::ops::Add<Output = T>
{
    let mut dp = vec![vec![T::from(0); cap + 1]; 2];

    for i in 0 .. n {
        let next = (i + 1) & 1;
        let cur = i & 1;
        for j in 0 ..= cap {
            if j < ws[i] {
                dp[next][j] = dp[cur][j];
            }
            else {
                dp[next][j] = std::cmp::max(dp[cur][j], dp[next][j - ws[i]] + vs[i]);
            }
        }
    }

    dp[n & 1][cap]
}

use crate::utils::merge::inplace_merge;

/// 要素数が小さいナップサック問題
///
/// Time complexity O(2 ^ (n / 2))
///
/// Space complexity O(2 ^ (n / 2))
pub fn knapsack_small_quantity<W, V>(n: usize, cap: W, ws: &[W], vs: &[V]) -> V
where
    W: From<usize> + Copy + std::ops::Add<Output = W> + Ord,
    V: From<usize> + Copy + std::ops::Add<Output = V> + Ord
{
    let p = n / 2;

    let zero_w = W::from(0);
    let zero_v = V::from(0);

    let mut a: Vec<(W, V)> = Vec::new();
    let mut b: Vec<(W, V)> = Vec::new();

    a.push((zero_w, zero_v));
    b.push((zero_w, zero_v));

    for i in 0 .. p {
        let k = a.len();

        let temp = a.iter().map(|&(w, v)| (w + ws[i], v + vs[i])).collect::<Vec<_>>();
        a.extend_from_slice(&temp);
        inplace_merge(&mut a, k);
    }

    for i in p .. n {
        let k = b.len();

        let temp = b.iter().map(|&(w, v)| (w + ws[i], v + vs[i])).collect::<Vec<_>>();
        b.extend_from_slice(&temp);
        inplace_merge(&mut b, k);
    }

    for i in 1 .. a.len() {
        chmax!(a[i].1, a[i - 1].1);
    }

    for i in 1 .. b.len() {
        chmax!(b[i].1, b[i - 1].1);
    }

    let mut ret = zero_v;

    b.reverse();

    let mut i = 0;
    let mut j = 0;
    while i < a.len() {
        while j < b.len() && a[i].0 + b[j].0 > cap {
            j += 1;
        }
        if j >= b.len() {
            break;
        }
        chmax!(ret, a[i].1 + b[j].1);
        i += 1;
    }

    ret
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_1_B
        assert_eq!(knapsack_small_weight(4, 5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_weight(2, 20, &[9, 10], &[5, 4]), 9);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_1_F
        assert_eq!(knapsack_small_value(4, 5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_value(2, 20, &[9, 10], &[5, 4]), 9);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_G
        assert_eq!(knapsack_limited(4, 8, &[3, 1, 2, 2], &[4, 2, 1, 3], &[2, 1, 4, 2]), 12);
        assert_eq!(knapsack_limited(2, 100, &[1, 1], &[1, 2], &[100, 50]), 150);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_C
        assert_eq!(knapsack_unlimited(4, 8, &[2, 2, 1, 3], &[4, 5, 2, 8]), 21);
        assert_eq!(knapsack_unlimited(2, 20, &[9, 10], &[5, 4]), 10);
        assert_eq!(knapsack_unlimited(3, 9, &[1, 1, 2], &[2, 3, 5]), 27);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_H
        assert_eq!(knapsack_small_quantity(4, 5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_quantity(2, 20, &[9, 10], &[5, 4]), 9);
    }
}
