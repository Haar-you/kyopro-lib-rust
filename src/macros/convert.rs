//! `impl_from!`, `impl_try_from!`

/// [`From`]を実装する。
#[macro_export]
#[doc(hidden)]
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

/// [`TryFrom`]を実装する。
#[macro_export]
#[doc(hidden)]
macro_rules! impl_try_from {
    ($(#[$meta:meta])* [ $($t:tt)* ]; $from:ty => $into:ty, type Error = $error:ty, $f:expr) => {
        impl<$($t)*> TryFrom<$from> for $into {
            type Error = $error;
            $(#[$meta])*
            fn try_from(value: $from) -> Result<Self, Self::Error> {
                $f(value)
            }
        }
    };
    ($(#[$meta:meta])* $from:ty => $into:ty, type Error = $error:ty, $f:expr) => {
        impl_try_from!($(#[$meta])* []; $from => $into, type Error = $error, $f);
    };
}
