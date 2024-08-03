//! 点と多角形の位置関係

use crate::geom::{ccw::*, *};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PointPolygon {
    INCLUSION,
    ON_SEGMENT,
    EXCLUSION,
}

impl PointPolygon {
    pub fn inclusion(self) -> bool {
        self == Self::INCLUSION
    }
    pub fn on_segment(self) -> bool {
        self == Self::ON_SEGMENT
    }
    pub fn exclusion(self) -> bool {
        self == Self::EXCLUSION
    }
}

pub fn point_in_polygon(p: Vector, pl: &[Vector], eps: Eps) -> PointPolygon {
    use self::PointPolygon::*;

    let n = pl.len();
    let mut d = 0.0;

    for i in 0..n {
        if ccw(pl[i], pl[(i + 1) % n], p, eps) == CCW::ON_SEGMENT {
            return ON_SEGMENT;
        }

        let mut a = pl[i].angle(p);
        let mut b = pl[(i + 1) % n].angle(p);

        if eps.lt(a, 0.0) {
            a += 2.0 * PI;
        }
        if eps.lt(b, 0.0) {
            b += 2.0 * PI;
        }

        let mut ang = b - a;

        if eps.gt(ang.abs(), PI) {
            if eps.le(ang, 0.0) {
                ang += 2.0 * PI;
            } else {
                ang -= 2.0 * PI;
            }
        }

        d += ang;
    }

    if eps.eq((d.abs() - 2.0 * PI).abs(), 0.0) {
        INCLUSION
    } else {
        EXCLUSION
    }
}
