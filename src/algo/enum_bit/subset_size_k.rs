//! ビットが`1`の個数が`k`であるものを列挙する
use std::iter::successors;

/// 幅`width`のなかで、ビットが`1`の個数が`k`であるものを列挙するイテレータを返す。
pub fn subset_size_k(width: u32, k: u32) -> impl Iterator<Item = usize> {
    successors(Some((1 << k) - 1), move |&t: &usize| {
        if t == 0 {
            None
        } else {
            let x = t & (!t).wrapping_add(1);
            let y = t + x;
            let t = (((t & !y) / x) >> 1) | y;
            (t < 1 << width).then_some(t)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: u32, k: u32) {
        let a = (0_usize..1 << n)
            .filter(|&i| i.count_ones() == k)
            .collect::<Vec<_>>();

        let b = subset_size_k(n, k).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        for n in 0..=10 {
            for k in 0..=n {
                check(n, k);
            }
        }
    }
}
