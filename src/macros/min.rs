#[macro_export]
macro_rules! min {
    ($x:expr, $($xs:expr),*) => {
        {
            let mut ret = $x;
            for &x in &[$($xs),*] {
                if x < ret {
                    ret = x
                }
            }
            ret
        }
    }
}
