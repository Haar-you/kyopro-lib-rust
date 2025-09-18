//! 乗法的関数

use std::ops::Mul;

use crate::num::one_zero::One;

/// 乗法的関数を列挙する。
///
/// `f(p, k)`は乗法的関数の$p^k$での値を返す。
pub fn multiplicative_table<T, F>(n: usize, f: F) -> Vec<T>
where
    T: Copy + Default + Mul<Output = T> + One,
    F: Fn(u64, u32) -> T,
{
    let mut ret = vec![T::default(); n + 1];
    let mut p = vec![(0, 0, 0); n + 1];

    ret[1] = T::one();

    for i in 2..=n {
        if p[i] == (0, 0, 0) {
            for j in (i..=n).step_by(i) {
                if p[j] == (0, 0, 0) {
                    let mut k = 0;
                    let mut m = j;

                    while m % i == 0 {
                        m /= i;
                        k += 1;
                    }

                    p[j] = (m, i, k);
                }
            }
        }
    }

    for i in 2..=n {
        let (m, j, k) = p[i];
        ret[i] = ret[m] * f(j as u64, k);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::totient::totient_table, timer};

    #[test]
    fn totient() {
        let f = |p: u64, k: u32| p.pow(k) - p.pow(k - 1);

        for n in [1, 10, 100, 1000, 10000, 100000, 1000000] {
            let res = timer! {n, {
                multiplicative_table(n, f)
            }};

            let ans = totient_table(n);

            assert_eq!(res, ans);
        }
    }

    #[test]
    fn mobius() {
        let f = |p: u64, k: u32| if k == 1 { -1 } else { 0 };

        for n in [1, 10, 100, 1000, 10000, 100000, 1000000] {
            let _res = timer! {n, {
                multiplicative_table(n, f)
            }};
        }
    }
}
