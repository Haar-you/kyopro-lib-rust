//! 最近点対

use crate::{algo::merge::*, geom::*};

pub fn closest_pair(mut ps: Vec<Vector>, eps: Eps) -> Option<(Vector, Vector)> {
    if ps.len() < 2 {
        None
    } else {
        ps.sort_by(|p, q| p.0.partial_cmp(&q.0).unwrap());
        rec(&mut ps, eps)
    }
}

fn rec(ps: &mut [Vector], eps: Eps) -> Option<(Vector, Vector)> {
    let n = ps.len();
    match n {
        0 => unreachable!(),
        1 => None,
        2 => {
            if eps.gt(ps[0].1, ps[1].1) {
                ps.swap(0, 1);
            }
            Some((ps[0], ps[1]))
        }
        _ => {
            let mid_x = ps[n / 2].0;
            let (left, right) = ps.split_at_mut(n / 2);
            let d1 = rec(left, eps);
            let d2 = rec(right, eps);

            inplace_merge_by(ps, n / 2, |a, b| a.1 < b.1);

            let mut d = f64::INFINITY;
            let mut ret = None;

            if let Some((p, q)) = d1 {
                let t = (p - q).abs();
                if eps.lt(t, d) {
                    d = t;
                    ret = Some((p, q));
                }
            }
            if let Some((p, q)) = d2 {
                let t = (p - q).abs();
                if eps.lt(t, d) {
                    d = t;
                    ret = Some((p, q));
                }
            }

            let mut v: Vec<Vector> = vec![];

            for &mut p in ps {
                if eps.gt((p.0 - mid_x).abs(), d) {
                    continue;
                }

                for &q in v.iter().rev() {
                    if eps.gt((p.1 - q.1).abs(), d) {
                        break;
                    }

                    let t = (p - q).abs();
                    if eps.lt(t, d) {
                        d = t;
                        ret = Some((p, q));
                    }
                }

                v.push(p);
            }

            ret
        }
    }
}
