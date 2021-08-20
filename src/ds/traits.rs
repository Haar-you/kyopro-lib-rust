pub trait Foldable<T> {
    fn fold(&self, l: usize, r: usize) -> T;
}

/// 列上の一点に値を代入
pub trait Assignable<T> {
    fn assign(&mut self, i: usize, value: T);
}

/// 列上の一点に演算を適用した値を代入
pub trait Updatable<T>: Assignable<T> {
    fn update(&mut self, i: usize, value: T);
}

pub trait RangeUpdatable<T> {
    fn range_update(&mut self, l: usize, r: usize, value: T);
}
