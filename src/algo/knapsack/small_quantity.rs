//! 要素数が小さいナップサック問題
use crate::{algo::merge::inplace_merge, chmax, num::one_zero::Zero};
use std::ops::Add;

/// 要素数が小さいナップサック問題
///
/// **Time complexity** $O(n 2 ^ {n / 2})$
///
/// **Space complexity** $O(2 ^ {n / 2})$
pub fn knapsack_small_quantity<W, V>(cap: W, ws: &[W], vs: &[V]) -> V
where
    W: Copy + Add<Output = W> + Ord + Zero,
    V: Copy + Add<Output = V> + Ord + Zero,
{
    let n = ws.len();
    assert_eq!(ws.len(), vs.len());

    let p = n / 2;

    let zero_w = W::zero();
    let zero_v = V::zero();

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

    for (w, v) in a {
        while j < b.len() && w + b[j].0 > cap {
            j += 1;
        }
        if j >= b.len() {
            break;
        }
        chmax!(ret, v + b[j].1);
    }

    ret
}
