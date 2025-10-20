//! 直積
pub use crate::algebra::traits::*;

macro_rules! impl_tuple {
    ($(#[$meta:meta])* $a:ident; $($t:tt),*; $($i:tt),*) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $a<$($t),*>($(pub $t),*);

        impl<$($t:Set),*> Set for $a<$($t),*> {}

        impl<$($t:BinaryOp),*> BinaryOp for $a<$($t),*> {
            fn op(self, b: Self) -> Self {
                Self($($t::op(self.$i, b.$i)),*)
            }
        }

        impl<$($t:Identity),*> Identity for $a<$($t),*> {
            fn id() -> Self {
                Self($($t::id()),*)
            }
        }

        impl<$($t:Inverse),*> Inverse for $a<$($t),*> {
            fn inv(self) -> Self {
                Self($($t::inv(self.$i)),*)
            }
        }

        impl<$($t:Associative),*> Associative for $a<$($t),*> {}
        impl<$($t:Commutative),*> Commutative for $a<$($t),*> {}
        impl<$($t:Idempotence),*> Idempotence for $a<$($t),*> {}
    };
}

impl_tuple!(#[doc = "2つの集合の直積"] Tuple2; T0, T1; 0, 1);
impl_tuple!(#[doc = "3つの集合の直積"] Tuple3; T0, T1, T2; 0, 1, 2);
impl_tuple!(#[doc = "4つの集合の直積"] Tuple4; T0, T1, T2, T3; 0, 1, 2, 3);
impl_tuple!(#[doc = "5つの集合の直積"] Tuple5; T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
