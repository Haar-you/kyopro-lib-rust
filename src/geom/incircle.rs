//! 三角形の内接円

use crate::geom::*;

pub fn incircle(a: Vector, b: Vector, c: Vector) -> Circle {
    let a_ = (b - c).abs();
    let b_ = (a - c).abs();
    let c_ = (a - b).abs();
    let s = (a_ + b_ + c_) / 2.0;

    Circle::new(
        (a * a_ + b * b_ + c * c_) / (a_ + b_ + c_),
        ((s - a_) * (s - b_) * (s - c_) / s).sqrt(),
    )
}
