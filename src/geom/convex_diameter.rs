//! 凸多角形の直径

use crate::geom::*;

pub fn convex_diameter(ps: &Vec<Vector>) -> f64 {
    let n = ps.len();

    let mut i = ps
        .iter()
        .enumerate()
        .min_by(|(_, p), (_, q)| p.1.partial_cmp(&q.1).unwrap())
        .unwrap()
        .0;
    let mut j = ps
        .iter()
        .enumerate()
        .max_by(|(_, p), (_, q)| p.1.partial_cmp(&q.1).unwrap())
        .unwrap()
        .0;

    let mut ret = (ps[i] - ps[j]).abs();

    for _ in 0..2 * n {
        if (ps[(i + 1) % n] - ps[i]).cross(ps[(j + 1) % n] - ps[j]) > 0.0 {
            j = (j + 1) % n;
        } else {
            i = (i + 1) % n;
        }

        ret = ret.max((ps[i] - ps[j]).abs());
    }

    ret
}
