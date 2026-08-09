//! 配列に対する範囲頻度取得クエリ

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

use crate::algo::bsearch_slice::BinarySearch;

/// 配列に対する範囲頻度取得クエリを処理する。
pub struct StaticRangeFreqQuery<T> {
    map: HashMap<T, Vec<usize>>,
}

impl<T: Hash + Eq> StaticRangeFreqQuery<T> {
    /// **Time complexity** $O(|a|)$
    pub fn new(a: Vec<T>) -> Self {
        let mut map = HashMap::new();

        for (i, x) in a.into_iter().enumerate() {
            map.entry(x).or_insert_with(Vec::new).push(i);
        }

        Self { map }
    }

    /// **Time complexity** $O(\log |a|)$
    pub fn query(&self, Range { start, end }: Range<usize>, value: &T) -> usize {
        if let Some(a) = self.map.get(value) {
            let lower = a.lower_bound(&start);
            let upper = a.lower_bound(&end);

            upper - lower
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use my_testtools::*;
    use rand::prelude::*;

    use super::*;

    #[test]
    fn test() {
        let mut rng = rand::rng();

        let m = 100;
        let n = 1000;
        let q = 100;

        let a = std::iter::repeat_with(|| rng.random_range(0..m))
            .take(n)
            .collect::<Vec<_>>();
        let sfq = StaticRangeFreqQuery::new(a.clone());

        for _ in 0..q {
            let lr = rand_range(&mut rng, 0..n);
            let x: u32 = rng.random_range(0..m);

            assert_eq!(
                sfq.query(lr.clone(), &x),
                a[lr].iter().filter(|&&y| y == x).count()
            );
        }
    }
}
