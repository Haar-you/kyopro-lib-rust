#![allow(clippy::wrong_self_convention)]
#![allow(clippy::nonminimal_bool)]
#![allow(missing_docs)]

pub trait IsNoneOr<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsNoneOr<T> for Option<T> {
    #[inline]
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        !self.is_some_and(|a| !f(a))
    }
}
