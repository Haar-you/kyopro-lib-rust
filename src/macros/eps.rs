#[macro_export]
macro_rules! eps {
    ($name:ident, $m:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
        struct $name {}
        impl EpsValue for $name {
            #[inline]
            fn eps() -> f64 {
                $m
            }
        }
    };
}
