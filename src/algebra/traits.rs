pub trait BinaryOp<T> {
    fn op(&self, _: T, _: T) -> T;
}

pub trait Identity<T> {
    fn id(&self) -> T;
}

pub trait Inverse<T> {
    fn inv(&self, _: T) -> T;
}

pub trait Semigroup<T>: BinaryOp<T> {}
pub trait Monoid<T>: BinaryOp<T> + Identity<T> {}
pub trait Group<T>: BinaryOp<T> + Identity<T> + Inverse<T> {}
