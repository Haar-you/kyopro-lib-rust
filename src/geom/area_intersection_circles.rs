use crate::geom::{intersect_circles::*, *};
use std::f64::consts::PI;

pub fn area_intersection_circles<T: Eps>(a: Circle<T>, b: Circle<T>) -> T {
    use self::IntersectCircles::*;
    let (s, _) = intersect_circles(a, b);

    match s {
        SAME => a.radius.sq() * T::from(PI),
        INSIDE | INSCRIBED => {
            let a_s = a.radius.sq() * T::from(PI);
            let b_s = b.radius.sq() * T::from(PI);
            a_s.min(b_s)
        }
        INTERSECTED => {
            let d = (a.center - b.center).abs();

            let ang =
                ((a.radius.sq() + d.sq() - b.radius.sq()) / (a.radius * d * T::from(2.0))).acos();
            let t1 = (ang - (ang * T::from(2.0)).sin() / T::from(2.0)) * a.radius.sq();

            let ang =
                ((b.radius.sq() + d.sq() - a.radius.sq()) / (b.radius * d * T::from(2.0))).acos();
            let t2 = (ang - (ang * T::from(2.0)).sin() / T::from(2.0)) * b.radius.sq();

            t1 + t2
        }
        _ => T::from(0.0),
    }
}
