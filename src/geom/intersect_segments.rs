//! 2つの線分の位置関係

use crate::geom::{ccw::*, *};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectSegments {
    INTERSECTED,
    OVERLAPPED,
    NOT_INTERSECTED,
    SAME,
}

impl IntersectSegments {
    pub fn intersected(self) -> bool {
        self == Self::INTERSECTED
    }
    pub fn overlapped(self) -> bool {
        self == Self::OVERLAPPED
    }
    pub fn not_intersected(self) -> bool {
        self == Self::NOT_INTERSECTED
    }
    pub fn same(self) -> bool {
        self == Self::SAME
    }
}

pub fn intersect_segments(a: Line, b: Line, eps: Eps) -> (IntersectSegments, Option<Vector>) {
    use self::IntersectSegments::*;

    let cr = a.cross(b);

    if eps.eq(cr.abs(), 0.0) {
        return if ccw(a.from, a.to, b.from, eps).to_value()
            * ccw(a.from, a.to, b.to, eps).to_value()
            <= 0
            && ccw(b.from, b.to, a.from, eps).to_value() * ccw(b.from, b.to, a.to, eps).to_value()
                <= 0
        {
            (OVERLAPPED, None)
        } else {
            (NOT_INTERSECTED, None)
        };
    }

    let t1 = (b.from - a.from).cross(b.diff()) / cr;
    let t2 = (b.from - a.from).cross(a.diff()) / cr;

    if eps.lt(t1, 0.0) || eps.gt(t1, 1.0) || eps.lt(t2, 0.0) || eps.gt(t2, 1.0) {
        (NOT_INTERSECTED, None)
    } else {
        (INTERSECTED, Some(a.from + a.diff() * t1))
    }
}
