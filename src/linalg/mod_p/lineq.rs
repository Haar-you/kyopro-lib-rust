//! $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の連立一次方程式
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/system_of_linear_equations>
use crate::{linalg::mod_p::gaussian_elim::*, num::ff::*};

/// $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上で連立一次方程式$A \boldsymbol{x} = \boldsymbol{b}$を解く。
///
/// ここで、$A$は$n \times m$の行列、$\boldsymbol{x}$は$m$行の縦ベクトル、$\boldsymbol{b}$は$n$行の縦ベクトル。
///
/// 連立方程式が解をもたないとき、`None`を返す。
/// そうでなければ、`Some((sol, bases))`を返す。
///
/// `sol`は$m$行のベクトル、`bases`は`dim`個の$m$行のベクトルで、
/// 連立方程式の解は、`bases`の要素の線型結合と`sol`の和で表される。
pub fn lineq<F>(
    mut a: Vec<Vec<F::Element>>,
    b: Vec<F::Element>,
    modulo: &F,
) -> Option<(Vec<F::Element>, Vec<Vec<F::Element>>)>
where
    F: FF,
    F::Element: FFElem,
{
    let n = a.len();
    assert_eq!(b.len(), n);

    let Some(m) = a.first().map(|a| a.len()) else {
        panic!("行数は0以上でなければならない。")
    };
    assert!(a.iter().all(|r| r.len() == m));

    for (r, bi) in a.iter_mut().zip(b.iter()) {
        r.push(*bi);
    }

    let (rank, mut a) = gaussian_elim(a);

    let dim = m - rank;

    let b: Vec<_> = a.iter_mut().map(|r| r.pop().unwrap()).collect();

    if rank > 0 && a[rank - 1].iter().all(|x| x.value() == 0) {
        return None;
    }

    let mut index_zero = vec![];
    let mut index_one = vec![];
    {
        let mut k = 0;
        for ai in a.iter().take(rank) {
            for (j, aij) in ai.iter().enumerate().take(m).skip(k) {
                if aij.value() != 0 {
                    index_one.push(j);
                    k = j + 1;
                    break;
                }
                index_zero.push(j);
            }
        }
        for j in k..m {
            index_zero.push(j);
        }
    }

    assert_eq!(index_zero.len(), dim);
    assert_eq!(index_one.len(), rank);

    let mut sol = vec![modulo.zero(); m];
    for (i, x) in b.into_iter().take(rank).enumerate() {
        sol[index_one[i]] = x;
    }

    let mut bases = vec![vec![modulo.zero(); m]; dim];
    for i in 0..rank {
        for (j, &k) in index_zero.iter().enumerate() {
            bases[j][index_one[i]] = -a[i][k];
        }
    }

    for i in 0..dim {
        bases[i][index_zero[i]] = modulo.one();
    }

    Some((sol, bases))
}
