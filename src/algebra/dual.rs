//! 演算の順序を逆にした代数的構造
pub use crate::algebra::traits::*;

/// 演算の順序を逆にした代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Dual<S>(pub S);

impl<S: Set> Dual<S> {
    /// `Dual<S>`を生成する。
    pub fn new(s: S) -> Self {
        Self(s)
    }
}

impl<S: Set> Set for Dual<S> {
    type Element = S::Element;
}

impl<S: BinaryOp> BinaryOp for Dual<S> {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        self.0.op(b, a)
    }
}

impl<S: Identity> Identity for Dual<S> {
    fn id(&self) -> Self::Element {
        self.0.id()
    }
}

impl<S: Commutative> Commutative for Dual<S> {}
impl<S: Associative> Associative for Dual<S> {}
impl<S: Idempotence> Idempotence for Dual<S> {}
