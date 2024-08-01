use crate::geom::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CCW {
    ONLINE_BACK,
    COUNTER_CLOCKWISE,
    ON_SEGMENT,
    CLOCKWISE,
    ONLINE_FRONT,
}

impl CCW {
    pub fn online_back(self) -> bool {
        self == Self::ONLINE_BACK
    }
    pub fn counter_clockwise(self) -> bool {
        self == Self::COUNTER_CLOCKWISE
    }
    pub fn on_segment(self) -> bool {
        self == Self::ON_SEGMENT
    }
    pub fn clockwise(self) -> bool {
        self == Self::CLOCKWISE
    }
    pub fn online_front(self) -> bool {
        self == Self::ONLINE_FRONT
    }

    pub fn to_value(self) -> i32 {
        use self::CCW::*;
        match self {
            ONLINE_BACK | COUNTER_CLOCKWISE => -1,
            ON_SEGMENT => 0,
            CLOCKWISE | ONLINE_FRONT => 1,
        }
    }
}

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
