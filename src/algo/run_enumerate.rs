//! Runの列挙
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/runenumerate>
use std::collections::{btree_map::Entry, BTreeMap};

use crate::algo::zalgo::*;

/// 文字列のrunを列挙する。
pub fn run_enumerate<T: PartialEq + Clone>(mut s: Vec<T>) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    let n = s.len();

    run(&mut result, &s, 0);

    let a = zalgo(&s);
    a.into_iter()
        .enumerate()
        .skip(1)
        .filter(|(i, x)| i <= x)
        .for_each(|(i, x)| result.push((i, 0, i + x)));

    s.reverse();
    let a = zalgo(&s);
    a.into_iter()
        .enumerate()
        .skip(1)
        .filter(|(i, x)| i <= x)
        .for_each(|(i, x)| result.push((i, n - i - x, n)));

    let mut m = BTreeMap::<(usize, usize), usize>::new();

    for (t, l, r) in result {
        let p = (l, r);

        match m.entry(p) {
            Entry::Occupied(mut x) => {
                x.insert((*x.get()).min(t));
            }
            Entry::Vacant(x) => {
                x.insert(t);
            }
        }
    }

    let mut result: Vec<_> = m.into_iter().map(|((l, r), t)| (t, l, r)).collect();
    result.sort();
    result
}

fn run<T: PartialEq + Clone>(result: &mut Vec<(usize, usize, usize)>, s: &[T], left: usize) {
    if s.len() <= 1 {
        return;
    }

    let n = s.len();
    let m = n / 2;
    let first = &s[0..m];
    let second = &s[m..];

    let res = aux(first.to_vec(), second.to_vec());
    result.extend(res.into_iter().map(|(t, l, r)| (t, left + l, left + r)));

    let res = aux(
        second.iter().rev().cloned().collect(),
        first.iter().rev().cloned().collect(),
    );
    result.extend(
        res.into_iter()
            .map(|(t, l, r)| (t, left + n - r, left + n - l)),
    );

    run(result, first, left);
    run(result, second, left + m);
}

fn aux<T: PartialEq + Clone>(first: Vec<T>, second: Vec<T>) -> Vec<(usize, usize, usize)> {
    let mut ret = vec![];

    let n = first.len();
    let m = second.len();

    let a = zalgo(&first.iter().rev().cloned().collect::<Vec<_>>());

    let t = second
        .clone()
        .into_iter()
        .map(Some)
        .chain(std::iter::once(None))
        .chain(first.into_iter().map(Some))
        .chain(second.into_iter().map(Some))
        .collect::<Vec<_>>();
    let b = zalgo(&t);

    for i in 1..n {
        let l1 = a[i];
        let l2 = b[n + m - i + 1];

        if l1 + i == n || l2 == m || i > l1 + l2 {
            continue;
        }

        let l = n - i - l1;
        let r = n + l2;
        let t = i;
        ret.push((t, l, r));
    }

    ret
}
