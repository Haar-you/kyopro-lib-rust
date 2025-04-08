//! `sort_with!`

/// 複数の配列をソートする。
///
/// 第一引数の比較関数に基づいて、順列`0,1,..,n-1`をソートする。
/// 第二引数以降の配列を、この順列に基づいてソートする。
#[macro_export]
macro_rules! sort_with {
    ($cmp:expr, $($a:expr),+) => {
        {
            let n = vec![$($a.len()),+].into_iter().min().unwrap();
            let mut ord = (0..n).collect::<Vec<usize>>();
            ord.sort_by($cmp);

            let mut check = vec![false; n];

            for i in 0..n {
                if !check[i] {
                    check[i] = true;
                    let mut j = i;
                    while ord[j] != i {
                        {
                            $(
                                $a.swap(j, ord[j]);
                            )+
                        }
                        j = ord[j];
                        check[j] = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut a = vec![0, 1, 2, 3];
        let mut b = vec![3, 2, 9, 4];

        sort_with!(|&i, &j| b[i].cmp(&b[j]), a, b);

        assert_eq!(b, vec![2, 3, 4, 9]);
        assert_eq!(a, vec![1, 0, 3, 2]);
    }
}
