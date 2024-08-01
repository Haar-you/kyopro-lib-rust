use crate::geom::*;

pub fn circumcircle(a: Vector, b: Vector, c: Vector) -> Circle {
    let a_ = (b - c).abs_sq();
    let b_ = (a - c).abs_sq();
    let c_ = (a - b).abs_sq();
    let s = a_ + b_ + c_;
    let a_ = a_ * (s - a_ * 2.0);
    let b_ = b_ * (s - b_ * 2.0);
    let c_ = c_ * (s - c_ * 2.0);
    let s = a_ + b_ + c_;
    let a_ = a_ / s;
    let b_ = b_ / s;
    let c_ = c_ / s;
    let center = a * a_ + b * b_ + c * c_;

    Circle::new(center, (center - a).abs())
}
