use crate::geom::*;

pub fn circumcircle<T: Eps>(a: Vector<T>, b: Vector<T>, c: Vector<T>) -> Circle<T> {
    let a_ = (b - c).abs_sq();
    let b_ = (a - c).abs_sq();
    let c_ = (a - b).abs_sq();
    let s = a_ + b_ + c_;
    let a_ = a_ * (s - a_ * T::from(2.0));
    let b_ = b_ * (s - b_ * T::from(2.0));
    let c_ = c_ * (s - c_ * T::from(2.0));
    let s = a_ + b_ + c_;
    let a_ = a_ / s;
    let b_ = b_ / s;
    let c_ = c_ / s;
    let center = a * a_ + b * b_ + c * c_;

    Circle::new(center, (center - a).abs())
}
