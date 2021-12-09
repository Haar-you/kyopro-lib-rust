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
    pub fn to_value(self) -> i32 {
        use self::CCW::*;
        match self {
            ONLINE_BACK | COUNTER_CLOCKWISE => -1,
            ON_SEGMENT => 0,
            CLOCKWISE | ONLINE_FRONT => 1,
        }
    }
}

pub fn ccw<T: Eps>(p0: Vector<T>, p1: Vector<T>, p2: Vector<T>) -> CCW {
    use self::CCW::*;
    let cr = (p1 - p0).cross(p2 - p0);
    let d = (p1 - p0).dot(p2 - p0);

    if cr == T::from(0.0) {
        if d < T::from(0.0) {
            ONLINE_BACK
        } else if (p2 - p0).abs() > (p1 - p0).abs() {
            ONLINE_FRONT
        } else {
            ON_SEGMENT
        }
    } else if cr > T::from(0.0) {
        COUNTER_CLOCKWISE
    } else {
        CLOCKWISE
    }
}
