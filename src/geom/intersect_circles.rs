//! 2つの円の位置関係

use crate::geom::*;

/// 2つの円の位置関係
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum IntersectCircles {
    /// 2つの円が同じである
    SAME,
    /// 一方の円がもう一方の円の内部にある
    INSIDE,
    /// 一方の円がもう一方の円に内部で接している
    INSCRIBED,
    /// 2つの円が交差している
    INTERSECTED,
    /// 一方の円がもう一方の円に外部で接している
    CIRCUMSCRIBED,
    /// 一方の円がもう一方の円の外部にある
    OUTSIDE,
}

impl IntersectCircles {
    /// `SAME`かを判定
    pub fn same(self) -> bool {
        self == Self::SAME
    }
    /// `INSIDE`かを判定
    pub fn inside(self) -> bool {
        self == Self::INSIDE
    }
    /// `INSCRIBED`かを判定
    pub fn inscribed(self) -> bool {
        self == Self::INSCRIBED
    }
    /// `INTERSECTED`かを判定
    pub fn intersected(self) -> bool {
        self == Self::INTERSECTED
    }
    /// `CIRCUMSCRIBED`かを判定
    pub fn circumscribed(self) -> bool {
        self == Self::CIRCUMSCRIBED
    }
    /// `OUTSIDE`かを判定
    pub fn outside(self) -> bool {
        self == Self::OUTSIDE
    }

    /// 2つの円の共通接線の個数を返す。但し、`SAME`のときは`None`を返す。
    pub fn num_common_tangent(self) -> Option<u32> {
        use self::IntersectCircles::*;
        match self {
            SAME => None,
            INSIDE => Some(0),
            INSCRIBED => Some(1),
            INTERSECTED => Some(2),
            CIRCUMSCRIBED => Some(3),
            OUTSIDE => Some(4),
        }
    }
}

/// 2つの円の位置関係と交点を求める
pub fn intersect_circles(a: Circle, b: Circle, eps: Eps) -> (IntersectCircles, Vec<Vector>) {
    use self::IntersectCircles::*;

    let d = (a.center - b.center).abs();
    let x = ((a.radius * a.radius + d * d - b.radius * b.radius) / (2.0 * d * a.radius)).acos();
    let t = (b.center.1 - a.center.1).atan2(b.center.0 - a.center.0);

    if eps.eq(a.radius + b.radius, d) {
        (CIRCUMSCRIBED, vec![a.center + Vector::polar(a.radius, t)])
    } else if eps.eq((a.radius - b.radius).abs(), d) {
        (INSCRIBED, vec![a.center + Vector::polar(a.radius, t)])
    } else if eps.gt(a.radius + b.radius, d) && eps.gt(d, (a.radius - b.radius).abs()) {
        (
            INTERSECTED,
            vec![
                a.center + Vector::polar(a.radius, t + x),
                a.center + Vector::polar(a.radius, t - x),
            ],
        )
    } else if eps.lt(a.radius + b.radius, d) {
        (OUTSIDE, vec![])
    } else if eps.gt((a.radius - b.radius).abs(), d) {
        (INSIDE, vec![])
    } else {
        (SAME, vec![])
    }
}
