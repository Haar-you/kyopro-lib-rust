//! `mul_vec!`

/// 多次元の`Vec`を作る。
#[macro_export]
macro_rules! mul_vec {
    ( $v:expr; $n:expr ) => {
        vec![$v; $n]
    };

    ( $v:expr; $n:expr, $($ns:expr),+ ) => {
        vec![mul_vec![$v; $($ns),+]; $n]
    }
}
