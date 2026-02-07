//! 単位元を追加したモノイド

pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 単位元を追加したモノイド
pub struct AppendId<M>(pub M);

impl_algebra!({M: Set} AppendId<M>; set: Option<M::Element>; id: |_| None;);
impl_algebra!({M: BinaryOp} AppendId<M>;
    op: |s: &Self, a, b|
        match (a, b) {
            (Some(a), Some(b)) => Some(s.0.op(a, b)),
            (a, None) => a,
            (None, b) => b,
    };
);
impl_algebra!({M: Associative} AppendId<M>; assoc;);
impl_algebra!({M: Commutative} AppendId<M>; commu;);
impl_algebra!({M: Idempotence} AppendId<M>; idem;);
