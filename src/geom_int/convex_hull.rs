//! 凸包
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/static_convex_hull>

use crate::geom_int::*;

/// 凸包の上半分か下半分かを指定する
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Hull {
    /// 上半分
    Upper,
    /// 下半分
    Lower,
}

/// 上半分/下半分の凸包を求める
pub fn half_hull(mut ps: Vec<VectorInt>, hull: Hull) -> Vec<VectorInt> {
    if ps.is_empty() {
        return vec![];
    }

    ps.sort_by(|p, q| (p.x, p.y).cmp(&(q.x, q.y)));

    if hull == Hull::Upper {
        ps.reverse();
    }

    let mut ret = vec![*ps.last().unwrap()];
    ps.pop();

    while let Some(s) = ps.pop() {
        if ret.len() == 1 {
            ret.push(s);
        } else {
            let p = ret[ret.len() - 2];
            let q = *ret.last().unwrap();

            if (q - p).cross(s - p) < 0 {
                ret.push(s);
            } else {
                ret.pop();
                ps.push(s);
            }
        }
    }

    ret
}

/// 凸包を求める
pub fn convex_hull(ps: Vec<VectorInt>) -> Vec<VectorInt> {
    let mut ret = half_hull(ps.clone(), Hull::Upper);
    let lower = half_hull(ps, Hull::Lower);
    ret.extend(lower);
    ret.dedup();

    if let Some(first) = ret.first().cloned() {
        while ret.len() > 1 && ret.last().is_some_and(|p| p == &first) {
            ret.pop();
        }
    }

    ret
}
