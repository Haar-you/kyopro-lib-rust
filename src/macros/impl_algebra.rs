//! `impl_algebra!`

#[macro_export]
macro_rules! impl_algebra {
    (@bound $t:ty, op: $f:expr; $($bound:tt)*) => {
        impl $($bound)* BinaryOp for $t {
            fn op(&self, a: Self::Element, b: Self::Element) -> Self::Element {
                $f(&self, a, b)
            }
        }
    };
    (@bound $t:ty, id: $f:expr; $($bound:tt)*) => {
        impl $($bound)* Identity for $t {
            fn id(&self) -> Self::Element {
                $f(&self)
            }
        }
    };
    (@bound $t:ty, inv: $f:expr; $($bound:tt)*) => {
        impl $($bound)* Inverse for $t {
            fn inv(&self, a: Self::Element) -> Self::Element {
                $f(self, a)
            }
        }
    };
    (@bound $t:ty, commu: $f:expr; $($bound:tt)*) => {impl $($bound)* Commutative for $t {}};
    (@bound $t:ty, assoc: $f:expr; $($bound:tt)*) => {impl $($bound)* Associative for $t {}};
    (@bound $t:ty, idem: $f:expr; $($bound:tt)*) => {impl $($bound)* Idempotence for $t {}};
    (@bound $t:ty, set: $elem:ty; $($bound:tt)*) => {
        impl $($bound)* Set for $t {
            type Element = $elem;
        }
    };

    (<const $a:ident : $b:ty>; $t:ty, set: $elem:ty, $($s:ident: $f:expr),+) => {
        impl_algebra!(@bound $t, set: $elem; <const $a: $b>);
        $(impl_algebra!(@bound $t, $s: $f; <const $a: $b>);)+
    };
    (<$a:ident>; $t:ty, set: $elem:ty, $($s:ident: $f:expr),+) => {
        impl_algebra!(@bound $t, set: $elem; <$a>);
        $(impl_algebra!(@bound $t, $s: $f; <$a>);)+
    };
    ($t:ty, set: $elem:ty, $($s:ident: $f:expr),+) => {
        impl_algebra!(@bound $t, set: $elem;);
        $(impl_algebra!(@bound $t, $s: $f;);)+
    };
}
