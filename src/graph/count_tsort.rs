//! トポロジカルソートの数え上げ
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc041/tasks/abc041_d>

use std::ops::Add;

use crate::{graph::*, num::one_zero::*};

/// トポロジカルソートを数え上げる。
///
/// **Time complexity** $O(V 2^V)$
pub fn count_tsort<T, W, I>(graph: &Graph<Directed, W, I>) -> T
where
    T: One + Zero + Copy + Add<Output = T>,
{
    let n = graph.len();
    let mut dp = vec![T::zero(); 1 << n];

    assert!(n <= 32);
    let mut g = vec![0; n];
    for i in 0..n {
        for e in graph.node_of(i) {
            g[i] |= 1 << e.to();
        }
    }

    dp[0] = T::one();

    for s in 0..(1 << n) {
        for i in 0..n {
            if s & (1 << i) != 0 {
                if (s ^ (1 << i)) & g[i] != 0 {
                    continue;
                }
                dp[s] = dp[s] + dp[s ^ (1 << i)];
            }
        }
    }

    dp[(1 << n) - 1]
}
