#[macro_export]
macro_rules! modulo {
    ($name:ident, $m:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
        struct $name {}
        impl Modulo for $name {
            fn value() -> u64 {
                $m
            }
        }
    }
}
