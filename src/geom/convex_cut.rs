use crate::geom::{ccw::*, intersect_line_segment::*, *};

pub fn convex_cut<T: Eps>(ps: &[Vector<T>], l: Line<T>) -> (Vec<Vector<T>>, Vec<Vector<T>>) {
    use self::IntersectLineSegment::*;

    let n = ps.len();
    let mut left = vec![];
    let mut right = vec![];

    for i in 0..n {
        let (s, c) = intersect_line_segment(l, Line::new(ps[i], ps[(i + 1) % n]));

        match s {
            LEFTSIDE => left.push(ps[i]),
            RIGHTSIDE => right.push(ps[i]),
            OVERLAPPED => {
                left.push(ps[i]);
                right.push(ps[i]);
            }
            CROSSED => {
                match ccw(l.from, l.to, ps[i]) {
                    CCW::CLOCKWISE => right.push(ps[i]),
                    _ => left.push(ps[i]),
                };

                left.push(c.unwrap());
                right.push(c.unwrap());
            }
        }
    }

    (left, right)
}
