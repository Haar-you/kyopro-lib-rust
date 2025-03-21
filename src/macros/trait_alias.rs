//! `trait_alias!`

/// トレイトの別名を作る。
#[macro_export]
macro_rules! trait_alias {
    ($(#[$meta:meta])* $name:ident: $($t:tt)+) => {
        $(#[$meta])*
        pub trait $name : $($t)+ {}
        impl<T: $($t)+> $name for T {}
    };
}
