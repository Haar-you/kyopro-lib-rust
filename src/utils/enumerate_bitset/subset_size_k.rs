pub struct SubsetSizeK {
    n: u32,
    c: u32,
}

impl Iterator for SubsetSizeK {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c >= 1 << self.n {
            None
        } else {
            let ret = self.c;

            let x = ((self.c as i32) & (-(self.c as i32))) as u32;
            let y = self.c + x;
            self.c = ((self.c & !y) / x) >> 1 | y;

            Some(ret)
        }
    }
}

pub fn subset_size_k(n: u32, k: u32) -> SubsetSizeK {
    SubsetSizeK { n, c: (1 << k) - 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: u32, k: u32) {
        let a = (0..1 << n)
            .filter(|&i| (i as u32).count_ones() == k)
            .collect::<Vec<_>>();

        let b = subset_size_k(n, k).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(10, 3);
    }
}
