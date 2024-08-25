#[macro_export]
macro_rules! impl_algebra {
    (@bound $bound:tt, $t:ty, op: $f:expr) => {
        impl<$bound> BinaryOp for $t {
            fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
                $f(&self, a, b)
            }
        }
    };
    ($t:ty, op: $f:expr) => {
        impl BinaryOp for $t {
            fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
                $f(&self, a, b)
            }
        }
    };

    (@bound $bound:tt, $t:ty, id: $f:expr) => {
        impl<$bound> Identity for $t {
            fn id(&self) -> Self::Output {
                $f(&self)
            }
        }
    };
    ($t:ty, id: $f:expr) => {
        impl Identity for $t {
            fn id(&self) -> Self::Output {
                $f(&self)
            }
        }
    };

    (@bound $bound:tt, $t:ty, inv: $f:expr) => {
        impl<$bound> Inverse for $t {
            fn inv(&self, a: Self::Output) -> Self::Output {
                $f(self, a)
            }
        }
    };
    ($t:ty, inv: $f:expr) => {
        impl Inverse for $t {
            fn inv(&self, a: Self::Output) -> Self::Output {
                $f(self, a)
            }
        }
    };

    (@bound $bound:tt, $t:ty, commu: $f:expr) => {impl<$bound> Commutative for $t {}};
    ($t:ty, commu: $f:expr) => {impl Commutative for $t {}};

    (@bound $bound:tt, $t:ty, assoc: $f:expr) => {impl<$bound> Associative for $t {}};
    ($t:ty, assoc: $f:expr) => {impl Associative for $t {}};

    (@bound $bound:tt, $t:ty, idem: $f:expr) => {impl<$bound> Idempotence for $t {}};
    ($t:ty, idem: $f:expr) => {impl Idempotence for $t {}};

    ($t:ty, $($s:ident: $f:expr),+) => {
        $(impl_algebra!($t, $s: $f);)+
    };
    ($trait:tt; $t:ty, $($s:ident: $f:expr),+) => {
        $(impl_algebra!(@bound $trait, $t, $s: $f);)+
    };
}
