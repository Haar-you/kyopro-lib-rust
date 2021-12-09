use crate::geom::{ccw::*, *};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PointPolygon {
    INCLUSION,
    ON_SEGMENT,
    EXCLUSION,
}

pub fn point_in_polygon<T: Eps>(p: Vector<T>, pl: &[Vector<T>]) -> PointPolygon {
    use self::PointPolygon::*;

    let n = pl.len();
    let mut d = T::from(0.0);

    for i in 0..n {
        if ccw(pl[i], pl[(i + 1) % n], p) == CCW::ON_SEGMENT {
            return ON_SEGMENT;
        }

        let mut a = pl[i].angle(p);
        let mut b = pl[(i + 1) % n].angle(p);

        if a < T::from(0.0) {
            a = a + T::from(2.0 * PI);
        }
        if b < T::from(0.0) {
            b = b + T::from(2.0 * PI);
        }

        let mut ang = b - a;

        if ang.abs() > T::from(PI) {
            if ang <= T::from(0.0) {
                ang = ang + T::from(2.0 * PI);
            } else {
                ang = ang - T::from(2.0 * PI);
            }
        }

        d = d + ang;
    }

    if (d.abs() - T::from(2.0 * PI)).abs() == T::from(0.0) {
        INCLUSION
    } else {
        EXCLUSION
    }
}
