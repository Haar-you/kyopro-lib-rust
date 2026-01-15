//! 和の最小値で畳み込み
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/min_plus_convolution_convex_arbitrary>
use crate::algo::monotone_minima::*;

/// $c_k = \min_{i + j = k} (a_i + b_j)$を満たす$c$を求める。
///
/// # Requirements
/// `a`は下に凸な列である。
///
/// # Return
/// `c.len` = `a.len() + b.len() - 1`
pub fn min_plus_conv_convex(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    assert!(!a.is_empty());
    assert!(!b.is_empty());
    let n = a.len();
    let m = b.len();

    if n >= 2 {
        for i in 0..n - 2 {
            assert!(2 * a[i + 1] <= a[i + 2] + a[i], "`a`が下に凸ではない。");
        }
    }

    let f = |i, j| {
        if i < j || i >= n + j {
            i64::MAX
        } else {
            a[i - j] + b[j]
        }
    };
    monotone_minima(n + m - 1, m, f)
        .into_iter()
        .map(|(_, c)| c)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{chmin, iter::collect::CollectVec};

    use super::*;

    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let n = rng.gen_range(100..=500);
            let m = rng.gen_range(100..=500);

            let a = {
                // let argmin = rng.gen_range(0..n);

                let mut a = vec![0; n];

                let mut d = -500;

                a[0] = rng.gen_range(-1000..=1000);

                for i in 1..n {
                    let dd = rng.gen_range(0..10);
                    d += dd;
                    a[i] = a[i - 1] + d;
                }

                a
            };
            let b = (0..m).map(|_| rng.gen_range(-1000..=1000)).collect_vec();

            let mut ans = vec![i64::MAX; n + m - 1];
            for i in 0..n {
                for j in 0..m {
                    chmin!(ans[i + j], a[i] + b[j]);
                }
            }

            assert_eq!(ans, min_plus_conv_convex(a, b));
        }
    }
}
