use crate::{
    geom::{dist_segment_point::*, *},
    min,
};

pub fn dist_segments(l1: Line, l2: Line) -> f64 {
    let cr = l1.cross(l2);
    let t1 = (l2.from - l1.from).cross(l2.diff()) / cr;
    let t2 = (l2.from - l1.from).cross(l1.diff()) / cr;

    if cr == 0.0 || t1 < 0.0 || t1 > 1.0 || t2 < 0.0 || t2 > 1.0 {
        min! {
            dist_segment_point(l1, l2.from),
            dist_segment_point(l1, l2.to),
            dist_segment_point(l2, l1.from),
            dist_segment_point(l2, l1.to)
        }
    } else {
        0.0
    }
}
