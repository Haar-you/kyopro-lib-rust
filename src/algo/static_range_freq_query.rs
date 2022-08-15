//! 配列に対する範囲頻度取得クエリ

use crate::algo::bsearch::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;

/// 配列に対する範囲頻度取得クエリを処理する。
pub struct StaticRangeFreqQuery<T> {
    map: HashMap<T, Vec<usize>>,
}

impl<T: Hash + Eq + Clone> StaticRangeFreqQuery<T> {
    /// Time Complexity $O(|a|)$
    pub fn new(a: Vec<T>) -> Self {
        let mut map = HashMap::new();

        for (i, x) in a.iter().enumerate() {
            map.entry(x.clone()).or_insert(vec![]).push(i);
        }

        Self { map }
    }

    /// Time Complexity $O(log |a|)$
    pub fn query(&self, Range { start, end }: Range<usize>, value: &T) -> usize {
        if let Some(a) = self.map.get(value) {
            let lower = lower_bound(a, &start);
            let upper = lower_bound(a, &end);

            upper - lower
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testtools::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let m = 100;
        let n = 1000;
        let q = 100;

        let a = (0..n).map(|_| rng.gen_range(0..m)).collect::<Vec<_>>();
        let sfq = StaticRangeFreqQuery::new(a.clone());

        for _ in 0..q {
            let lr = rand_range(&mut rng, 0..n);
            let x: u32 = rng.gen_range(0..m);

            assert_eq!(
                sfq.query(lr.clone(), &x),
                a[lr].iter().filter(|&&y| y == x).count()
            );
        }
    }
}
