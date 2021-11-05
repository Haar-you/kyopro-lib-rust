use std::mem::size_of;

#[derive(Default, Debug, Clone)]
struct Node {
    ch: [Option<Box<Node>>; 2],
    size: usize,
}

impl Node {
    fn count(&self, value: u64, depth: usize) -> usize {
        if depth == 0 {
            self.size
        } else {
            let depth = depth - 1;
            let b = (value >> depth) & 1;
            self.ch[b as usize]
                .as_ref()
                .map_or(0, |t| t.count(value, depth))
        }
    }

    fn insert(&mut self, value: u64, depth: usize) {
        self.size += 1;
        if depth > 0 {
            let depth = depth - 1;
            let b = (value >> depth) & 1;
            self.ch[b as usize]
                .get_or_insert(Box::new(Node::default()))
                .insert(value, depth);
        }
    }

    fn erase(&mut self, value: u64, depth: usize) {
        self.size -= 1;
        if depth > 0 {
            let depth = depth - 1;
            let b = (value >> depth) & 1;
            self.ch[b as usize]
                .get_or_insert(Box::new(Node::default()))
                .erase(value, depth);
        }
    }

    fn min(&self, x: u64, depth: usize) -> u64 {
        if depth == 0 {
            0
        } else {
            let depth = depth - 1;
            let mut b = (x >> depth) & 1;
            if self.ch[b as usize].as_ref().map_or(0, |t| t.size) == 0 {
                b ^= 1;
            }
            self.ch[b as usize].as_ref().unwrap().min(x, depth) | (b << depth)
        }
    }

    fn max(&self, x: u64, depth: usize) -> u64 {
        if depth == 0 {
            0
        } else {
            let depth = depth - 1;
            let mut b = ((x >> depth) & 1) ^ 1;
            if self.ch[b as usize].as_ref().map_or(0, |t| t.size) == 0 {
                b ^= 1;
            }
            self.ch[b as usize].as_ref().unwrap().max(x, depth) | (b << depth)
        }
    }

    fn to_vec(&self, value: u64, ret: &mut Vec<u64>) {
        if self.ch[0].is_none() && self.ch[1].is_none() {
            ret.extend_from_slice(&vec![value; self.size]);
        }
        if let Some(t) = self.ch[0].as_ref() {
            t.to_vec(value << 1, ret);
        }
        if let Some(t) = self.ch[1].as_ref() {
            t.to_vec(value << 1 | 1, ret);
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BinaryTrie {
    root: Option<Box<Node>>,
}

impl BinaryTrie {
    const fn bitlen() -> usize {
        size_of::<u64>() * 8
    }

    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn len(&self) -> usize {
        self.root.as_ref().map_or(0, |x| x.size)
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn count(&self, value: u64) -> usize {
        self.root
            .as_ref()
            .map_or(0, |t| t.count(value, Self::bitlen()))
    }

    pub fn insert(&mut self, value: u64) {
        self.root
            .get_or_insert(Box::new(Node::default()))
            .insert(value, Self::bitlen());
    }

    pub fn erase(&mut self, value: u64) {
        self.root
            .get_or_insert(Box::new(Node::default()))
            .erase(value, Self::bitlen());
    }

    pub fn min(&mut self, x: u64) -> Option<u64> {
        self.root.as_ref().map(|t| t.min(x, Self::bitlen()))
    }

    pub fn max(&mut self, x: u64) -> Option<u64> {
        self.root.as_ref().map(|t| t.max(x, Self::bitlen()))
    }

    pub fn to_vec(&self) -> Vec<u64> {
        let mut ret = vec![];
        if let Some(t) = self.root.as_ref() {
            t.to_vec(0, &mut ret);
        }
        ret
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

        let mut bt = BinaryTrie::new();
        let mut m = BTreeMap::new();

        for _ in 0..1000 {
            let x = rng.gen::<u64>() % 100;

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
            )
        }
    }
}
