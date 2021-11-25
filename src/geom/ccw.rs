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
        match self {
            CCW::ONLINE_BACK | CCW::COUNTER_CLOCKWISE => -1,
            CCW::ON_SEGMENT => 0,
            CCW::CLOCKWISE | CCW::ONLINE_FRONT => 1,
        }
    }
}

pub fn ccw<T: Eps>(p0: Vector<T>, p1: Vector<T>, p2: Vector<T>) -> CCW {
    let cr = (p1 - p0).cross(p2 - p0);
    let d = (p1 - p0).dot(p2 - p0);

    if cr == T::from(0.0) {
        if d < T::from(0.0) {
            CCW::ONLINE_BACK
        } else if (p2 - p0).abs() > (p1 - p0).abs() {
            CCW::ONLINE_FRONT
        } else {
            CCW::ON_SEGMENT
        }
    } else if cr > T::from(0.0) {
        CCW::COUNTER_CLOCKWISE
    } else {
        CCW::CLOCKWISE
    }
}
