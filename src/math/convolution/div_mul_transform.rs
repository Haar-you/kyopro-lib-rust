//! 約数・倍数 Zeta / Möbius 変換

use std::ops::{Add, Sub};

/// [`div_zeta`]の逆変換操作。
pub fn div_mobius<T>(f: &mut [T])
where
    T: Copy + Sub<Output = T>,
{
    let n = f.len();
    let mut check = vec![true; n];
    for i in 2..n {
        if check[i] {
            for j in (1..=(n - 1) / i).rev() {
                check[j * i] = false;
                f[j * i] = f[j * i] - f[j];
            }
        }
    }
}

/// $F_j = \sum_{j = 0 \pmod i} f_i$を満たす`F`を求める。
pub fn div_zeta<T>(f: &mut [T])
where
    T: Copy + Add<Output = T>,
{
    let n = f.len();
    let mut check = vec![true; n];
    for i in 2..n {
        if check[i] {
            for j in (1..).take_while(|j| j * i < n) {
                check[j * i] = false;
                f[j * i] = f[j * i] + f[j];
            }
        }
    }
}

/// [`mul_zeta`]の逆変換操作。
pub fn mul_mobius<T>(f: &mut [T])
where
    T: Copy + Sub<Output = T>,
{
    let n = f.len();
    let mut check = vec![true; n];
    for i in 2..n {
        if check[i] {
            for j in (1..).take_while(|j| j * i < n) {
                check[j * i] = false;
                f[j] = f[j] - f[j * i];
            }
        }
    }
}

/// $F_j = \sum_{i = 0 \pmod j} f_i$を満たす`F`を求める。
pub fn mul_zeta<T>(f: &mut [T])
where
    T: Copy + Add<Output = T>,
{
    let n = f.len();
    let mut check = vec![true; n];
    for i in 2..n {
        if check[i] {
            for j in (1..=(n - 1) / i).rev() {
                check[j * i] = false;
                f[j] = f[j] + f[j * i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::collect::CollectVec;

    use super::*;
    use rand::Rng;

    #[test]
    fn test_div() {
        let mut rng = rand::thread_rng();

        let n = 500;
        let m = 100000;

        let f = (0..=n)
            .map(|i| rng.gen_range(0..m) * i as u64 % m)
            .collect_vec();

        let mut g = vec![0; n + 1];
        for (j, gj) in g.iter_mut().enumerate().take(n + 1).skip(1) {
            for (i, fi) in f.iter().enumerate().take(n + 1).skip(1) {
                if j % i == 0 {
                    *gj += fi;
                }
            }
        }

        let mut h = f.clone();
        div_zeta(&mut h);
        assert_eq!(h, g);

        div_mobius(&mut h);
        assert_eq!(h, f);
    }

    #[test]
    fn test_mul() {
        let mut rng = rand::thread_rng();

        let n = 500;
        let m = 100000;

        let f = (0..=n)
            .map(|i| rng.gen_range(0..m) * i as u64 % m)
            .collect_vec();

        let mut g = vec![0; n + 1];

        for (j, gj) in g.iter_mut().enumerate().take(n + 1).skip(1) {
            for i in (j..=n).step_by(j) {
                *gj += f[i];
            }
        }

        let mut h = f.clone();
        mul_zeta(&mut h);
        assert_eq!(h, g);

        mul_mobius(&mut h);
        assert_eq!(h, f);
    }
}
