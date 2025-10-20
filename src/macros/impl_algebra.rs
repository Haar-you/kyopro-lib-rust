//! `impl_algebra!`

/// [`Set`](crate::algebra::traits::Set), [`BinaryOp`](crate::algebra::traits::BinaryOp),
/// [`Identity`](crate::algebra::traits::Identity), [`Inverse`](crate::algebra::traits::Inverse),
/// [`Commutative`](crate::algebra::traits::Commutative), [`Associative`](crate::algebra::traits::Associative),
/// [`Idempotence`](crate::algebra::traits::Idempotence)を実装する。
#[macro_export]
#[doc(hidden)]
macro_rules! impl_algebra {
    (@inner [$($bound:tt)*]; $t:ty;) => {
        impl <$($bound)*> Set for $t {}
    };
    (@inner [$($bound:tt)*]; $t:ty; op: $f:expr; $($rest:tt)*) => {
        impl <$($bound)*> BinaryOp for $t {
            fn op(self, b: Self) -> Self {
                $f(self, b)
            }
        }
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };
    (@inner [$($bound:tt)*]; $t:ty; id: $f:expr; $($rest:tt)*) => {
        impl <$($bound)*> Identity for $t {
            fn id() -> Self {
                $f
            }
        }
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };
    (@inner [$($bound:tt)*]; $t:ty; inv: $f:expr; $($rest:tt)*) => {
        impl <$($bound)*> Inverse for $t {
            fn inv(self) -> Self {
                $f(self)
            }
        }
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };
    (@inner [$($bound:tt)*]; $t:ty; commu; $($rest:tt)*) => {
        impl <$($bound)*> Commutative for $t {}
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };
    (@inner [$($bound:tt)*]; $t:ty; assoc; $($rest:tt)*) => {
        impl <$($bound)*> Associative for $t {}
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };
    (@inner [$($bound:tt)*]; $t:ty; idem; $($rest:tt)*) => {
        impl <$($bound)*> Idempotence for $t {}
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };

    ([$($bound:tt)*]; $t:ty; $($rest:tt)*) => {
        impl_algebra!(@inner [$($bound)*]; $t; $($rest)*);
    };
    ($t:ty; $($rest:tt)*) => {
        impl_algebra!(@inner []; $t; $($rest)*);
    };
}
