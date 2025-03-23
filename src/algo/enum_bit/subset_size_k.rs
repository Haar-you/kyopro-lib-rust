//! ビットが`1`の個数が`k`であるものを列挙する
use std::iter::successors;

/// 幅`width`のなかで、ビットが`1`の個数が`k`であるものを列挙するイテレータを返す。
pub fn subset_size_k(width: u32, k: u32) -> impl Iterator<Item = u32> {
    successors(Some((1 << k) - 1), move |&t| {
        let x = ((t as i32) & (-(t as i32))) as u32;
        let y = t + x;
        let t = ((t & !y) / x) >> 1 | y;
        (t < 1 << width).then_some(t)
    })
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
