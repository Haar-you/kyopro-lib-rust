use crate::geom::*;

pub fn dist_line_point(l: Line, p: Vector) -> f64 {
    l.diff().cross(p - l.from).abs() / l.abs()
}
