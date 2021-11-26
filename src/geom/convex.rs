use crate::geom::{ccw::*, *};

pub fn is_convex<T: Eps>(ps: &[Vector<T>]) -> bool {
    let n = ps.len();
    let mut temp = None;

    for i in 0..n {
        let c = ccw(ps[i], ps[(i + 1) % n], ps[(i + 2) % n]);

        if c == CCW::CLOCKWISE || c == CCW::COUNTER_CLOCKWISE {
            match temp {
                Some(x) if x != c => return false,
                None => temp = Some(c),
                _ => {}
            }
        }
    }

    true
}
