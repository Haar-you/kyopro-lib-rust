use crate::geom::{dist_line_point::*, *};

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectCircleLine {
    OUTSIDE,
    TANGENT,
    CROSSED,
}

pub fn intersect_circle_line<T: Eps>(
    c: Circle<T>,
    l: Line<T>,
) -> (IntersectCircleLine, Vec<Vector<T>>) {
    use self::IntersectCircleLine::*;

    let d = dist_line_point(l, c.center);

    if d > c.radius {
        return (OUTSIDE, vec![]);
    }

    let n = l.normal();
    let b = l.from + l.diff() * n.cross(c.center + n - l.from) / n.cross(l.diff());

    if d == c.radius {
        (TANGENT, vec![b])
    } else {
        let a = (c.radius * c.radius - d * d).sqrt();
        (CROSSED, vec![b + l.unit() * a, b - l.unit() * a])
    }
}
