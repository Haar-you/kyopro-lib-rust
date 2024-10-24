//! 複数の値の最大値を得る。

/// 複数の値の最大値を得る。
#[macro_export]
macro_rules! max {
    ($x:expr, $($xs:expr),*) => {
        {
            let mut ret = $x;
            for &x in &[$($xs),*] {
                if x > ret {
                    ret = x
                }
            }
            ret
        }
    }
}
