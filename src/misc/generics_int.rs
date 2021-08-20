pub trait GenericsInt {
    type Output;
    fn value() -> Self::Output;
}

#[macro_export]
macro_rules! generics_int {
    ( $t:ident, $s:expr ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        struct $t {}
        impl GenericsInt for $t {
            type Output = u64;
            fn value() -> u64 {
                $s
            }
        }
    };
}
