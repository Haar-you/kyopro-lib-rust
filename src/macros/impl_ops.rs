#[macro_export]
macro_rules! impl_ops {
    (@inner, $tr:ty, $a:ty, $f:expr, $fn:tt; $($bound:tt)*) => {
        impl $($bound)* $tr for $a {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
    (@inner_assign, $tr:ty, $a:ty, $f:expr, $fn:tt; $($bound:tt)*) => {
        impl $($bound)* $tr for $a {
            fn $fn(&mut self, rhs: Self) {
                $f(self, rhs)
            }
        }
    };

    (<const $m:tt: $t:ty>; $trait:ident, $a:ty, $f:expr) => {
        impl_ops!(@when $trait, $a, $f; <const $m: $t>);
    };
    ($trait:ident, $a:ty, $f:expr) => {
        impl_ops!(@when $trait, $a, $f;);
    };


    (@when Add, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, std::ops::Add, $a, $f, add; $($bound)*);
    };
    (@when Sub, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, std::ops::Sub, $a, $f, sub; $($bound)*);
    };
    (@when Mul, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, std::ops::Mul, $a, $f, mul; $($bound)*);
    };
    (@when Div, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner, std::ops::Div, $a, $f, div; $($bound)*);
    };

    (@when AddAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, std::ops::AddAssign, $a, $f, add_assign; $($bound)*);
    };
    (@when SubAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, std::ops::SubAssign, $a, $f, sub_assign; $($bound)*);
    };
    (@when MulAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, std::ops::MulAssign, $a, $f, mul_assign; $($bound)*);
    };
    (@when DivAssign, $a:ty, $f:expr; $($bound:tt)*) => {
        impl_ops!(@inner_assign, std::ops::DivAssign, $a, $f, div_assign; $($bound)*);
    };

    (@when Neg, $a:ty, $f:expr; $($bound:tt)*) => {
        impl $($bound)* std::ops::Neg for $a {
            type Output = Self;
            fn neg(self) -> Self::Output {
                $f(self)
            }
        }
    }
}
