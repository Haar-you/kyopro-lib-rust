#[macro_export]
macro_rules! modulo {
    ($name:ident, $m:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
        struct $name {}
        impl Modulo for $name {
            #[inline]
            fn value() -> u32 {
                $m
            }
        }
    };
}
