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

pub fn intersect_line_segment<T: Eps>(
    l: Line<T>,
    s: Line<T>,
) -> (IntersectLineSegment, Option<Vector<T>>) {
    use self::IntersectLineSegment::*;

    let a = l.diff().cross(s.from - l.from);
    let b = l.diff().cross(s.to - l.from);

    match (a.partial_cmp(&T::from(0.0)), b.partial_cmp(&T::from(0.0))) {
        (Some(Equal), Some(Equal)) => (OVERLAPPED, None),
        (Some(Less), Some(Less)) => (RIGHTSIDE, None),
        (Some(Greater), Some(Greater)) => (LEFTSIDE, None),
        _ => (
            CROSSED,
            Some(s.from + s.diff() * l.diff().cross(l.from - s.from) / l.cross(s)),
        ),
    }
}
