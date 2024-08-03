//! 線分と点の距離

use crate::geom::*;

pub fn dist_segment_point(l: Line, p: Vector) -> f64 {
    if l.diff().dot(p - l.from) < 0.0 {
        (p - l.from).abs()
    } else if -l.diff().dot(p - l.to) < 0.0 {
        (p - l.to).abs()
    } else {
        l.diff().cross(p - l.from).abs() / l.abs()
    }
}
