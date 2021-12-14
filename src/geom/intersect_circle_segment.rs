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

pub fn intersect_circle_segment<T: Eps>(
    c: Circle<T>,
    s: Line<T>,
) -> (IntersectCircleSegment, Vec<Vector<T>>) {
    use self::IntersectCircleSegment::*;

    let Circle {
        center: c,
        radius: r,
    } = c;
    let d1 = (c - s.from).abs();
    let d2 = (c - s.to).abs();
    let v = dist_segment_point(s, c);
    let m = (r.sq() - v.sq()).sqrt();
    let n = s.normal();
    let k = s.from + s.diff() * n.cross(c + n - s.from) / n.cross(s.diff());

    if d1 < r && d2 < r {
        (INSIDE, vec![])
    } else if v == r {
        (TANGENT, vec![k])
    } else if v > r {
        (OUTSIDE, vec![])
    } else if d1 >= r && d2 >= r {
        (TWO_CROSSPOINTS, vec![k - s.unit() * m, k + s.unit() * m])
    } else {
        let b = s.unit().dot(s.from - c);
        let a = (s.from - c).abs_sq() - r.sq();
        let x = (b.sq() - a).sqrt();

        (
            ONE_CROSSPOINT,
            vec![s.from + s.unit() * (if -b >= x { -b - x } else { -b + x })],
        )
    }
}
