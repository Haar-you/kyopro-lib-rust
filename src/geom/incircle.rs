use crate::geom::*;

pub fn incircle<T: Eps>(a: Vector<T>, b: Vector<T>, c: Vector<T>) -> Circle<T> {
    let a_ = (b - c).abs();
    let b_ = (a - c).abs();
    let c_ = (a - b).abs();
    let s = (a_ + b_ + c_) / T::from(2.0);

    Circle::new(
        (a * a_ + b * b_ + c * c_) / (a_ + b_ + c_),
        ((s - a_) * (s - b_) * (s - c_) / s).sqrt(),
    )
}
