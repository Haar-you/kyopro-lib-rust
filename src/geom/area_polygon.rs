use crate::geom::*;

pub fn area_polygon(ps: &[Vector]) -> f64 {
    let mut ret = 0.0;
    let n = ps.len();

    for i in 0..n {
        ret += (ps[i].0 - ps[(i + 1) % n].0) * (ps[i].1 + ps[(i + 1) % n].1);
    }

    ret.abs() / 2.0
}
