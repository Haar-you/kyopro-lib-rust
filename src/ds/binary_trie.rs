#[derive(Debug, Clone)]
struct Node {
    ch: [usize; 2],
    count: usize,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            ch: [NIL, NIL],
            count: 0,
        }
    }
}

const NIL: usize = !0;

#[derive(Debug, Clone)]
pub struct BinaryTrie {
    data: Vec<Node>,
    bitlen: usize,
}

impl BinaryTrie {
    pub fn new(bitlen: usize) -> Self {
        assert!(bitlen <= 64);
        let data = vec![Node::default()];
        Self { data, bitlen }
    }

    pub fn len(&self) -> usize {
        self.data[0].count
    }

    pub fn is_empty(&self) -> bool {
        self.data[0].count == 0
    }

    pub fn count(&self, value: u64) -> usize {
        let mut node = 0;
        let mut depth = self.bitlen;

        while depth > 0 {
            depth -= 1;
            let b = (value >> depth) & 1;

            let t = self.data[node].ch[b as usize];
            if t == NIL {
                return 0;
            }
            node = t;
        }

        self.data[node].count
    }

    pub fn insert(&mut self, value: u64) -> usize {
        let mut node = 0;
        let mut depth = self.bitlen;

        while depth > 0 {
            self.data[node].count += 1;
            depth -= 1;

            let b = (value >> depth) & 1;

            let ch = self.data[node].ch[b as usize];
            if ch != NIL {
                node = ch;
            } else {
                self.data.push(Node::default());
                let ch = self.data.len() - 1;
                self.data[node].ch[b as usize] = ch;
                node = ch;
            }
        }

        self.data[node].count += 1;
        self.data[node].count
    }

    pub fn erase(&mut self, value: u64) -> Option<usize> {
        let mut node = 0;
        let mut depth = self.bitlen;
        let mut path = vec![];

        while depth > 0 {
            depth -= 1;
            let b = (value >> depth) & 1;

            path.push(node);

            let ch = self.data[node].ch[b as usize];
            if ch != NIL {
                node = ch;
            } else {
                self.data.push(Node::default());
                let ch = self.data.len() - 1;
                self.data[node].ch[b as usize] = ch;
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

    pub fn min(&mut self, x: u64) -> Option<u64> {
        if self.data[0].count == 0 {
            None
        } else {
            let mut node = 0;
            let mut depth = self.bitlen;
            let mut ret = 0;

            while depth > 0 {
                depth -= 1;

                let mut b = (x >> depth) & 1;

                let t = self.data[node].ch[b as usize];
                if t == NIL || self.data[t].count == 0 {
                    b ^= 1;
                }

                node = self.data[node].ch[b as usize];
                ret |= b << depth;
            }

            Some(ret)
        }
    }

    pub fn max(&mut self, x: u64) -> Option<u64> {
        if self.data[0].count == 0 {
            None
        } else {
            let mut node = 0;
            let mut depth = self.bitlen;
            let mut ret = 0;

            while depth > 0 {
                depth -= 1;

                let mut b = ((x >> depth) & 1) ^ 1;

                let t = self.data[node].ch[b as usize];
                if t == NIL || self.data[t].count == 0 {
                    b ^= 1;
                }

                node = self.data[node].ch[b as usize];
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
