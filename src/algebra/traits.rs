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

pub trait Zero {
    type Output;
    fn zero() -> Self::Output;
}

pub trait One {
    type Output;
    fn one() -> Self::Output;
}

macro_rules! impl_one_zero {
    ($($t:ty),*) => {
        $(
            impl Zero for $t {
                type Output = $t;
                fn zero() -> Self::Output { 0 as $t }
            }

            impl One for $t {
                type Output = $t;
                fn one() -> Self::Output { 1 as $t }
            }
        )*
    }
}

impl_one_zero!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
