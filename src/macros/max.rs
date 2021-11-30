#[macro_export]
macro_rules! max {
    ($x:expr, $($xs:expr),*) => {
        {
            let mut ret = $x;
            for x in vec![$($xs),*] {
                if x > ret {
                    ret = x
                }
            }
            ret
        }
    }
}
