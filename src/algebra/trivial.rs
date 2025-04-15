//! 自明な群
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 自明な群
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Trivial;

impl_algebra!(Trivial; set: (); op: |_, _, _| (); id: |_| (); inv: |_, _| (); assoc; commu; idem;);
