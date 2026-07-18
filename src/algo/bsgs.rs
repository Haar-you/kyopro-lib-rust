//! Baby-step giant-step
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Baby-step_giant-step>
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc270/tasks/abc270_g>
//! - <https://atcoder.jp/contests/abc186/tasks/abc186_e>

use std::{collections::HashMap, hash::Hash};

/// Baby-step giant-step
///
/// $f^{k}(a) = b$を満たす$k$($0 \le k \lt n$)を求める。
/// ($f$について、逆関数$f^{-1}$が存在すること。)
///
/// 条件を満たす$k$が存在しないとき、`Err`を返す
///
/// **Time complexity** $O(\sqrt{n})$
///
/// # Arguments
/// - `f(x, m)` : $x$に$f$を$m$回だけ適用した値を返す。($f^m(x)$)
pub fn bsgs<T, F>(a: T, b: T, n: u64, f: F) -> Option<u64>
where
    T: Copy + Eq + Hash,
    F: Fn(T, u64) -> T,
{
    let m = (n as f64).sqrt().ceil() as u64;

    let mut map = HashMap::new();

    let mut t = b;
    for i in 1..=m {
        t = f(t, 1);
        map.insert(t, i);
    }

    let mut t = a;
    for i in 1..=m {
        t = f(t, m);
        if let Some(j) = map.get(&t) {
            return Some(i * m - j);
        }
    }

    None
}
