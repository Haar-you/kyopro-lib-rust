#![allow(clippy::many_single_char_names)]

use crate::{chmax, utils::merge::inplace_merge};
use std::ops::Add;

/// 要素数が小さいナップサック問題
///
/// Time complexity O(n * 2 ^ (n / 2))
///
/// Space complexity O(2 ^ (n / 2))
pub fn knapsack_small_quantity<W, V>(n: usize, cap: W, ws: &[W], vs: &[V]) -> V
where
    W: Default + Copy + Add<Output = W> + Ord,
    V: Default + Copy + Add<Output = V> + Ord,
{
    let p = n / 2;

    let zero_w = W::default();
    let zero_v = V::default();

    let mut a: Vec<(W, V)> = vec![(zero_w, zero_v)];
    let mut b: Vec<(W, V)> = vec![(zero_w, zero_v)];

    for i in 0..p {
        let k = a.len();

        let temp = a
            .iter()
            .map(|&(w, v)| (w + ws[i], v + vs[i]))
            .collect::<Vec<_>>();
        a.extend_from_slice(&temp);
        inplace_merge(&mut a, k);
    }

    for i in p..n {
        let k = b.len();

        let temp = b
            .iter()
            .map(|&(w, v)| (w + ws[i], v + vs[i]))
            .collect::<Vec<_>>();
        b.extend_from_slice(&temp);
        inplace_merge(&mut b, k);
    }

    for i in 1..a.len() {
        chmax!(a[i].1, a[i - 1].1);
    }

    for i in 1..b.len() {
        chmax!(b[i].1, b[i - 1].1);
    }

    b.reverse();

    let mut ret = zero_v;
    let mut j = 0;
    for i in 0..a.len() {
        while j < b.len() && a[i].0 + b[j].0 > cap {
            j += 1;
        }
        if j >= b.len() {
            break;
        }
        chmax!(ret, a[i].1 + b[j].1);
    }

    ret
}
