use crate::geom::*;

pub fn dist_segment_point<T: Eps>(l: Line<T>, p: Vector<T>) -> T {
    if l.diff().dot(p - l.from) < T::from(0.0) {
        (p - l.from).abs()
    } else if -l.diff().dot(p - l.to) < T::from(0.0) {
        (p - l.to).abs()
    } else {
        l.diff().cross(p - l.from).abs() / l.abs()
    }
}
