#[macro_export]
macro_rules! impl_algebra {
    ($t:ty, binaryop: $f:expr) => {
        impl BinaryOp for $t {
            fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
                $f(&self, a, b)
            }
        }
    };
    ($t:ty, identity: $f:expr) => {
        impl Identity for $t {
            fn id(&self) -> Self::Output {
                $f(&self)
            }
        }
    };
    ($t:ty, inverse: $f:expr) => {
        impl Inverse for $t {
            fn inv(&self, a: Self::Output) -> Self::Output {
                $f(self, a)
            }
        }
    };
    ($t:ty, commutative: $f:expr) => {
        impl Commutative for $t {}
    };
    ($t:ty, associative: $f:expr) => {
        impl Associative for $t {}
    };
    ($t:ty, idempotence: $f:expr) => {
        impl Idempotence for $t {}
    };

    ($t:ty, $($s:ident: $f:expr),+) => {
        $(
            impl_algebra!($t, $s: $f);
        )+
    }
}
