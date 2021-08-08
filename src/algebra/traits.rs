pub trait AlgeStruct {
    type Output;
}

pub trait BinaryOp: AlgeStruct {
    fn op(&self, _: Self::Output, _: Self::Output) -> Self::Output;
}

pub trait Identity: AlgeStruct {
    fn id(&self) -> Self::Output;
}

pub trait Inverse: AlgeStruct {
    fn inv(&self, _: Self::Output) -> Self::Output;
}

pub trait Semigroup: BinaryOp {}
impl<T: BinaryOp> Semigroup for T {}

pub trait Monoid: Semigroup + Identity {}
impl<T: Semigroup + Identity> Monoid for T {}

pub trait Group: Monoid + Inverse {}
impl<T: Monoid + Inverse> Group for T {}
