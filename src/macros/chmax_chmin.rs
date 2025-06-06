//! `chmax!`, `chmin!`

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

/// 値を最小値で更新する。
#[macro_export]
macro_rules! chmin {
    ( $a:expr, $b:expr ) => {{
        let temp = $b;
        if $a > temp {
            $a = temp;
            true
        } else {
            false
        }
    }};
}
