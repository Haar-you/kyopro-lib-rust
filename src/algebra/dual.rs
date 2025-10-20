//! 演算の順序を逆にした代数的構造
pub use crate::algebra::traits::*;

/// 演算の順序を逆にした代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dual<S>(pub S);

impl<S: Set> Set for Dual<S> {}

impl<S: BinaryOp> BinaryOp for Dual<S> {
    fn op(self, b: Self) -> Self {
        Self(b.0.op(self.0))
    }
}

impl<S: Identity> Identity for Dual<S> {
    fn id() -> Self {
        Self(S::id())
    }
}

impl<S: Inverse> Inverse for Dual<S> {
    fn inv(self) -> Self {
        Self(self.0.inv())
    }
}

impl<S: Commutative> Commutative for Dual<S> {}
impl<S: Associative> Associative for Dual<S> {}
impl<S: Idempotence> Idempotence for Dual<S> {}
