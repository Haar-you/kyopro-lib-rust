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
