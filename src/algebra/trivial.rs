//! 自明な群
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 自明な群
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Trivial;

impl_algebra!(Trivial; op: |_, _| Self; id: Self; inv: |_| Self; assoc; commu; idem;);
