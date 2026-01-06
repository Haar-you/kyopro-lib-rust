//! $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の余因子行列
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/adjugate_matrix>
use crate::{
    linalg::mod_p::{determinant::determinant, inverse::inverse, lineq::lineq},
    num::{ff::FFElem, one_zero::*},
};

/// $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上で余因子行列を求める。
///
/// **Time complexity** $O(n^3)$
pub fn adjugate<T>(a: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: FFElem + Copy + Zero + One + std::fmt::Debug,
{
    let n = a.len();
    assert!(
        a.iter().all(|r| r.len() == n),
        "正方行列でなければならない。"
    );

    if let Some(inv) = inverse(a.clone()) {
        let det = determinant(a);
        inv.into_iter()
            .map(|r| r.into_iter().map(|x| x * det).collect())
            .collect()
    } else {
        let (_, s) = lineq(a.clone(), vec![T::zero(); n]).unwrap();

        if s.len() >= 2 {
            return vec![vec![T::zero(); n]; n];
        }

        let b = (0..n).map(|i| (0..n).map(|j| a[j][i]).collect()).collect();
        let (_, t) = lineq(b, vec![T::zero(); n]).unwrap();

        let i = (0..n).find(|&k| s[0][k] != T::zero()).unwrap();
        let j = (0..n).find(|&k| t[0][k] != T::zero()).unwrap();

        let m = {
            let mut b = a.clone();
            b.remove(j);
            for r in &mut b {
                r.remove(i);
            }

            let mut d = determinant(b);
            if (i + j) % 2 == 1 {
                d = -d;
            }
            d
        };

        let k = m / (s[0][i] * t[0][j]);

        (0..n)
            .map(|i| (0..n).map(|j| s[0][i] * t[0][j] * k).collect())
            .collect()
    }
}
