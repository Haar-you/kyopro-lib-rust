//! 直線と点の距離

use crate::geom::*;

/// 直線と点の距離を求める
pub fn dist_line_point(l: Line, p: Vector) -> f64 {
    l.diff().cross(p - l.from).abs() / l.abs()
}
