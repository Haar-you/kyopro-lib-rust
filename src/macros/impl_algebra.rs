#[macro_export]
macro_rules! impl_algebra {
    (@bound $t:ty, op: $f:expr; $($bound:tt)+) => {
        impl <$($bound)+> BinaryOp for $t {
            fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
                $f(&self, a, b)
            }
        }
    };
    (@nobound $t:ty, op: $f:expr) => {
        impl BinaryOp for $t {
            fn op(&self, a: Self::Output, b: Self::Output) -> Self::Output {
                $f(&self, a, b)
            }
        }
    };

    (@bound $t:ty, id: $f:expr; $($bound:tt)+) => {
        impl<$($bound)+> Identity for $t {
            fn id(&self) -> Self::Output {
                $f(&self)
            }
        }
    };
    (@nobound $t:ty, id: $f:expr) => {
        impl Identity for $t {
            fn id(&self) -> Self::Output {
                $f(&self)
            }
        }
    };

    (@bound $t:ty, inv: $f:expr; $($bound:tt)+) => {
        impl<$($bound)+> Inverse for $t {
            fn inv(&self, a: Self::Output) -> Self::Output {
                $f(self, a)
            }
        }
    };
    (@nobound $t:ty, inv: $f:expr) => {
        impl Inverse for $t {
            fn inv(&self, a: Self::Output) -> Self::Output {
                $f(self, a)
            }
        }
    };

    (@bound $t:ty, commu: $f:expr; $($bound:tt)+) => {impl<$($bound)+> Commutative for $t {}};
    (@nobound $t:ty, commu: $f:expr) => {impl Commutative for $t {}};

    (@bound $t:ty, assoc: $f:expr; $($bound:tt)+) => {impl<$($bound)+> Associative for $t {}};
    (@nobound $t:ty, assoc: $f:expr) => {impl Associative for $t {}};

    (@bound $t:ty, idem: $f:expr; $($bound:tt)+) => {impl<$($bound)+> Idempotence for $t {}};
    (@nobound $t:ty, idem: $f:expr) => {impl Idempotence for $t {}};

    (const $a:ident : $b:ty; $t:ty, $($s:ident: $f:expr),+) => {
        $(impl_algebra!(@bound $t, $s: $f; const $a: $b);)+
    };

    ($a:ident; $t:ty, $($s:ident: $f:expr),+) => {
        $(impl_algebra!(@bound $t, $s: $f; $a);)+
    };

    ($t:ty, $($s:ident: $f:expr),+) => {
        $(impl_algebra!(@nobound $t, $s: $f);)+
    };

}
