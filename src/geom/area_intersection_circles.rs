use crate::geom::{intersect_circles::*, *};
use std::f64::consts::PI;

pub fn area_intersection_circles<T: Eps>(a: Circle<T>, b: Circle<T>) -> T {
    use self::IntersectCircles::*;
    let (s, _) = intersect_circles(a, b);

    match s {
        SAME => a.radius * a.radius * T::from(PI),
        INSIDE | INSCRIBED => {
            let a_s = a.radius * a.radius * T::from(PI);
            let b_s = b.radius * b.radius * T::from(PI);
            a_s.min(b_s)
        }
        INTERSECTED => {
            let d = (a.center - b.center).abs();

            let ang = ((a.radius * a.radius + d * d - b.radius * b.radius)
                / (a.radius * d * T::from(2.0)))
            .acos();
            let t1 = (ang - (ang * T::from(2.0)).sin() / T::from(2.0)) * a.radius * a.radius;

            let ang = ((b.radius * b.radius + d * d - a.radius * a.radius)
                / (b.radius * d * T::from(2.0)))
            .acos();
            let t2 = (ang - (ang * T::from(2.0)).sin() / T::from(2.0)) * b.radius * b.radius;

            t1 + t2
        }
        _ => T::from(0.0),
    }
}
