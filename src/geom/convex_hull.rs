use crate::geom::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Hull {
    Upper,
    Lower,
}

pub fn half_hull(mut ps: Vec<Vector>, hull: Hull, eps: Eps) -> Vec<Vector> {
    if ps.is_empty() {
        return vec![];
    }

    ps.sort_by(|p, q| (p.0, p.1).partial_cmp(&(q.0, q.1)).unwrap());

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

            if eps.le((q - p).cross(s - p), 0.0) {
                ret.push(s);
            } else {
                ret.pop();
                ps.push(s);
            }
        }
    }

    ret
}

pub fn convex_hull(ps: Vec<Vector>, eps: Eps) -> Vec<Vector> {
    let mut ret = half_hull(ps.clone(), Hull::Upper, eps);
    ret.pop();
    let mut lower = half_hull(ps, Hull::Lower, eps);
    lower.pop();
    ret.extend(lower);
    ret
}
