use crate::geom::*;
use std::cmp::Ordering::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectLineSegment {
    LEFTSIDE,
    RIGHTSIDE,
    OVERLAPPED,
    CROSSED,
}

impl IntersectLineSegment {
    pub fn leftside(self) -> bool {
        self == Self::LEFTSIDE
    }
    pub fn rightside(self) -> bool {
        self == Self::RIGHTSIDE
    }
    pub fn overlapped(self) -> bool {
        self == Self::OVERLAPPED
    }
    pub fn crossed(self) -> bool {
        self == Self::CROSSED
    }
}

pub fn intersect_line_segment(
    l: Line,
    s: Line,
    eps: Eps,
) -> (IntersectLineSegment, Option<Vector>) {
    use self::IntersectLineSegment::*;

    let a = l.diff().cross(s.from - l.from);
    let b = l.diff().cross(s.to - l.from);

    match (eps.partial_cmp(a, 0.0), eps.partial_cmp(b, 0.0)) {
        (Some(Equal), Some(Equal)) => (OVERLAPPED, None),
        (Some(Less), Some(Less)) => (RIGHTSIDE, None),
        (Some(Greater), Some(Greater)) => (LEFTSIDE, None),
        _ => (
            CROSSED,
            Some(s.from + s.diff() * l.diff().cross(l.from - s.from) / l.cross(s)),
        ),
    }
}
