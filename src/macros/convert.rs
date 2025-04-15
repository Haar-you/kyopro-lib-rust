//! `impl_from!`

/// [`From`]を実装する。
#[macro_export]
macro_rules! impl_from {
    ($(#[$meta:meta])* [ $($t:tt)* ]; $from:ty => $into:ty, $f:expr) => {
        impl<$($t)*> From<$from> for $into {
            $(#[$meta])*
            fn from(value: $from) -> Self {
                $f(value)
            }
        }
    };
    ($(#[$meta:meta])* $from:ty => $into:ty, $f:expr) => {
        impl_from!($(#[$meta])* []; $from => $into, $f);
    };
}
