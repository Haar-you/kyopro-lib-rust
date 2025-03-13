//! Monotone minima
//!
//! # Problems
//! - <https://atcoder.jp/contests/colopl2018-final/tasks/colopl2018_final_c>
//! - <https://judge.yosupo.jp/problem/min_plus_convolution_convex_convex>

use crate::chmin;

/// `n`行`m`列の行列の各行の最小値の位置と値を求める。
///
/// ただし、$i$番目の行の最小値となる列の位置$a_i$について、$a_0 \le a_1 \le \dots \le a_{n-1}$、を満たしていること。
///
/// **Time complexity** $O(n + m \log n)$
pub fn monotone_minima<T, F>(n: usize, m: usize, a: F) -> Vec<(usize, T)>
where
    T: Ord + Clone,
    F: Fn(usize, usize) -> T,
{
    let mut ret = vec![None; n];
    rec(n, m, 0, n, 0, m, &a, &mut ret);
    ret.into_iter().flatten().collect()
}

fn rec<T, F>(
    _n: usize,
    _m: usize,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    a: &F,
    ret: &mut Vec<Option<(usize, T)>>,
) where
    T: Ord,
    F: Fn(usize, usize) -> T,
{
    if top >= bottom {
        return;
    }
    if top + 1 == bottom {
        let i = top;

        let mut index = left;
        let mut m = a(i, left);

        for j in left + 1..right {
            if chmin!(m, a(i, j)) {
                index = j;
            }
        }

        ret[i] = Some((index, m));
        return;
    }

    let mid = (top + bottom) / 2;
    rec(_n, _m, mid, mid + 1, left, right, a, ret);

    let min_index = ret[mid].as_ref().unwrap().0;
    rec(_n, _m, top, mid, left, min_index + 1, a, ret);
    rec(_n, _m, mid + 1, bottom, min_index, right, a, ret);
}
