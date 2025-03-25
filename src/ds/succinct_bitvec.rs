//! 完結ビットベクトル
use std::ops::Range;

const CHUNK_SIZE: usize = 256;
const BLOCK_SIZE: usize = 64;
const BLOCK_NUM: usize = CHUNK_SIZE / BLOCK_SIZE;

/// 完結ビットベクトル
#[derive(Clone)]
pub struct SuccinctBitVec {
    size: usize,
    data: Vec<u64>,
    blocks: Vec<[u8; BLOCK_NUM]>,
    chunks: Vec<u32>,
}

impl SuccinctBitVec {
    /// `Vec<bool>`から[`SuccinctBitVec`]を構築する。
    pub fn new(b: Vec<bool>) -> Self {
        let size = b.len();
        let chunk_num = (size + CHUNK_SIZE - 1) / CHUNK_SIZE;
        let mut data: Vec<u64> = vec![0; chunk_num * BLOCK_NUM + 1];

        for (i, x) in b.into_iter().enumerate() {
            if x {
                let block_index = i / BLOCK_SIZE;
                let index = i % BLOCK_SIZE;
                data[block_index] |= 1 << index;
            }
        }

        let mut chunks: Vec<u32> = vec![0; chunk_num + 1];
        let mut blocks: Vec<[u8; BLOCK_NUM]> = vec![[0; BLOCK_NUM]; chunk_num + 1];

        for (i, block_i) in blocks.iter_mut().take(chunk_num).enumerate() {
            for j in 0..BLOCK_NUM - 1 {
                block_i[j + 1] = block_i[j] + data[i * BLOCK_NUM + j].count_ones() as u8;
            }

            chunks[i + 1] = chunks[i]
                + block_i[BLOCK_NUM - 1] as u32
                + data[(i + 1) * BLOCK_NUM - 1].count_ones();
        }

        Self {
            size,
            data,
            blocks,
            chunks,
        }
    }

    /// ビットベクトルの長さを返す。
    pub fn len(&self) -> usize {
        self.size
    }

    /// [0, index) に含まれる`b`の個数
    pub fn rank(&self, index: usize, b: bool) -> usize {
        assert!(index <= self.size);

        if b {
            let chunk_pos = index / CHUNK_SIZE;
            let block_pos = (index % CHUNK_SIZE) / BLOCK_SIZE;

            let mask =
                self.data[chunk_pos * BLOCK_NUM + block_pos] & ((1 << (index % BLOCK_SIZE)) - 1);

            self.chunks[chunk_pos] as usize
                + self.blocks[chunk_pos][block_pos] as usize
                + mask.count_ones() as usize
        } else {
            index - self.rank(index, !b)
        }
    }

    /// [l, r) に含まれる`b`の個数
    pub fn count(&self, Range { start: l, end: r }: Range<usize>, b: bool) -> usize {
        assert!(l <= r);
        self.rank(r, b) - self.rank(l, b)
    }

    /// `index`番目のビットを返す。
    pub fn access(&self, index: usize) -> u64 {
        (self.data[index / BLOCK_SIZE] >> (index % BLOCK_SIZE)) & 1
    }

    /// nth(0-indexed)番目の`b`の位置
    pub fn select(&self, nth: usize, b: bool) -> Option<usize> {
        let nth = nth + 1;

        if self.rank(self.size, b) < nth {
            None
        } else {
            let mut lb: isize = -1;
            let mut ub: isize = self.size as isize;
            while (ub - lb).abs() > 1 {
                let mid = (lb + ub) / 2;

                if self.rank(mid as usize, b) >= nth {
                    ub = mid;
                } else {
                    lb = mid;
                }
            }

            Some(lb as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_rank() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let b = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();

        let s = SuccinctBitVec::new(b.clone());

        for i in 0..=n {
            let t = (0..i).filter(|&i| b[i]).count();
            assert_eq!(s.rank(i, true), t);

            let t = (0..i).filter(|&i| !b[i]).count();
            assert_eq!(s.rank(i, false), t);
        }
    }

    #[test]
    fn test_count() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let b = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();

        let s = SuccinctBitVec::new(b.clone());

        for l in 0..=n {
            for r in l..=n {
                let t = (l..r).filter(|&i| b[i]).count();
                assert_eq!(s.count(l..r, true), t);

                let t = (l..r).filter(|&i| !b[i]).count();
                assert_eq!(s.count(l..r, false), t);
            }
        }
    }

    #[test]
    fn test_select() {
        let mut rng = rand::thread_rng();
        let n = 30;
        let b = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();

        let s = SuccinctBitVec::new(b.clone());

        for i in 1..=n {
            let t = (0..n).filter(|&i| b[i]).nth(i);
            assert_eq!(s.select(i, true), t);

            let t = (0..n).filter(|&i| !b[i]).nth(i);
            assert_eq!(s.select(i, false), t);
        }
    }
}
