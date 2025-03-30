//! `impl_from!`

/// `From`を実装する。
#[macro_export]
macro_rules! impl_from {
    ($(#[$meta:meta])* $from:ty => $into:ty, $f:expr) => {
        impl From<$from> for $into {
            $(#[$meta])*
            fn from(value: $from) -> Self {
                $f(value)
            }
        }
    };
}
