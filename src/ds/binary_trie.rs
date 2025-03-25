//! 非負整数を2進数として管理する。
use crate::misc::nullable_usize::NullableUsize;

#[derive(Debug, Clone)]
struct Node {
    ch: [NullableUsize; 2],
    count: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            ch: [NullableUsize::NULL, NullableUsize::NULL],
            count: 0,
        }
    }
}

/// 非負整数を2進数として管理する。
#[derive(Debug, Clone)]
pub struct BinaryTrie {
    data: Vec<Node>,
    bitlen: usize,
}

impl BinaryTrie {
    /// `bitlen`ビットの数を扱える[`BinaryTrie`]を生成する。
    pub fn new(bitlen: usize) -> Self {
        assert!(bitlen <= 64);
        let data = vec![Node::default()];
        Self { data, bitlen }
    }

    /// 要素数を返す。
    pub fn len(&self) -> usize {
        self.data[0].count
    }

    /// 要素数が0ならば`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.data[0].count == 0
    }

    /// 値`value`の個数を返す。
    pub fn count(&self, value: u64) -> usize {
        let mut node = 0;
        let mut depth = self.bitlen;

        while depth > 0 {
            depth -= 1;
            let b = (value >> depth) & 1;

            let t = self.data[node].ch[b as usize];
            if t.is_null() {
                return 0;
            }
            node = t.0;
        }

        self.data[node].count
    }

    /// 値`value`を挿入して、`value`の個数を返す。
    pub fn insert(&mut self, value: u64) -> usize {
        let mut node = 0;
        let mut depth = self.bitlen;

        while depth > 0 {
            self.data[node].count += 1;
            depth -= 1;

            let b = (value >> depth) & 1;

            let ch = self.data[node].ch[b as usize];
            if !ch.is_null() {
                node = ch.0;
            } else {
                self.data.push(Node::default());
                let ch = self.data.len() - 1;
                self.data[node].ch[b as usize] = NullableUsize(ch);
                node = ch;
            }
        }

        self.data[node].count += 1;
        self.data[node].count
    }

    /// 値`value`が存在すれば、一つ削除して、削除後の`value`の個数を`Some`に包んで返す。
    /// 存在しなければ、`None`を返す。
    pub fn erase(&mut self, value: u64) -> Option<usize> {
        let mut node = 0;
        let mut depth = self.bitlen;
        let mut path = vec![];

        while depth > 0 {
            depth -= 1;
            let b = (value >> depth) & 1;

            path.push(node);

            let ch = self.data[node].ch[b as usize];
            if !ch.is_null() {
                node = ch.0;
            } else {
                self.data.push(Node::default());
                let ch = self.data.len() - 1;
                self.data[node].ch[b as usize] = NullableUsize(ch);
                node = ch;
            }
        }

        if self.data[node].count > 0 {
            path.push(node);
            for a in path {
                self.data[a].count -= 1;
            }

            Some(self.data[node].count)
        } else {
            None
        }
    }

    /// $\min_{a \in S} a \oplus xor$を求める。
    pub fn min(&mut self, xor: u64) -> Option<u64> {
        if self.data[0].count == 0 {
            None
        } else {
            let mut node = 0;
            let mut depth = self.bitlen;
            let mut ret = 0;

            while depth > 0 {
                depth -= 1;

                let mut b = (xor >> depth) & 1;

                let t = self.data[node].ch[b as usize];
                if t.is_null() || self.data[t.0].count == 0 {
                    b ^= 1;
                }

                node = self.data[node].ch[b as usize].0;
                ret |= b << depth;
            }

            Some(ret)
        }
    }

    /// $\max_{a \in S} a \oplus xor$を求める。
    pub fn max(&mut self, xor: u64) -> Option<u64> {
        if self.data[0].count == 0 {
            None
        } else {
            let mut node = 0;
            let mut depth = self.bitlen;
            let mut ret = 0;

            while depth > 0 {
                depth -= 1;

                let mut b = ((xor >> depth) & 1) ^ 1;

                let t = self.data[node].ch[b as usize];
                if t.is_null() || self.data[t.0].count == 0 {
                    b ^= 1;
                }

                node = self.data[node].ch[b as usize].0;
                ret |= b << depth;
            }

            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let mut bt = BinaryTrie::new(64);
        let mut m = BTreeMap::new();

        for _ in 0..1000 {
            let x = rng.gen_range(0..100);

            bt.insert(x);
            *m.entry(x).or_insert(0) += 1;

            let y = rng.gen::<u64>();

            assert_eq!(
                bt.min(y),
                m.iter().map(|(&a, _)| a).min_by_key(|&a| (a ^ y))
            );

            assert_eq!(
                bt.max(y),
                m.iter().map(|(&a, _)| a).max_by_key(|&a| (a ^ y))
            );

            assert_eq!(
                (0..100).map(|i| bt.count(i)).collect::<Vec<_>>(),
                (0..100)
                    .map(|i| *m.get(&i).unwrap_or(&0))
                    .collect::<Vec<_>>()
            );

            let x = rng.gen_range(0..100);

            assert_eq!(bt.erase(x).unwrap_or(0), bt.count(x));
            match m.get_mut(&x) {
                Some(y) if *y >= 1 => {
                    *y -= 1;
                    if *y == 0 {
                        m.remove(&x);
                    }
                }
                _ => {}
            }
        }
    }
}
