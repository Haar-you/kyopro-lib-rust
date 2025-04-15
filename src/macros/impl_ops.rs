//! `impl_ops!`

/// [`Add`](std::ops::Add), [`Sub`](std::ops::Sub), [`Mul`](std::ops::Mul), [`Div`](std::ops::Div), [`Rem`](std::ops::Rem), [`AddAssign`](std::ops::AddAssign), [`SubAssign`](std::ops::SubAssign), [`MulAssign`](std::ops::MulAssign), [`DivAssign`](std::ops::DivAssign), [`RemAssign`](std::ops::RemAssign), [`Neg`](std::ops::Neg)を実装する。
#[macro_export]
macro_rules! impl_ops {
    (@inner, $(#[$meta:meta])* [$($bound:tt)*]; $tr:ty, $a:ty, $f:expr, $fn:tt) => {
        impl <$($bound)*> $tr for $a {
            type Output = Self;
            $(#[$meta])*
            fn $fn(self, rhs: Self) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
    (@inner_assign, $(#[$meta:meta])* [$($bound:tt)*]; $tr:ty, $a:ty, $f:expr, $fn:tt) => {
        impl <$($bound)*> $tr for $a {
            $(#[$meta])*
            fn $fn(&mut self, rhs: Self) {
                $f(self, rhs)
            }
        }
    };

    ($(#[$meta:meta])* [$($bound:tt)*]; $trait:ident for $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* [$($bound)*]; $trait, $a, $f);
    };
    ($(#[$meta:meta])* $trait:ident for $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* []; $trait, $a, $f);
    };


    (@when $(#[$meta:meta])* [$($bound:tt)*]; Add, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Add, $a, $f, add);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Sub, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Sub, $a, $f, sub);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Mul, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Mul, $a, $f, mul);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Div, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Div, $a, $f, div);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Rem, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Rem, $a, $f, rem);
    };

    (@when $(#[$meta:meta])* [$($bound:tt)*]; AddAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::AddAssign, $a, $f, add_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; SubAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::SubAssign, $a, $f, sub_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; MulAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::MulAssign, $a, $f, mul_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; DivAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::DivAssign, $a, $f, div_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; RemAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::RemAssign, $a, $f, rem_assign);
    };

    (@when $(#[$meta:meta])* [$($bound:tt)*]; Neg, $a:ty, $f:expr) => {
        impl <$($bound)*> std::ops::Neg for $a {
            type Output = Self;
            $(#[$meta])*
            fn neg(self) -> Self::Output {
                $f(self)
            }
        }
    }
}
