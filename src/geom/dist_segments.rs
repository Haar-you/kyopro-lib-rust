use crate::{
    geom::{dist_segment_point::*, *},
    min,
};

pub fn dist_segments<T: Eps + std::fmt::Debug>(l1: Line<T>, l2: Line<T>) -> T {
    let cr = l1.cross(l2);
    let t1 = (l2.from - l1.from).cross(l2.diff()) / cr;
    let t2 = (l2.from - l1.from).cross(l1.diff()) / cr;

    if cr == T::from(0.0)
        || t1 < T::from(0.0)
        || t1 > T::from(1.0)
        || t2 < T::from(0.0)
        || t2 > T::from(1.0)
    {
        min! {
            dist_segment_point(l1, l2.from),
            dist_segment_point(l1, l2.to),
            dist_segment_point(l2, l1.from),
            dist_segment_point(l2, l1.to)
        }
    } else {
        T::from(0.0)
    }
}
