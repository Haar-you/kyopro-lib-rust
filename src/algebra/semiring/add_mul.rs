//! AddとMulの半環
pub use crate::algebra::semiring::Semiring;

/// AddとMulの半環
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct AddMulSemiring<T>(pub T);

macro_rules! impl_semiring {
    ($($t:ty),*) => {
        $(impl Semiring for AddMulSemiring<$t> {
            fn zero() -> Self {
                Self(0)
            }
            fn one() -> Self {
                Self(1)
            }
            fn add(self, b: Self) -> Self {
                Self(self.0 + b.0)
            }
            fn mul(self, b: Self) -> Self {
                Self(self.0 * b.0)
            }
        })*
    };
}

impl_semiring!(u8, u16, u32, u64, u128, usize);
