pub trait Set {
    type Element;
}

pub trait BinaryOp: Set {
    fn op(&self, _: Self::Element, _: Self::Element) -> Self::Element;
}

pub trait Identity: Set {
    fn id(&self) -> Self::Element;
}

pub trait Inverse: Set {
    fn inv(&self, _: Self::Element) -> Self::Element;
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

pub trait Times: BinaryOp + Identity
where
    Self::Element: Clone,
{
    fn times(&self, mut a: Self::Element, mut n: u64) -> Self::Element {
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
impl<A: BinaryOp + Identity> Times for A where Self::Element: Clone {}
