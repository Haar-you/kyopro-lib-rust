//! 最小値とその個数の総和
use crate::algebra::traits::*;
use crate::num::one_zero::Zero;
use std::{cmp::Ordering, ops::Add};

/// 最小値とその個数の総和
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct MinCount<T, U>(pub Option<T>, pub U);
impl<T, U> Set for MinCount<T, U> {}
impl<T: Ord, U: Add<Output = U>> BinaryOp for MinCount<T, U> {
    fn op(self, b: Self) -> Self {
        match (self, b) {
            (Self(None, ca), Self(None, cb)) => Self(None, ca + cb),
            (Self(None, _), a) => a,
            (b, Self(None, _)) => b,
            (Self(Some(a), ca), Self(Some(b), cb)) => match a.cmp(&b) {
                Ordering::Equal => Self(Some(a), ca + cb),
                Ordering::Less => Self(Some(a), ca),
                Ordering::Greater => Self(Some(b), cb),
            },
        }
    }
}
impl<T, U: Zero> Identity for MinCount<T, U> {
    fn id() -> Self {
        Self(None, U::zero())
    }
}
impl<T, U> Associative for MinCount<T, U> {}
impl<T, U> Commutative for MinCount<T, U> {}
