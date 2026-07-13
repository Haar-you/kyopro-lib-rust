//! 代数的構造$A$の$n$個の直積$A^n$
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 代数的構造$A$の$n$個の直積$A^n$
pub struct Array<A>(pub A, pub usize);

impl_algebra!({A: Set} Array<A>; set: Vec<A::Element>;);
impl_algebra!({A: BinaryOp} Array<A>; op: |s: &Self, a: Self::Element, b| a.into_iter().zip(b).map(|(a, b)| s.0.op(a, b)).collect(););
impl_algebra!({A: Identity} Array<A> where {A::Element: Clone}; id: |s: &Self| vec![s.0.id(); s.1];);
impl_algebra!({A: Inverse} Array<A>; inv: |s: &Self, a: Self::Element| a.into_iter().map(|a| s.0.inv(a)).collect(););
impl_algebra!({A: Associative} Array<A>; assoc;);
impl_algebra!({A: Commutative} Array<A>; commu;);
impl_algebra!({A: Idempotence} Array<A>; idem;);
