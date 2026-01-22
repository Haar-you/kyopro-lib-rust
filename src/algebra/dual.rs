//! 演算の順序を逆にした代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 演算の順序を逆にした代数的構造
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dual<S>(pub S);

impl_algebra!({S: Set} Dual<S>; set: S::Element;);
impl_algebra!({S: BinaryOp} Dual<S>; op: |s: &Self, a, b| s.0.op(b, a););
impl_algebra!({S: Identity} Dual<S>; id: |s: &Self| s.0.id(););
impl_algebra!({S: Inverse} Dual<S>; inv: |s: &Self, a| s.0.inv(a););
impl_algebra!({S: Commutative} Dual<S>; commu;);
impl_algebra!({S: Associative} Dual<S>; assoc;);
impl_algebra!({S: Idempotence} Dual<S>; idem;);
