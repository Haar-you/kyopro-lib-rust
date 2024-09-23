#[macro_export]
macro_rules! impl_ops {
    (@inner, $tr:ty, $a:ty, $f:expr, $fn:tt) => {
        impl $tr for $a {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                $f(self, rhs)
            }
        }
    };
    (@inner_assign, $tr:ty, $a:ty, $f:expr, $fn:tt) => {
        impl $tr for $a {
            fn $fn(&mut self, rhs: Self) {
                $f(self, rhs)
            }
        }
    };

    (Add, $a:ty, $f:expr) => {
        impl_ops!(@inner, std::ops::Add, $a, $f, add);
    };
    (Sub, $a:ty, $f:expr) => {
        impl_ops!(@inner, std::ops::Sub, $a, $f, sub);
    };
    (Mul, $a:ty, $f:expr) => {
        impl_ops!(@inner, std::ops::Mul, $a, $f, mul);
    };
    (Div, $a:ty, $f:expr) => {
        impl_ops!(@inner, std::ops::Div, $a, $f, div);
    };

    (AddAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, std::ops::AddAssign, $a, $f, add_assign);
    };
    (SubAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, std::ops::SubAssign, $a, $f, sub_assign);
    };
    (MulAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, std::ops::MulAssign, $a, $f, mul_assign);
    };
    (DivAssign, $a:ty, $f:expr) => {
        impl_ops!(@inner_assign, std::ops::DivAssign, $a, $f, div_assign);
    };

    (Neg, $a:ty, $f:expr) => {
        impl std::ops::Neg for $a {
            type Output = Self;
            fn neg(self) -> Self::Output {
                $f(self)
            }
        }
    }
}
