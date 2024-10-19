//! 直線と線分の位置関係

use crate::geom::*;
use std::cmp::Ordering::*;

/// 直線と線分の位置関係
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectLineSegment {
    /// 線分が直線の左側にある
    LEFTSIDE,
    /// 線分が直線の右側にある
    RIGHTSIDE,
    /// 線分が直線上にある
    OVERLAPPED,
    /// 線分が直線と交差している
    CROSSED,
}

impl IntersectLineSegment {
    /// `LEFTSIDE`かを判定
    pub fn leftside(self) -> bool {
        self == Self::LEFTSIDE
    }
    /// `RIGHTSIDE`かを判定
    pub fn rightside(self) -> bool {
        self == Self::RIGHTSIDE
    }
    /// `OVERLAPPED`かを判定
    pub fn overlapped(self) -> bool {
        self == Self::OVERLAPPED
    }
    /// `CROSSED`かを判定
    pub fn crossed(self) -> bool {
        self == Self::CROSSED
    }
}

/// 直線と線分の位置関係と交点を求める
pub fn intersect_line_segment(
    l: Line,
    s: Line,
    eps: Eps,
) -> (IntersectLineSegment, Option<Vector>) {
    use self::IntersectLineSegment::*;

    let a = l.diff().cross(s.from - l.from);
    let b = l.diff().cross(s.to - l.from);

    match (eps.partial_cmp(a, 0.0), eps.partial_cmp(b, 0.0)) {
        (Some(Equal), Some(Equal)) => (OVERLAPPED, None),
        (Some(Less), Some(Less)) => (RIGHTSIDE, None),
        (Some(Greater), Some(Greater)) => (LEFTSIDE, None),
        _ => (
            CROSSED,
            Some(s.from + s.diff() * l.diff().cross(l.from - s.from) / l.cross(s)),
        ),
    }
}
