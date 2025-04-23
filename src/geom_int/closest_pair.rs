//! 最近点対
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/closest_pair>

use crate::chmin;
use crate::{algo::merge::*, geom_int::*};

/// 最近点対を求める。
pub fn closest_pair(ps: Vec<VectorInt>) -> Option<((usize, VectorInt), (usize, VectorInt))> {
    if ps.len() < 2 {
        None
    } else {
        let mut ps: Vec<_> = ps.into_iter().enumerate().collect();
        ps.sort_by(|(_, p), (_, q)| p.x.cmp(&q.x));
        rec(&mut ps)
    }
}

fn rec(ps: &mut [(usize, VectorInt)]) -> Option<((usize, VectorInt), (usize, VectorInt))> {
    let n = ps.len();
    match n {
        0 => unreachable!(),
        1 => None,
        2 => {
            if ps[0].1.y > ps[1].1.y {
                ps.swap(0, 1);
            }
            Some((ps[0], ps[1]))
        }
        _ => {
            let mid_x = ps[n / 2].1.x;
            let (left, right) = ps.split_at_mut(n / 2);
            let d1 = rec(left);
            let d2 = rec(right);

            inplace_merge_by(ps, n / 2, |a, b| a.1.y < b.1.y);

            let mut d = i64::MAX;
            let mut ret = None;

            if let Some((p, q)) = d1 {
                let t = (p.1 - q.1).abs_sq();
                if chmin!(d, t) {
                    ret = Some((p, q));
                }
            }
            if let Some((p, q)) = d2 {
                let t = (p.1 - q.1).abs_sq();
                if chmin!(d, t) {
                    ret = Some((p, q));
                }
            }

            let mut v: Vec<(usize, VectorInt)> = vec![];

            for &mut p in ps {
                let t = (p.1.x - mid_x).abs();
                if t * t >= d {
                    continue;
                }

                for &q in v.iter().rev() {
                    let t = (p.1.y - q.1.y).abs();
                    if t * t >= d {
                        break;
                    }

                    let t = (p.1 - q.1).abs_sq();
                    if chmin!(d, t) {
                        ret = Some((p, q));
                    }
                }

                v.push(p);
            }

            ret
        }
    }
}
