//! 凸多角形の判定

use crate::geom::{ccw::*, *};

/// 多角形`ps`が凸多角形であれば`true`を返す。
pub fn is_convex(ps: &[Vector], eps: Eps) -> bool {
    let n = ps.len();
    let mut temp = None;

    for i in 0..n {
        let c = ccw(ps[i], ps[(i + 1) % n], ps[(i + 2) % n], eps);

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
