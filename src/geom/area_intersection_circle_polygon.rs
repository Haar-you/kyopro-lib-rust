//! 円と多角形の共通部分の面積

use crate::geom::{intersect_circle_segment::*, *};

/// 円と多角形の共通部分の面積を求める。
pub fn area_intersection_circle_polygon(cl: Circle, ps: &[Vector], eps: Eps) -> f64 {
    use self::IntersectCircleSegment::*;

    let n = ps.len();
    let mut ret = 0.0;

    for i in 0..n {
        let mut temp = 0.0;

        let Circle {
            center: c,
            radius: r,
        } = cl;
        let p1 = ps[i];
        let p2 = ps[(i + 1) % n];

        let (t, res) = intersect_circle_segment(cl, Line::new(p1, p2), eps);

        let d1 = (p1 - c).abs();
        let d2 = (p2 - c).abs();

        match res.len() {
            0 => match t {
                INSIDE => temp += (p1 - c).cross(p2 - c) / 2.0,
                _ => temp += r * r * (p1 - c).angle_diff(p2 - c) / 2.0,
            },
            1 => {
                let q = res[0];
                if d1 >= r && d2 >= r {
                    temp += r * r * (p1 - c).angle_diff(p2 - c) / 2.0;
                } else if d1 >= r {
                    temp += r * r * (p1 - c).angle_diff(q - c) / 2.0 + (q - c).cross(p2 - c) / 2.0
                } else {
                    temp += (p1 - c).cross(q - c) / 2.0 + r * r * (q - c).angle_diff(p2 - c) / 2.0
                }
            }
            _ => {
                let q1 = res[0];
                let q2 = res[1];

                temp += r * r * (p1 - c).angle_diff(q1 - c) / 2.0
                    + (q1 - c).cross(q2 - c) / 2.0
                    + r * r * (q2 - c).angle_diff(p2 - c) / 2.0
            }
        }

        ret += temp;
    }

    ret
}
