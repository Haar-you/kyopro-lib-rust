//! Suffix Array
use std::collections::VecDeque;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum LS {
    L,
    S,
}

fn sa(mut s: Vec<u32>) -> Vec<usize> {
    use self::LS::{L, S};

    let n = s.len();

    if n == 0 {
        return vec![0];
    }
    if n == 1 {
        return vec![1, 0];
    }

    s.push(0);

    let mut ls = vec![S; n + 1];
    for i in (0..n).rev() {
        use std::cmp::Ordering::*;
        match s[i].cmp(&s[i + 1]) {
            Less => ls[i] = S,
            Greater => ls[i] = L,
            Equal => ls[i] = ls[i + 1],
        }
    }

    let bucket_count = *s.iter().max().unwrap() as usize;
    let mut bucket_size = vec![0; bucket_count + 1];
    for &x in &s {
        bucket_size[x as usize] += 1;
    }

    let induced_sort = |lms: &[usize]| -> Vec<usize> {
        let mut bucket = vec![None; n + 1];
        let mut is_lms = vec![false; n + 1];
        let mut empty = vec![VecDeque::new(); bucket_count + 1];

        let mut k = 0;
        for i in 0..=bucket_count {
            for _ in 0..bucket_size[i] {
                empty[i].push_back(k);
                k += 1;
            }
        }

        for &x in lms.iter().rev() {
            let i = empty[s[x] as usize].pop_back().unwrap();
            bucket[i] = Some(x);
            is_lms[i] = true;
        }

        for i in 0..=n {
            match bucket[i] {
                Some(b) if b >= 1 && ls[b - 1] == L => {
                    let j = empty[s[b - 1] as usize].pop_front().unwrap();
                    bucket[j] = Some(b - 1);
                }
                _ => {}
            }
        }

        for i in 0..=n {
            if is_lms[i] {
                bucket[i] = None;
            }
        }

        let mut k = 0;
        for i in 0..=bucket_count {
            empty[i].clear();

            for _ in 0..bucket_size[i] {
                empty[i].push_back(k);
                k += 1;
            }
        }

        for i in (0..=n).rev() {
            match bucket[i] {
                Some(b) if b >= 1 && ls[b - 1] == S => {
                    let j = empty[s[b - 1] as usize].pop_back().unwrap();
                    bucket[j] = Some(b - 1);
                }
                _ => {}
            }
        }

        bucket[0] = Some(n);
        bucket.into_iter().map(|x| x.unwrap()).collect()
    };

    let lms: Vec<_> = (1..=n).filter(|&i| ls[i] == S && ls[i - 1] == L).collect();

    let mut lms_bucket_length = vec![1; n + 1];
    for i in 0..lms.len() - 1 {
        lms_bucket_length[lms[i]] = lms[i + 1] - lms[i] + 1;
    }

    let lms_substr_sorted: Vec<_> = induced_sort(&lms)
        .into_iter()
        .filter(|&i| i > 0 && ls[i - 1] == L && ls[i] == S)
        .collect();

    let mut rank = vec![0; n + 1];
    rank[lms_substr_sorted[0]] = 1;

    let mut k = 1;
    for i in 1..lms_substr_sorted.len() {
        let x = lms_substr_sorted[i - 1];
        let y = lms_substr_sorted[i];
        let eq = (|| {
            if lms_bucket_length[x] != lms_bucket_length[y] {
                false
            } else {
                for j in 0..lms_bucket_length[x] {
                    if s[x + j] != s[y + j] {
                        return false;
                    }
                }
                true
            }
        })();

        if !eq {
            k += 1;
        }
        rank[y] = k;
    }

    let t: Vec<_> = (0..=n).map(|i| rank[i]).filter(|&x| x != 0).collect();
    let lms_sorted: Vec<_> = sa(t).into_iter().skip(1).map(|i| lms[i]).collect();

    induced_sort(&lms_sorted)
}

/// 接尾辞配列
#[derive(Clone, Debug)]
pub struct SuffixArray {
    data: Vec<usize>,
    str_data: Vec<u8>,
}

impl SuffixArray {
    /// 文字列`s`から接尾辞配列を構築する。
    pub fn new(s: &str) -> Self {
        let mut str_data = s.as_bytes().to_vec();
        str_data.push(0);

        let s_ = s.as_bytes().iter().map(|&x| x as u32).collect();
        Self {
            data: sa(s_),
            str_data,
        }
    }

    /// 接尾辞配列への参照を返す。
    pub fn to_slice(&self) -> &[usize] {
        &self.data
    }

    pub fn lcp_array(&self) -> Vec<usize> {
        let n = self.data.len();
        let mut rank = vec![0; n];
        let mut ret = vec![0; n];

        for i in 0..n {
            rank[self.data[i]] = i;
        }

        let mut h: usize = 0;
        for i in 0..n {
            if rank[i] == 0 {
                continue;
            }

            h = h.saturating_sub(1);
            let j = self.data[rank[i] - 1];
            while j + h < n && i + h < n {
                if self.str_data[j + h] != self.str_data[i + h] {
                    break;
                }
                h += 1;
            }

            ret[rank[i]] = h;
        }

        ret
    }
}

impl std::ops::Index<usize> for SuffixArray {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let sa = SuffixArray::new("abracadabra");
        assert_eq!(sa.to_slice(), &[11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
    }
}
