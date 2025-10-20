//! 円と直線の位置関係

use crate::geom::{dist_line_point::*, *};

/// 円と直線の位置関係
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum IntersectCircleLine {
    /// 直線が円の外側にある
    OUTSIDE,
    /// 直線が円に接している
    TANGENT,
    /// 直線と円が交わっている
    CROSSED,
}

impl IntersectCircleLine {
    /// `OUTSIDE`かを判定
    pub fn outside(self) -> bool {
        self == Self::OUTSIDE
    }
    /// `TANGENT`かを判定
    pub fn tangent(self) -> bool {
        self == Self::TANGENT
    }
    /// `CROSSED`かを判定
    pub fn crossed(self) -> bool {
        self == Self::CROSSED
    }
}

/// 円と直線の位置関係と交点を求める
pub fn intersect_circle_line(c: Circle, l: Line, eps: Eps) -> (IntersectCircleLine, Vec<Vector>) {
    use self::IntersectCircleLine::*;

    let d = dist_line_point(l, c.center);

    if eps.gt(d, c.radius) {
        return (OUTSIDE, vec![]);
    }

    let n = l.normal();
    let b = l.from + l.diff() * n.cross(c.center + n - l.from) / n.cross(l.diff());

    if eps.eq(d, c.radius) {
        (TANGENT, vec![b])
    } else {
        let a = c.radius.mul_add(c.radius, -(d * d)).sqrt();
        (CROSSED, vec![b + l.unit() * a, b - l.unit() * a])
    }
}
