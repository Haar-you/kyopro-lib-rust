//! 64分木

pub const MAX: u32 = (1 << 24) - 1;

///　0 ~ 16777215 (2²⁴ - 1) の値の集合を管理する
pub struct QwordTree {
    v0: u64,
    v1: Vec<u64>,
    v2: Vec<u64>,
    v3: Vec<u64>,
    count: usize,
}

impl Default for QwordTree {
    fn default() -> Self {
        Self::new()
    }
}

impl QwordTree {
    pub fn new() -> Self {
        Self {
            v0: 0,
            v1: vec![0; 1 << 6],
            v2: vec![0; 1 << 12],
            v3: vec![0; 1 << 18],
            count: 0,
        }
    }

    /// # Safety
    ///
    /// `x`は`MAX`以下でなければならない。
    ///
    /// `x`はQwordTreeに含まれていない。
    pub unsafe fn insert_unchecked(&mut self, x: u32) {
        self.count += 1;

        let x = x as usize;

        *self.v3.get_unchecked_mut(x >> 6) |= 1 << (x & 0x3f);
        *self.v2.get_unchecked_mut(x >> 12) |= 1 << (x >> 6 & 0x3f);
        *self.v1.get_unchecked_mut(x >> 18) |= 1 << (x >> 12 & 0x3f);
        self.v0 |= 1 << (x >> 18);
    }

    /// xを集合に加える
    pub fn insert(&mut self, x: u32) -> bool {
        if x > MAX || self.v3[x as usize >> 6] & (1 << (x & 0x3f)) != 0 {
            false
        } else {
            unsafe {
                self.insert_unchecked(x);
            }

            true
        }
    }

    /// # Safety
    ///
    /// `x`は`MAX`以下でなければならない。
    ///
    /// `x`はQwordTreeに含まれている。
    pub unsafe fn erase_unchecked(&mut self, x: u32) {
        self.count -= 1;

        let x = x as usize;

        *self.v3.get_unchecked_mut(x >> 6) &= !(1 << (x & 0x3f));
        if *self.v3.get_unchecked(x >> 6) == 0 {
            *self.v2.get_unchecked_mut(x >> 12) &= !(1 << (x >> 6 & 0x3f));
        }
        if *self.v2.get_unchecked(x >> 12) == 0 {
            *self.v1.get_unchecked_mut(x >> 18) &= !(1 << (x >> 12 & 0x3f));
        }
        if *self.v1.get_unchecked(x >> 18) == 0 {
            self.v0 &= !(1 << (x >> 18));
        }
    }

    /// xを集合から削除する
    pub fn erase(&mut self, x: u32) -> bool {
        if x > MAX || self.v3[x as usize >> 6] & (1 << (x & 0x3f)) == 0 {
            false
        } else {
            unsafe {
                self.erase_unchecked(x);
            }

            true
        }
    }

    /// xを含むかどうかを判定する
    pub fn contains(&self, x: u32) -> bool {
        if x > MAX {
            false
        } else {
            unsafe { self.v3.get_unchecked(x as usize >> 6) & (1 << (x & 0x3f)) != 0 }
        }
    }

    /// 集合が空かどうかを判断する
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// 集合に含まれている要素数を返す
    pub fn len(&self) -> usize {
        self.count
    }

    /// 最小値を返す
    pub fn min(&self) -> Option<u32> {
        if self.v0 == 0 {
            None
        } else {
            let mut ret = self.v0.trailing_zeros();
            unsafe {
                ret = ret << 6 | self.v1.get_unchecked(ret as usize).trailing_zeros();
                ret = ret << 6 | self.v2.get_unchecked(ret as usize).trailing_zeros();
                ret = ret << 6 | self.v3.get_unchecked(ret as usize).trailing_zeros();
            }
            Some(ret)
        }
    }

    /// 最大値を返す
    pub fn max(&self) -> Option<u32> {
        if self.v0 == 0 {
            None
        } else {
            let mut ret = 63 - self.v0.leading_zeros();
            unsafe {
                ret = ret << 6 | (63 - self.v1.get_unchecked(ret as usize).leading_zeros());
                ret = ret << 6 | (63 - self.v2.get_unchecked(ret as usize).leading_zeros());
                ret = ret << 6 | (63 - self.v3.get_unchecked(ret as usize).leading_zeros());
            }
            Some(ret)
        }
    }

    /// x以上で最小の値を返す
    pub fn min_ge(&self, mut x: u32) -> Option<u32> {
        if x > MAX {
            return None;
        }

        let mask = !((1 << (x & 0x3f)) - 1);
        let t = unsafe { (self.v3.get_unchecked(x as usize >> 6) & mask).trailing_zeros() };
        if t != 64 {
            return Some((x & !0x3f) | t);
        }

        x >>= 6;
        let mask = (!0 << (x & 0x3f)) << 1;
        let t = unsafe { (self.v2.get_unchecked(x as usize >> 6) & mask).trailing_zeros() };
        if t != 64 {
            let mut ret = (x & !0x3f) | t;
            unsafe {
                ret = ret << 6 | self.v3.get_unchecked(ret as usize).trailing_zeros();
            }
            return Some(ret);
        }

        x >>= 6;
        let mask = (!0 << (x & 0x3f)) << 1;
        let t = unsafe { (self.v1.get_unchecked(x as usize >> 6) & mask).trailing_zeros() };
        if t != 64 {
            let mut ret = (x & !0x3f) | t;
            unsafe {
                ret = ret << 6 | self.v2.get_unchecked(ret as usize).trailing_zeros();
                ret = ret << 6 | self.v3.get_unchecked(ret as usize).trailing_zeros();
            }
            return Some(ret);
        }

        x >>= 6;
        let mask = (!0 << (x & 0x3f)) << 1;
        let t = (self.v0 & mask).trailing_zeros();
        if t != 64 {
            let mut ret = t;
            unsafe {
                ret = ret << 6 | self.v1.get_unchecked(ret as usize).trailing_zeros();
                ret = ret << 6 | self.v2.get_unchecked(ret as usize).trailing_zeros();
                ret = ret << 6 | self.v3.get_unchecked(ret as usize).trailing_zeros();
            }
            return Some(ret);
        }

        None
    }

    /// x以下で最大の値を返す
    pub fn max_le(&self, mut x: u32) -> Option<u32> {
        if x > MAX {
            return None;
        }

        let mask = !((!0 << (x & 0x3f)) << 1);
        let t = unsafe { (self.v3.get_unchecked(x as usize >> 6) & mask).leading_zeros() };
        if t != 64 {
            return Some((x & !0x3f) | (63 - t));
        }

        x >>= 6;
        let mask = (1 << (x & 0x3f)) - 1;
        let t = unsafe { (self.v2.get_unchecked(x as usize >> 6) & mask).leading_zeros() };
        if t != 64 {
            let mut ret = (x & !0x3f) | (63 - t);
            unsafe {
                ret = ret << 6 | (63 - self.v3.get_unchecked(ret as usize).leading_zeros());
            }
            return Some(ret);
        }

        x >>= 6;
        let mask = (1 << (x & 0x3f)) - 1;
        let t = unsafe { (self.v1.get_unchecked(x as usize >> 6) & mask).leading_zeros() };
        if t != 64 {
            let mut ret = (x & !0x3f) | (63 - t);
            unsafe {
                ret = ret << 6 | (63 - self.v2.get_unchecked(ret as usize).leading_zeros());
                ret = ret << 6 | (63 - self.v3.get_unchecked(ret as usize).leading_zeros());
            }
            return Some(ret);
        }

        x >>= 6;
        let mask = (1 << (x & 0x3f)) - 1;
        let t = (self.v0 & mask).leading_zeros();
        if t != 64 {
            let mut ret = 63 - t;
            unsafe {
                ret = ret << 6 | (63 - self.v1.get_unchecked(ret as usize).leading_zeros());
                ret = ret << 6 | (63 - self.v2.get_unchecked(ret as usize).leading_zeros());
                ret = ret << 6 | (63 - self.v3.get_unchecked(ret as usize).leading_zeros());
            }
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

        for _ in 0..5000 {
            let x: u32 = rng.gen_range(0..1 << 12);
            assert_eq!(set.insert(x), qt.insert(x));
            assert_eq!(set.len(), qt.len());

            assert_eq!(set.iter().next(), qt.min().as_ref());
            assert_eq!(set.iter().rev().next(), qt.max().as_ref());

            let x: u32 = rng.gen_range(0..1 << 12);
            assert_eq!(set.remove(&x), qt.erase(x));
            assert_eq!(set.len(), qt.len());

            assert_eq!(set.iter().next(), qt.min().as_ref());
            assert_eq!(set.iter().rev().next(), qt.max().as_ref());

            let x: u32 = rng.gen_range(0..1 << 12);
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
