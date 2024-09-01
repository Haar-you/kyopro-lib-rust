//! 相異なる部分列の総数

use std::collections::HashMap;
use std::hash::Hash;

/// 返り値`ret`に対して、`ret[i]`は`a[0..i]`の相異なる部分列の総数
///
/// **Time complexity O(|a|)**
pub fn num_subseq<T: Hash + Eq + Copy>(a: &[T], m: u64) -> Vec<u64> {
    let n = a.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    let mut tbl = HashMap::new();

    for (i, x) in a.iter().enumerate() {
        let mut t = dp[i] * 2 % m;

        if let Some(&j) = tbl.get(x) {
            t = (t + m - dp[j]) % m;
        }

        dp[i + 1] = t;

        tbl.insert(x, i);
    }

    dp
}
