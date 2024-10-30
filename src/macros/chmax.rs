//! 値を最大値で更新する。

/// 値を最大値で更新する。
#[macro_export]
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {{
        let temp = $b;
        if $a < temp {
            $a = temp;
            true
        } else {
            false
        }
    }};
}
