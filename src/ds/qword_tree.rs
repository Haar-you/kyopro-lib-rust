const MAX: u32 = 1 << 24;

pub struct QwordTree {
    v0: u64,
    v1: Vec<u64>,
    v2: Vec<u64>,
    v3: Vec<u64>,
}

impl QwordTree {
    pub fn new() -> Self {
        Self {
            v0: 0,
            v1: vec![0; 1 << 6],
            v2: vec![0; 1 << 12],
            v3: vec![0; 1 << 18],
        }
    }

    pub fn insert(&mut self, x: u32) {
        assert!(x < MAX);

        let x = x as usize;

        self.v3[x >> 6] |= 1 << (x & 0x3f);
        self.v2[x >> 12] |= 1 << (x >> 6 & 0x3f);
        self.v1[x >> 18] |= 1 << (x >> 12 & 0x3f);
        self.v0 |= 1 << (x >> 18);
    }

    pub fn erase(&mut self, x: u32) {
        assert!(x < MAX);

        let x = x as usize;

        self.v3[x >> 6] &= !(1 << (x & 0x3f));
        self.v2[x >> 12] &= !((if self.v3[x >> 6] == 0 { 1 } else { 0 }) << (x >> 6 & 0x3f));
        self.v1[x >> 18] &= !((if self.v2[x >> 12] == 0 { 1 } else { 0 }) << (x >> 12 & 0x3f));
        self.v0 &= !((if self.v1[x >> 18] == 0 { 1 } else { 0 }) << (x >> 18));
    }

    pub fn contains(&self, x: u32) -> bool {
        if x >= MAX {
            false
        } else {
            self.v3[x as usize >> 6] & (1 << (x & 0x3f)) != 0
        }
    }

    pub fn min(&self) -> Option<u32> {
        if self.v0 == 0 {
            None
        } else {
            let mut ret = self.v0.trailing_zeros();
            ret = ret << 6 | self.v1[ret as usize].trailing_zeros();
            ret = ret << 6 | self.v2[ret as usize].trailing_zeros();
            ret = ret << 6 | self.v3[ret as usize].trailing_zeros();
            Some(ret)
        }
    }

    pub fn max(&self) -> Option<u32> {
        if self.v0 == 0 {
            None
        } else {
            let mut ret = 63 - self.v0.leading_zeros();
            ret = ret << 6 | (63 - self.v1[ret as usize].leading_zeros());
            ret = ret << 6 | (63 - self.v2[ret as usize].leading_zeros());
            ret = ret << 6 | (63 - self.v3[ret as usize].leading_zeros());
            Some(ret)
        }
    }

    pub fn min_ge(&self, x: u32) -> Option<u32> {
        if x >= MAX {
            return None;
        }

        let mask = !((1 << (x & 0x3f)) - 1);
        let t = (self.v3[x as usize >> 6] & mask).trailing_zeros();
        if t != 64 {
            return Some((x & !0x3f) | t);
        }

        let mask = (!0 << ((x >> 6) & 0x3f)) << 1;
        let t = (self.v2[x as usize >> 12] & mask).trailing_zeros();
        if t != 64 {
            let mut ret = ((x >> 6) & !0x3f) | t;
            ret = ret << 6 | self.v3[ret as usize].trailing_zeros();

            return Some(ret);
        }

        let mask = (!0 << ((x >> 12) & 0x3f)) << 1;
        let t = (self.v1[x as usize >> 18] & mask).trailing_zeros();
        if t != 64 {
            let mut ret = ((x >> 12) & !0x3f) | t;
            ret = ret << 6 | self.v2[ret as usize].trailing_zeros();
            ret = ret << 6 | self.v3[ret as usize].trailing_zeros();
            return Some(ret);
        }

        let mask = (!0 << ((x >> 18) & 0x3f)) << 1;
        let t = (self.v0 & mask).trailing_zeros();
        if t != 64 {
            let mut ret = t;
            ret = ret << 6 | self.v1[ret as usize].trailing_zeros();
            ret = ret << 6 | self.v2[ret as usize].trailing_zeros();
            ret = ret << 6 | self.v3[ret as usize].trailing_zeros();
            return Some(ret);
        }

        None
    }

    pub fn max_le(&self, x: u32) -> Option<u32> {
        if x >= MAX {
            return None;
        }

        let mask = !((!0 << (x & 0x3f)) << 1);
        let t = (self.v3[x as usize >> 6] & mask).leading_zeros();
        if t != 64 {
            return Some((x & !0x3f) | (63 - t));
        }

        let mask = (1 << ((x >> 6) & 0x3f)) - 1;
        let t = (self.v2[x as usize >> 12] & mask).leading_zeros();
        if t != 64 {
            let mut ret = ((x >> 6) & !0x3f) | (63 - t);
            ret = ret << 6 | (63 - self.v3[ret as usize].leading_zeros());
            return Some(ret);
        }

        let mask = (1 << ((x >> 12) & 0x3f)) - 1;
        let t = (self.v1[x as usize >> 18] & mask).leading_zeros();
        if t != 64 {
            let mut ret = ((x >> 12) & !0x3f) | (63 - t);
            ret = ret << 6 | (63 - self.v2[ret as usize].leading_zeros());
            ret = ret << 6 | (63 - self.v3[ret as usize].leading_zeros());
            return Some(ret);
        }

        let mask = (1 << ((x >> 18) & 0x3f)) - 1;
        let t = (self.v0 & mask).leading_zeros();
        if t != 64 {
            let mut ret = 63 - t;
            ret = ret << 6 | (63 - self.v1[ret as usize].leading_zeros());
            ret = ret << 6 | (63 - self.v2[ret as usize].leading_zeros());
            ret = ret << 6 | (63 - self.v3[ret as usize].leading_zeros());
            return Some(ret);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::collections::BTreeSet;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let mut set = BTreeSet::new();
        let mut qt = QwordTree::new();

        for _ in 0..50 {
            let x: u32 = rng.gen_range(0..100);
            set.insert(x);
            qt.insert(x);

            assert_eq!(set.iter().next(), qt.min().as_ref());
            assert_eq!(set.iter().rev().next(), qt.max().as_ref());

            let x: u32 = rng.gen_range(0..100);
            set.remove(&x);
            qt.erase(x);

            assert_eq!(set.iter().next(), qt.min().as_ref());
            assert_eq!(set.iter().rev().next(), qt.max().as_ref());

            let x: u32 = rng.gen_range(0..100);
            assert_eq!(set.contains(&x), qt.contains(x));
        }
    }

    #[test]
    fn test_min_ge() {
        let mut rng = rand::thread_rng();

        let mut set = BTreeSet::new();
        let mut qt = QwordTree::new();

        for _ in 0..1000 {
            let x: u32 = rng.gen_range(0..1 << 24);
            set.insert(x);
            qt.insert(x);

            let x: u32 = rng.gen_range(0..1 << 24);
            assert_eq!(set.range(x..).next(), qt.min_ge(x).as_ref());
        }
    }

    #[test]
    fn test_max_le() {
        let mut rng = rand::thread_rng();

        let mut set = BTreeSet::new();
        let mut qt = QwordTree::new();

        for _ in 0..1000 {
            let x: u32 = rng.gen_range(0..1 << 24);
            set.insert(x);
            qt.insert(x);

            let x: u32 = rng.gen_range(0..1 << 24);
            assert_eq!(set.range(..=x).rev().next(), qt.max_le(x).as_ref());
        }
    }
}
