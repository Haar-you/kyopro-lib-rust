//! `rec!`

/// immutableかつ引数を一つのみとる再帰クロージャ
///
/// # References
/// - [https://qiita.com/vain0x/items/90c9580aa34926160ac1](https://qiita.com/vain0x/items/90c9580aa34926160ac1)
#[macro_export]
macro_rules! rec {
    ( |$self:ident, $arg_name:tt: $arg_ty:ty| -> $ret:ty $body:block ) => {{
        fn recurse<X, Y>(f: &dyn Fn(&dyn Fn(X) -> Y, X) -> Y, x: X) -> Y {
            f(&|x: X| recurse(&f, x), x)
        }

        |x| recurse(&|$self, $arg_name: $arg_ty| -> $ret { $body }, x)
    }};
}
