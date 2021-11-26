use crate::geom::{ccw::*, *};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectSegments {
    INTERSECTED,
    OVERLAPPED,
    NOT_INTERSECTED,
    SAME,
}

pub fn intersect_segments<T: Eps>(
    a: Line<T>,
    b: Line<T>,
) -> (IntersectSegments, Option<Vector<T>>) {
    let cr = a.cross(b);

    if cr.abs() == T::from(0.0) {
        return if ccw(a.from, a.to, b.from).to_value() * ccw(a.from, a.to, b.to).to_value() <= 0
            && ccw(b.from, b.to, a.from).to_value() * ccw(b.from, b.to, a.to).to_value() <= 0
        {
            (IntersectSegments::OVERLAPPED, None)
        } else {
            (IntersectSegments::NOT_INTERSECTED, None)
        };
    }

    let t1 = (b.from - a.from).cross(b.diff()) / cr;
    let t2 = (b.from - a.from).cross(a.diff()) / cr;

    if t1 < T::from(0.0) || t1 > T::from(1.0) || t2 < T::from(0.0) || t2 > T::from(1.0) {
        (IntersectSegments::NOT_INTERSECTED, None)
    } else {
        (IntersectSegments::INTERSECTED, Some(a.from + a.diff() * t1))
    }
}
