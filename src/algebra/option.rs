//! 単位元を追加したモノイド

pub use crate::algebra::traits::*;

/// 単位元を追加したモノイド
pub struct AppendId<M>(pub M);

impl<M: Set> Set for AppendId<M> {
    type Element = Option<M::Element>;
}

impl<M: BinaryOp> BinaryOp for AppendId<M> {
    fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
        match (a, b) {
            (Some(a), Some(b)) => Some(self.0.op(a, b)),
            (a, None) => a,
            (None, b) => b,
        }
    }
}
impl<M: Set> Identity for AppendId<M> {
    fn id(&self) -> Self::Element {
        None
    }
    fn is_id(&self, a: &Self::Element) -> bool {
        a.is_none()
    }
}
impl<M: Associative> Associative for AppendId<M> {}
impl<M: Commutative> Commutative for AppendId<M> {}
impl<M: Idempotence> Idempotence for AppendId<M> {}
