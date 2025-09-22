//! ワイルドカードパターンマッチング
//!
//! # References
//! - <https://qiita.com/MatsuTaku/items/cd5581fab97d7e74a7b3>
//! - <https://noshi91.hatenablog.com/entry/2024/05/26/060854>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/wildcard_pattern_matching>

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::iter::collect::CollectVec;
use crate::math::ntt::*;
use crate::math::prime_mod::Prime;
use crate::num::ff::FFElem;

/// `seq`の`|pat|`長の各連続部分列が`pat`と一致するかを判定する。
/// `wildcard`はワイルドカードとして扱う。
pub fn wildcard_pattern_matching<T>(seq: Vec<T>, pat: Vec<T>, wildcard: T) -> Vec<bool>
where
    T: Hash + Eq + Copy,
{
    assert!(pat.len() <= seq.len());

    let m = seq.len() - pat.len() + 1;
    let n = (seq.len() + pat.len() - 1).next_power_of_two();
    let ntt = NTT::<Prime<998244353>>::new();

    let mut s = vec![0.into(); n];

    for (i, x) in seq.into_iter().enumerate() {
        if x != wildcard {
            s[i] = hash(x).into();
        }
    }

    let mut p = vec![0.into(); n];

    for (i, x) in pat.into_iter().enumerate() {
        let i = (n - i) % n;

        if x != wildcard {
            p[i] = hash(x).into()
        }
    }

    let pr = p
        .iter()
        .enumerate()
        .map(|(i, &x)| x * hash(i).into())
        .collect_vec();

    let mut s2 = s.iter().map(|&x| x * x).collect_vec();
    let mut p1r = pr.clone();
    ntt.ntt(&mut s2);
    ntt.ntt(&mut p1r);
    s2.iter_mut().zip(p1r).for_each(|(x, y)| *x *= y);

    let mut s1 = s.clone();
    let mut p2r = pr.into_iter().zip(p).map(|(x, y)| x * y).collect_vec();
    ntt.ntt(&mut s1);
    ntt.ntt(&mut p2r);
    s1.iter_mut().zip(p2r).for_each(|(x, y)| *x *= y);

    let mut ret = vec![0.into(); n];
    ret.iter_mut().zip(s2).for_each(|(x, y)| *x += y);
    ret.iter_mut().zip(s1).for_each(|(x, y)| *x -= y);
    ntt.intt(&mut ret);
    ret.into_iter().take(m).map(|x| x.value() == 0).collect()
}

fn hash<T: Hash>(x: T) -> u64 {
    let mut s = DefaultHasher::new();
    x.hash(&mut s);
    s.finish()
}
