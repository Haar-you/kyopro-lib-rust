#[macro_export]
macro_rules! impl_one_zero {
    ([$($bound:tt)*]; $t:ty; zero: $e:expr; $($rest:tt)*) => {
        impl <$($bound)*> Zero for $t { fn zero() -> Self { $e } }
        impl_one_zero!([$($bound)*]; $t; $($rest)*);
    };
    ([$($bound:tt)*]; $t:ty; one: $e:expr; $($rest:tt)*) => {
        impl <$($bound)*> One for $t { fn one() -> Self { $e } }
        impl_one_zero!([$($bound)*]; $t; $($rest)*);
    };
    ([$($bound:tt)*]; $t:ty;) => {};
    ($t:ty; $($rest:tt)*) => {impl_one_zero!([]; $t; $($rest)*);};
}
