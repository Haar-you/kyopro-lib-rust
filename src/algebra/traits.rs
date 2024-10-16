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

pub trait Commutative {}
pub trait Associative {}
pub trait Idempotence {}

pub trait Semigroup: BinaryOp + Associative {}
impl<T: BinaryOp + Associative> Semigroup for T {}

pub trait Monoid: Semigroup + Identity {}
impl<T: Semigroup + Identity> Monoid for T {}

pub trait AbelianMonoid: Monoid + Commutative {}
impl<T: Monoid + Commutative> AbelianMonoid for T {}

pub trait Group: Monoid + Inverse {}
impl<T: Monoid + Inverse> Group for T {}

pub trait AbelianGroup: Group + Commutative {}
impl<T: Group + Commutative> AbelianGroup for T {}

pub trait Times<T: Clone>: BinaryOp<Output = T> + Identity {
    fn times(&self, mut a: Self::Output, mut n: u64) -> Self::Output {
        let mut ret = self.id();

        while n > 0 {
            if n & 1 == 1 {
                ret = self.op(ret, a.clone());
            }
            a = self.op(a.clone(), a);
            n >>= 1;
        }

        ret
    }
}
impl<T: Clone, A: BinaryOp<Output = T> + Identity> Times<T> for A {}
