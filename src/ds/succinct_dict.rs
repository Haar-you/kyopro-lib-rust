use std::ops::Range;

const CHUNK_SIZE: usize = 256;
const BLOCK_SIZE: usize = 64;
const BLOCK_NUM: usize = CHUNK_SIZE / BLOCK_SIZE;

#[derive(Clone)]
pub struct SuccinctDict {
    size: usize,
    data: Vec<u64>,
    blocks: Vec<Vec<u8>>,
    chunks: Vec<u32>,
}

impl SuccinctDict {
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
        let mut blocks: Vec<Vec<u8>> = vec![vec![0; BLOCK_NUM]; chunk_num + 1];

        for i in 0..chunk_num {
            for j in 0..BLOCK_NUM - 1 {
                blocks[i][j + 1] = blocks[i][j] + data[i * BLOCK_NUM + j].count_ones() as u8;
            }

            chunks[i + 1] = chunks[i]
                + blocks[i][BLOCK_NUM - 1] as u32
                + data[(i + 1) * BLOCK_NUM - 1].count_ones();
        }

        Self {
            size,
            data,
            blocks,
            chunks,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// [0, index) に含まれる`b`の個数
    pub fn rank(&self, index: usize, b: bool) -> u32 {
        if b {
            let chunk_pos = index / CHUNK_SIZE;
            let block_pos = (index % CHUNK_SIZE) / BLOCK_SIZE;

            let mask =
                self.data[chunk_pos * BLOCK_NUM + block_pos] & ((1 << (index % BLOCK_SIZE)) - 1);

            self.chunks[chunk_pos] + self.blocks[chunk_pos][block_pos] as u32 + mask.count_ones()
        } else {
            index as u32 - self.rank(index, !b)
        }
    }

    /// [l, r) に含まれる`b`の個数
    pub fn count(&self, Range { start: l, end: r }: Range<usize>, b: bool) -> u32 {
        assert!(l <= r);
        self.rank(r, b) - self.rank(l, b)
    }

    pub fn access(&self, index: usize) -> u32 {
        ((self.data[index / BLOCK_SIZE] >> (index % BLOCK_SIZE)) & 1) as u32
    }

    /// nth(1-indexed)番目の`b`の位置
    pub fn select(&self, nth: usize, b: bool) -> Option<usize> {
        assert!(nth >= 1);

        if self.rank(self.size, b) < nth as u32 {
            None
        } else {
            let mut lb: isize = -1;
            let mut ub: isize = self.size as isize;
            while (ub - lb).abs() > 1 {
                let mid = (lb + ub) / 2;

                if self.rank(mid as usize, b) >= nth as u32 {
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

        let s = SuccinctDict::new(b.clone());

        for i in 0..=n {
            let t = (0..i).filter(|&i| b[i]).count();
            assert_eq!(s.rank(i, true), t as u32);

            let t = (0..i).filter(|&i| !b[i]).count();
            assert_eq!(s.rank(i, false), t as u32);
        }
    }

    #[test]
    fn test_count() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let b = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();

        let s = SuccinctDict::new(b.clone());

        for l in 0..=n {
            for r in l..=n {
                let t = (l..r).filter(|&i| b[i]).count();
                assert_eq!(s.count(l..r, true), t as u32);

                let t = (l..r).filter(|&i| !b[i]).count();
                assert_eq!(s.count(l..r, false), t as u32);
            }
        }
    }

    #[test]
    fn test_select() {
        let mut rng = rand::thread_rng();
        let n = 30;
        let b = (0..n).map(|_| rng.gen::<bool>()).collect::<Vec<_>>();

        let s = SuccinctDict::new(b.clone());

        for i in 1..=n {
            let t = (0..n).filter(|&i| b[i]).nth(i - 1);
            assert_eq!(s.select(i, true), t);

            let t = (0..n).filter(|&i| !b[i]).nth(i - 1);
            assert_eq!(s.select(i, false), t);
        }
    }
}
