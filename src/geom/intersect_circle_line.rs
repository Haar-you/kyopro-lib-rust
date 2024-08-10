//! 円と直線の位置関係

use crate::geom::{dist_line_point::*, *};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectCircleLine {
    OUTSIDE,
    TANGENT,
    CROSSED,
}

impl IntersectCircleLine {
    pub fn outside(self) -> bool {
        self == Self::OUTSIDE
    }
    pub fn tangent(self) -> bool {
        self == Self::TANGENT
    }
    pub fn crossed(self) -> bool {
        self == Self::CROSSED
    }
}

pub fn intersect_circle_line(c: Circle, l: Line, eps: Eps) -> (IntersectCircleLine, Vec<Vector>) {
    use self::IntersectCircleLine::*;

    let d = dist_line_point(l, c.center);

    if eps.gt(d, c.radius) {
        return (OUTSIDE, vec![]);
    }

    let n = l.normal();
    let b = l.from + l.diff() * n.cross(c.center + n - l.from) / n.cross(l.diff());

    if eps.eq(d, c.radius) {
        (TANGENT, vec![b])
    } else {
        let a = (c.radius * c.radius - d * d).sqrt();
        (CROSSED, vec![b + l.unit() * a, b - l.unit() * a])
    }
}
