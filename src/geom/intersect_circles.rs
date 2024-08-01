use crate::geom::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectCircles {
    SAME,
    INSIDE,
    INSCRIBED,
    INTERSECTED,
    CIRCUMSCRIBED,
    OUTSIDE,
}

impl IntersectCircles {
    pub fn same(self) -> bool {
        self == Self::SAME
    }
    pub fn inside(self) -> bool {
        self == Self::INSIDE
    }
    pub fn inscribed(self) -> bool {
        self == Self::INSCRIBED
    }
    pub fn intersected(self) -> bool {
        self == Self::INTERSECTED
    }
    pub fn circumscribed(self) -> bool {
        self == Self::CIRCUMSCRIBED
    }
    pub fn outside(self) -> bool {
        self == Self::OUTSIDE
    }

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
