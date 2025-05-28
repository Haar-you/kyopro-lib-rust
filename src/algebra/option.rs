pub use crate::algebra::traits::*;

impl<T> Set for Option<T> {}
impl<T: BinaryOp> BinaryOp for Option<T> {
    fn op(self, other: Self) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.op(b)),
            (a, None) => a,
            (None, b) => b,
        }
    }
}
impl<T> Identity for Option<T> {
    fn id() -> Self {
        None
    }
}
impl<T: Associative> Associative for Option<T> {}
impl<T: Commutative> Commutative for Option<T> {}
impl<T: Idempotence> Idempotence for Option<T> {}
