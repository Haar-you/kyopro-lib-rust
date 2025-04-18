//! `impl_ops!`

/// [`Add`](std::ops::Add), [`Sub`](std::ops::Sub), [`Mul`](std::ops::Mul), [`Div`](std::ops::Div), [`Rem`](std::ops::Rem), [`AddAssign`](std::ops::AddAssign), [`SubAssign`](std::ops::SubAssign), [`MulAssign`](std::ops::MulAssign), [`DivAssign`](std::ops::DivAssign), [`RemAssign`](std::ops::RemAssign), [`Neg`](std::ops::Neg)を実装する。
#[macro_export]
macro_rules! impl_ops {
    (@inner, $(#[$meta:meta])* [$($bound:tt)*]; $tr:ty, $rhs:ty, $a:ty, $f:expr, $fn:tt) => {
        impl <$($bound)*> $tr for $a {
            type Output = Self;
            $(#[$meta])*
            fn $fn(self, rhs: $rhs) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
    (@inner_assign, $(#[$meta:meta])* [$($bound:tt)*]; $tr:ty, $rhs:ty, $a:ty, $f:expr, $fn:tt) => {
        impl <$($bound)*> $tr for $a {
            $(#[$meta])*
            fn $fn(&mut self, rhs: $rhs) {
                $f(self, rhs)
            }
        }
    };

    ($(#[$meta:meta])* [$($bound:tt)*]; $trait:ident for $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* [$($bound)*]; $trait, Self, $a, $f);
    };
    ($(#[$meta:meta])* [$($bound:tt)*]; $trait:ident <$rhs:ty> for $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* [$($bound)*]; $trait, $rhs, $a, $f);
    };
    ($(#[$meta:meta])* $trait:ident for $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* []; $trait, Self, $a, $f);
    };
    ($(#[$meta:meta])* $trait:ident <$rhs:ty> for $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* []; $trait, $rhs, $a, $f);
    };

    (@when $(#[$meta:meta])* [$($bound:tt)*]; Add, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Add<$rhs>, $rhs, $a, $f, add);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Sub, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Sub<$rhs>, $rhs, $a, $f, sub);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Mul, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Mul<$rhs>, $rhs, $a, $f, mul);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Div, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Div<$rhs>, $rhs, $a, $f, div);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; Rem, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner, $(#[$meta])* [$($bound)*]; std::ops::Rem<$rhs>, $rhs, $a, $f, rem);
    };

    (@when $(#[$meta:meta])* [$($bound:tt)*]; AddAssign, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::AddAssign<$rhs>, $rhs, $a, $f, add_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; SubAssign, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::SubAssign<$rhs>, $rhs, $a, $f, sub_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; MulAssign, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::MulAssign<$rhs>, $rhs, $a, $f, mul_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; DivAssign, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::DivAssign<$rhs>, $rhs, $a, $f, div_assign);
    };
    (@when $(#[$meta:meta])* [$($bound:tt)*]; RemAssign, $rhs:ty, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, $(#[$meta])* [$($bound)*]; std::ops::RemAssign<$rhs>, $rhs, $a, $f, rem_assign);
    };

    (@when $(#[$meta:meta])* [$($bound:tt)*]; Neg, $rhs:ty, $a:ty, $f:expr) => {
        impl <$($bound)*> std::ops::Neg for $a {
            type Output = Self;
            $(#[$meta])*
            fn neg(self) -> Self::Output {
                $f(self)
            }
        }
    }
}
