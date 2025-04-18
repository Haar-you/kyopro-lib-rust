//! 偏角ソート
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/sort_points_by_argument>
use std::cmp::Ordering;

use crate::geom_int::*;

/// 偏角ソートした`Vec`を返す。
///
/// 偏角($-\pi \lt \arg \le \pi$)で点をソートする。
/// ただし、点(0,0)は偏角が0とする。
pub fn arg_sort(a: Vec<VectorInt>) -> Vec<VectorInt> {
    let mut ret = vec![];

    let mut upper = vec![];
    let mut lower = vec![];
    let mut zero = vec![];
    let mut x_plus = vec![];
    let mut x_minus = vec![];

    for p in a {
        match p.1.cmp(&0) {
            Ordering::Equal => match p.0.cmp(&0) {
                Ordering::Greater => x_plus.push(p),
                Ordering::Less => x_minus.push(p),
                Ordering::Equal => zero.push(p),
            },
            Ordering::Greater => upper.push(p),
            Ordering::Less => lower.push(p),
        }
    }

    upper.sort_by(|p, q| q.cross(*p).cmp(&0));
    lower.sort_by(|p, q| q.cross(*p).cmp(&0));

    ret.extend(lower);
    ret.extend(x_plus);
    ret.extend(zero);
    ret.extend(upper);
    ret.extend(x_minus);

    ret
}
