//! 連立一次方程式$A \boldsymbol{x} = \boldsymbol{b} \pmod 2$を解く。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/system_of_linear_equations_mod_2>
use crate::{ds::bitset::Bitset, linalg::mod_2::gaussian_elim::*};

/// 連立一次方程式$A \boldsymbol{x} = \boldsymbol{b} \pmod 2$を解く。
///
/// ここで、$A$は$n \times m$の行列、$\boldsymbol{x}$は$m$行の縦ベクトル、$\boldsymbol{b}$は$n$行の縦ベクトル。
///
/// 連立方程式が解をもたないとき、`None`を返す。
/// そうでなければ、`Some((sol, bases))`を返す。
///
/// `sol`は$m$行のベクトル、`bases`は`dim`個の$m$行のベクトルで、
/// 連立方程式の解は、`bases`の要素の線型結合と`sol`の和で表される。
pub fn lineq(mut a: Vec<Bitset>, b: Vec<bool>) -> Option<(Bitset, Vec<Bitset>)> {
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

    if rank > 0 && a[rank - 1].count_ones() == 0 {
        return None;
    }

    let mut index_zero = vec![];
    let mut index_one = vec![];
    {
        let mut k = 0;
        for ai in a.iter().take(rank) {
            for j in (0..).take(m).skip(k) {
                if ai.test(j) {
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

    let mut sol = Bitset::new(m);
    for (i, x) in b.into_iter().take(rank).enumerate() {
        sol.set(index_one[i], x);
    }

    let mut bases = vec![Bitset::new(m); dim];
    for i in 0..rank {
        for (j, &k) in index_zero.iter().enumerate() {
            bases[j].set(index_one[i], a[i].test(k));
        }
    }

    for i in 0..dim {
        bases[i].set(index_zero[i], true);
    }

    Some((sol, bases))
}
