use crate::geom::*;

pub fn dist_line_point<T: Eps>(l: Line<T>, p: Vector<T>) -> T {
    l.diff().cross(p - l.from).abs() / l.abs()
}
