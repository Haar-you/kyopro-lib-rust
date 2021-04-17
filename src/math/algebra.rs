pub trait BinaryOp<T> {
    fn op(&self, _: T, _: T) -> T;
}

pub trait Identity<T> {
    fn id(&self) -> T;
}

pub trait Inverse<T> {
    fn inv(&self, _: T) -> T;
}



#[derive(Clone)]
pub struct Monoid<T> {
    id_: T,
    op_: fn(T, T) -> T
}

impl<T> Monoid<T> {
    pub fn new(id_: T, op_: fn(T, T) -> T) -> Self {
        Monoid { id_, op_ }
    }
}

impl<T> BinaryOp<T> for Monoid<T> {
    fn op(&self, x: T, y: T) -> T {
        (self.op_)(x, y)
    }
}

impl<T: Clone> Identity<T> for Monoid<T> {
    fn id(&self) -> T {
        self.id_.clone()
    }
}




pub struct Group<T> {
    id_: T,
    op_: fn(T, T) -> T,
    inv_: fn(T) -> T
}

impl<T> Group<T> {
    pub fn new(id_: T, op_: fn(T, T) -> T, inv_: fn(T) -> T) -> Self {
        Group { id_, op_, inv_ }
    }
}

impl<T> BinaryOp<T> for Group<T> {
    fn op(&self, x: T, y: T) -> T {
        (self.op_)(x, y)
    }
}

impl<T: Clone> Identity<T> for Group<T> {
    fn id(&self) -> T {
        self.id_.clone()
    }
}

impl<T> Inverse<T> for Group<T> {
    fn inv(&self, x: T) -> T {
        (self.inv_)(x)
    }
}
