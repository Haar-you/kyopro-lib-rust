use crate::geom::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Hull {
    Upper,
    Lower,
}

pub fn half_hull<T: Eps>(mut ps: Vec<Vector<T>>, hull: Hull) -> Vec<Vector<T>> {
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

            if (q - p).cross(s - p) <= T::from(0.0) {
                ret.push(s);
            } else {
                ret.pop();
                ps.push(s);
            }
        }
    }

    ret
}

pub fn convex_hull<T: Eps>(ps: Vec<Vector<T>>) -> Vec<Vector<T>> {
    let mut ret = half_hull(ps.clone(), Hull::Upper);
    ret.pop();
    let mut lower = half_hull(ps, Hull::Lower);
    lower.pop();
    ret.extend(lower);
    ret
}
