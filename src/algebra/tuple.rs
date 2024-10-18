pub use crate::algebra::traits::*;

macro_rules! impl_tuple {
    ($a:tt; $($t:tt),*; $($i:tt),*) => {
        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
        pub struct $a<$($t),*>($(pub $t),*);

        impl<$($t:Set),*> Set for $a<$($t),*> {
            type Element = ($($t::Element),*);
        }

        impl<$($t:BinaryOp),*> BinaryOp for $a<$($t),*> {
            fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                ($(self.$i.op(a.$i, b.$i)),*)
            }
        }

        impl<$($t:Identity),*> Identity for $a<$($t),*> {
            fn id(&self) -> Self::Element {
                ($(self.$i.id()),*)
            }
        }

        impl<$($t:Inverse),*> Inverse for $a<$($t),*> {
            fn inv(&self, a: Self::Element) -> Self::Element {
                ($(self.$i.inv(a.$i)),*)
            }
        }

        impl<$($t:Associative),*> Associative for $a<$($t),*> {}
        impl<$($t:Commutative),*> Commutative for $a<$($t),*> {}
        impl<$($t:Idempotence),*> Idempotence for $a<$($t),*> {}
    };
}

impl_tuple!(Tuple2; T0, T1; 0, 1);
impl_tuple!(Tuple3; T0, T1, T2; 0, 1, 2);
impl_tuple!(Tuple4; T0, T1, T2, T3; 0, 1, 2, 3);
impl_tuple!(Tuple5; T0, T1, T2, T3, T4; 0, 1, 2, 3, 4);
