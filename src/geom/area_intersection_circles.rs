//! 2つの円の共通部分の面積

use crate::geom::{intersect_circles::*, *};
use std::f64::consts::PI;

/// 2つの円の共通部分の面積を求める
pub fn area_intersection_circles(a: Circle, b: Circle, eps: Eps) -> f64 {
    use self::IntersectCircles::*;
    let (s, _) = intersect_circles(a, b, eps);

    match s {
        SAME => a.radius * a.radius * PI,
        INSIDE | INSCRIBED => {
            let a_s = a.radius * a.radius * PI;
            let b_s = b.radius * b.radius * PI;
            a_s.min(b_s)
        }
        INTERSECTED => {
            let d = (a.center - b.center).abs();

            let ang =
                ((a.radius * a.radius + d * d - b.radius * b.radius) / (a.radius * d * 2.0)).acos();
            let t1 = (ang - (ang * 2.0).sin() / 2.0) * a.radius * a.radius;

            let ang =
                ((b.radius * b.radius + d * d - a.radius * a.radius) / (b.radius * d * 2.0)).acos();
            let t2 = (ang - (ang * 2.0).sin() / 2.0) * b.radius * b.radius;

            t1 + t2
        }
        _ => 0.0,
    }
}
