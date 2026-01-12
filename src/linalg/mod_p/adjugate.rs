//! $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の余因子行列
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/adjugate_matrix>
use crate::{
    linalg::mod_p::{determinant::determinant, inverse::inverse, lineq::lineq},
    num::ff::*,
};

/// $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上で余因子行列を求める。
///
/// **Time complexity** $O(n^3)$
pub fn adjugate<F>(a: Vec<Vec<F::Element>>, modulo: &F) -> Vec<Vec<F::Element>>
where
    F: FF,
    F::Element: FFElem,
{
    let n = a.len();
    assert!(
        a.iter().all(|r| r.len() == n),
        "正方行列でなければならない。"
    );

    if let Some(inv) = inverse(a.clone(), modulo) {
        let det = determinant(a, modulo);
        inv.into_iter()
            .map(|r| r.into_iter().map(|x| x * det).collect())
            .collect()
    } else {
        let (_, s) = lineq(a.clone(), vec![modulo.zero(); n], modulo).unwrap();

        if s.len() >= 2 {
            return vec![vec![modulo.zero(); n]; n];
        }

        let b = (0..n).map(|i| (0..n).map(|j| a[j][i]).collect()).collect();
        let (_, t) = lineq(b, vec![modulo.zero(); n], modulo).unwrap();

        let i = (0..n).find(|&k| s[0][k].value() != 0).unwrap();
        let j = (0..n).find(|&k| t[0][k].value() != 0).unwrap();

        let m = {
            let mut b = a.clone();
            b.remove(j);
            for r in &mut b {
                r.remove(i);
            }

            let mut d = determinant(b, modulo);
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
