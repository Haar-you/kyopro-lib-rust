#[macro_export]
macro_rules! impl_algebra {
    ($t:ty, op: $f:expr) => {
        impl BinaryOp for $t {
            fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
                $f(&self, a, b)
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
    ($t:ty, inv: $f:expr) => {
        impl Inverse for $t {
            fn inv(&self, a: Self::Output) -> Self::Output {
                $f(self, a)
            }
        }
    };
    ($t:ty, commu: $f:expr) => {
        impl Commutative for $t {}
    };
    ($t:ty, assoc: $f:expr) => {
        impl Associative for $t {}
    };
    ($t:ty, idem: $f:expr) => {
        impl Idempotence for $t {}
    };

    ($t:ty, $($s:ident: $f:expr),+) => {
        $(
            impl_algebra!($t, $s: $f);
        )+
    }
}
