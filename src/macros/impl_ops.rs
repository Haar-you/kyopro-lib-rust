//! `impl_ops!`

/// [`std::ops::Add`], [`std::ops::Sub`], [`std::ops::Mul`], [`std::ops::Div`], [`std::ops::AddAssign`], [`std::ops::SubAssign`], [`std::ops::MulAssign`], [`std::ops::DivAssign`], [`std::ops::Neg`]を実装する。
#[macro_export]
macro_rules! impl_ops {
    (@inner, $(#[$meta:meta])* $tr:ty, $a:ty, $f:expr, $fn:tt; $($bound:tt)*) => {
        impl $($bound)* $tr for $a {
            type Output = Self;
            $(#[$meta])*
            fn $fn(self, rhs: Self) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
    (@inner_assign, $(#[$meta:meta])* $tr:ty, $a:ty, $f:expr, $fn:tt; $($bound:tt)*) => {
        impl $($bound)* $tr for $a {
            $(#[$meta])*
            fn $fn(&mut self, rhs: Self) {
                $f(self, rhs)
            }
        }
    };

    ($(#[$meta:meta])* <const $m:tt: $t:ty>; $trait:ident, $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* $trait, $a, $f; <const $m: $t>);
    };
    ($(#[$meta:meta])* $trait:ident, $a:ty, $f:expr) => {
        impl_ops!(@when $(#[$meta])* $trait, $a, $f;);
    };


    (@when $(#[$meta:meta])* Add, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, $(#[$meta])* std::ops::Add, $a, $f, add; $($bound)*);
    };
    (@when $(#[$meta:meta])* Sub, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, $(#[$meta])* std::ops::Sub, $a, $f, sub; $($bound)*);
    };
    (@when $(#[$meta:meta])* Mul, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, $(#[$meta])* std::ops::Mul, $a, $f, mul; $($bound)*);
    };
    (@when $(#[$meta:meta])* Div, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, $(#[$meta])* std::ops::Div, $a, $f, div; $($bound)*);
    };
    (@when $(#[$meta:meta])* Rem, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, $(#[$meta])* std::ops::Rem, $a, $f, rem; $($bound)*);
    };

    (@when $(#[$meta:meta])* AddAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, $(#[$meta])* std::ops::AddAssign, $a, $f, add_assign; $($bound)*);
    };
    (@when $(#[$meta:meta])* SubAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, $(#[$meta])* std::ops::SubAssign, $a, $f, sub_assign; $($bound)*);
    };
    (@when $(#[$meta:meta])* MulAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, $(#[$meta])* std::ops::MulAssign, $a, $f, mul_assign; $($bound)*);
    };
    (@when $(#[$meta:meta])* DivAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, $(#[$meta])* std::ops::DivAssign, $a, $f, div_assign; $($bound)*);
    };
    (@when $(#[$meta:meta])* RemAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, $(#[$meta])* std::ops::RemAssign, $a, $f, rem_assign; $($bound)*);
    };

    (@when $(#[$meta:meta])* Neg, $a:ty, $f:expr; $($bound:tt)*) => {
        impl $($bound)* std::ops::Neg for $a {
            type Output = Self;
            $(#[$meta])*
            fn neg(self) -> Self::Output {
                $f(self)
            }
        }
    }
}
