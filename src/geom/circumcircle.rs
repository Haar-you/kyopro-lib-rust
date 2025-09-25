//! 三角形の外接円

use crate::geom::*;

/// 三角形(△abc)の外接円を求める
pub fn circumcircle(a: Vector, b: Vector, c: Vector) -> Circle {
    let a_ = (b - c).abs_sq();
    let b_ = (a - c).abs_sq();
    let c_ = (a - b).abs_sq();
    let s = a_ + b_ + c_;
    let a_ = a_ * a_.mul_add(-2.0, s);
    let b_ = b_ * b_.mul_add(-2.0, s);
    let c_ = c_ * c_.mul_add(-2.0, s);
    let s = a_ + b_ + c_;
    let a_ = a_ / s;
    let b_ = b_ / s;
    let c_ = c_ / s;
    let center = a * a_ + b * b_ + c * c_;

    Circle::new(center, (center - a).abs())
}
