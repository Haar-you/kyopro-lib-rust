//! $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の行列のパフィアン
//!
//! # References
//! - <https://en.wikipedia.org/wiki/Pfaffian>
//! # Problems
//! - <https://judge.yosupo.jp/problem/pfaffian_of_matrix>

use crate::misc::swap::swap_vv;
use crate::num::ff::*;

fn swap<T>(a: &mut [Vec<T>], i: usize, j: usize)
where
    T: FFElem,
{
    assert!(i < j);

    for k in 0..i {
        swap_vv(a, i, k, j, k);
    }

    for k in i + 1..j {
        swap_vv(a, k, i, j, k);
        a[k][i] = -a[k][i];
        a[j][k] = -a[j][k];
    }

    for k in j + 1..a.len() {
        swap_vv(a, k, i, k, j);
    }

    a[j][i] = -a[j][i];
}

fn add<T>(a: &mut [Vec<T>], s: T, i: usize, j: usize)
where
    T: FFElem,
{
    assert!(i < j);

    for k in 0..i {
        unsafe {
            let t = *a.get_unchecked(i).get_unchecked(k);
            *a.get_unchecked_mut(j).get_unchecked_mut(k) += t * s;
        }
    }

    for k in i + 1..j {
        unsafe {
            let t = *a.get_unchecked(k).get_unchecked(i);
            *a.get_unchecked_mut(j).get_unchecked_mut(k) -= t * s;
        }
    }

    for k in j + 1..a.len() {
        unsafe {
            let t = *a.get_unchecked(k).get_unchecked(i);
            *a.get_unchecked_mut(k).get_unchecked_mut(j) += t * s;
        }
    }
}

/// $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の行列のパフィアンを求める。
///
/// 入力の行列は、$n \times n$($n$は偶数)の[歪対称行列](https://en.wikipedia.org/wiki/Skew-symmetric_matrix)である。
///
/// **Time complexity** $O(n^3)$
pub fn pfaffian<F>(mut a: Vec<Vec<F::Element>>, modulo: &F) -> F::Element
where
    F: FF,
    F::Element: FFElem + std::fmt::Debug,
{
    let n = a.len();

    assert_eq!(n % 2, 0);
    assert!(a.iter().all(|r| r.len() == n));

    for i in 0..n {
        for j in 0..n {
            assert_eq!(a[i][j], -a[j][i]);
        }
    }

    for (i, r) in a.iter_mut().enumerate() {
        r.truncate(i + 1);
    }

    let mut minus = false;
    for i in (0..n).step_by(2) {
        if a[i + 1][i].value() == 0 {
            if let Some(j) = (i + 2..n).find(|&j| a[j][i].value() != 0) {
                swap(&mut a, i + 1, j);
                minus = !minus;
            } else {
                return modulo.zero();
            }
        }

        assert_ne!(a[i + 1][i], modulo.zero());
        let t = a[i + 1][i].inv();
        for j in i + 2..n {
            let c = -a[j][i] * t;
            add(&mut a, c, i + 1, j);

            let c = a[j][i + 1] * t;
            add(&mut a, c, i, j);
        }
    }

    let mut ret = modulo.one();

    for i in (0..n).step_by(2) {
        ret *= -a[i + 1][i];
    }
    if minus {
        ret = -ret;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::modint::*;

    fn check(a: Vec<Vec<i64>>, m: u32, ans: u32) {
        let m = ModIntBuilder::new(m);
        let a = a
            .into_iter()
            .map(|b| b.into_iter().map(|x| m.from_i64(x)).collect())
            .collect();
        assert_eq!(pfaffian(a, &m).value(), ans);
    }

    #[test]
    fn test() {
        check(
            vec![
                vec![0, 1, 2, 3],
                vec![-1, 0, 4, 5],
                vec![-2, -4, 0, 6],
                vec![-3, -5, -6, 0],
            ],
            998244353,
            8,
        );
        check(vec![vec![0, 1], vec![-1, 0]], 998244353, 1);
    }
}
