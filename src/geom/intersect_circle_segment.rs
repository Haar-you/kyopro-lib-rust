//! 円と線分の位置関係

use crate::geom::{dist_segment_point::*, *};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectCircleSegment {
    INSIDE,
    OUTSIDE,
    TANGENT,
    ONE_CROSSPOINT,
    TWO_CROSSPOINTS,
}

impl IntersectCircleSegment {
    pub fn inside(self) -> bool {
        self == Self::INSIDE
    }
    pub fn outside(self) -> bool {
        self == Self::OUTSIDE
    }
    pub fn tangent(self) -> bool {
        self == Self::TANGENT
    }
    pub fn one_crosspoint(self) -> bool {
        self == Self::ONE_CROSSPOINT
    }
    pub fn two_crosspoints(self) -> bool {
        self == Self::TWO_CROSSPOINTS
    }
}

pub fn intersect_circle_segment(
    c: Circle,
    s: Line,
    eps: Eps,
) -> (IntersectCircleSegment, Vec<Vector>) {
    use self::IntersectCircleSegment::*;

    let Circle {
        center: c,
        radius: r,
    } = c;
    let d1 = (c - s.from).abs();
    let d2 = (c - s.to).abs();
    let v = dist_segment_point(s, c);
    let m = (r * r - v * v).sqrt();
    let n = s.normal();
    let k = s.from + s.diff() * n.cross(c + n - s.from) / n.cross(s.diff());

    if eps.lt(d1, r) && eps.lt(d2, r) {
        (INSIDE, vec![])
    } else if eps.eq(v, r) {
        (TANGENT, vec![k])
    } else if eps.gt(v, r) {
        (OUTSIDE, vec![])
    } else if eps.ge(d1, r) && eps.ge(d2, r) {
        (TWO_CROSSPOINTS, vec![k - s.unit() * m, k + s.unit() * m])
    } else {
        let b = s.unit().dot(s.from - c);
        let a = (s.from - c).abs_sq() - r * r;
        let x = (b * b - a).sqrt();

        (
            ONE_CROSSPOINT,
            vec![s.from + s.unit() * (if eps.ge(-b, x) { -b - x } else { -b + x })],
        )
    }
}
