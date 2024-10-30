//! 点と線分の位置関係
//!
//! # Problems
//! - <https://onlinejudge.u-aizu.ac.jp/courses/library/4/CGL/1/CGL_1_C>

use crate::geom::*;

/// 点と線分の位置関係
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CCW {
    /// 点が線分と同一直線上にあり、かつ、点が線分の方向に対して後ろにある。
    ONLINE_BACK,
    /// 点が線分に対して、半時計回り方向にある(左側)。
    COUNTER_CLOCKWISE,
    /// 点が線分上にある。
    ON_SEGMENT,
    /// 点が線分に対して、時計回り方向にある(右側)。
    CLOCKWISE,
    /// 点が線分と同一直線上にあり、かつ、点が線分の方向に対して前にある。
    ONLINE_FRONT,
}

impl CCW {
    /// `ONLINE_BACK`ならば`true`を返す。
    pub fn online_back(self) -> bool {
        self == Self::ONLINE_BACK
    }
    /// `COUNTER_CLOCKWISE`ならば`true`を返す。
    pub fn counter_clockwise(self) -> bool {
        self == Self::COUNTER_CLOCKWISE
    }
    /// `ON_SEGMENT`ならば`true`を返す。
    pub fn on_segment(self) -> bool {
        self == Self::ON_SEGMENT
    }
    /// `CLOCKWISE`ならば`true`を返す。
    pub fn clockwise(self) -> bool {
        self == Self::CLOCKWISE
    }
    /// `ONLINE_FRONT`ならば`true`を返す。
    pub fn online_front(self) -> bool {
        self == Self::ONLINE_FRONT
    }

    /// `ONLINE_BACK`または`COUNTER_CLOCKWISE`ならば`-1`を返す。
    ///
    /// `ON_SEGMENT`ならば`0`を返す。
    ///
    /// `CLOCKWISE`または`ONLINE_FRONT`ならば`1`を返す。
    pub fn to_value(self) -> i32 {
        use self::CCW::*;
        match self {
            ONLINE_BACK | COUNTER_CLOCKWISE => -1,
            ON_SEGMENT => 0,
            CLOCKWISE | ONLINE_FRONT => 1,
        }
    }
}

/// `p0`から`p1`に向かう線分に対して、点`p2`の位置関係を求める。
pub fn ccw(p0: Vector, p1: Vector, p2: Vector, eps: Eps) -> CCW {
    use self::CCW::*;
    let cr = (p1 - p0).cross(p2 - p0);
    let d = (p1 - p0).dot(p2 - p0);

    if eps.eq(cr, 0.0) {
        if eps.lt(d, 0.0) {
            ONLINE_BACK
        } else if eps.gt((p2 - p0).abs(), (p1 - p0).abs()) {
            ONLINE_FRONT
        } else {
            ON_SEGMENT
        }
    } else if eps.gt(cr, 0.0) {
        COUNTER_CLOCKWISE
    } else {
        CLOCKWISE
    }
}
