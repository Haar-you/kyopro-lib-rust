//! `impl_algebra!`

/// [`Set`](crate::algebra::traits::Set), [`BinaryOp`](crate::algebra::traits::BinaryOp),
/// [`Identity`](crate::algebra::traits::Identity), [`Inverse`](crate::algebra::traits::Inverse),
/// [`Commutative`](crate::algebra::traits::Commutative), [`Associative`](crate::algebra::traits::Associative),
/// [`Idempotence`](crate::algebra::traits::Idempotence)を実装する。
#[macro_export]
#[doc(hidden)]
macro_rules! impl_algebra {
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?;) => {};
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; set: $f:ty; $($rest:tt)*) => {
        impl <$($bound)*> Set for $t $(where $($where)*)? {
            type Element = $f;
        }
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; op: $f:expr; $($rest:tt)*) => {
        impl <$($bound)*> BinaryOp for $t $(where $($where)*)? {
            fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                $f(self, a, b)
            }
        }
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; id: $f:expr, $g:expr; $($rest:tt)*) => {
        impl <$($bound)*> Identity for $t $(where $($where)*)? {
            fn id(&self) -> Self::Element {
                $f(self)
            }
            fn is_id(&self, a: &Self::Element) -> bool {
                $g(self, a)
            }
        }
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; inv: $f:expr; $($rest:tt)*) => {
        impl <$($bound)*> Inverse for $t $(where $($where)*)? {
            fn inv(&self, a: Self::Element) -> Self::Element {
                $f(self, a)
            }
        }
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; commu; $($rest:tt)*) => {
        impl <$($bound)*> Commutative for $t $(where $($where)*)? {}
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; assoc; $($rest:tt)*) => {
        impl <$($bound)*> Associative for $t $(where $($where)*)? {}
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };
    (@inner {$($bound:tt)*} $t:ty $({$($where:tt)*})?; idem; $($rest:tt)*) => {
        impl <$($bound)*> Idempotence for $t $(where $($where)*)? {}
        impl_algebra!(@inner {$($bound)*} $t $({$($where)*})?; $($rest)*);
    };

    ($({$($bound:tt)*})? $t:ty $(where {$($where:tt)*})?; $($rest:tt)*) => {
        impl_algebra!(@inner {$($($bound)*)?} $t $({$($where)*})?; $($rest)*);
    };
}
