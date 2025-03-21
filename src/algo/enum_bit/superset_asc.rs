//! ビット列の上位集合を昇順に列挙する
use std::iter::successors;

/// ビット列としての`a`の上位集合を昇順に列挙するイテレータを返す。
pub fn super_asc(a: u32, n: u32) -> impl Iterator<Item = u32> {
    successors(Some(a), move |&t| {
        let t = (t + 1) | a;
        (t < 1 << n).then_some(t)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0b11111111, 8)]
    #[test_case(0b00000000, 8)]
    #[test_case(0b10101010, 8)]
    #[test_case(0b00000001, 8)]
    #[test_case(0b10000000, 8)]
    #[test_case(0b10000001, 8)]
    #[test_case(0b11011011, 8)]
    fn check(x: u32, n: u32) {
        let a = (0..1 << n).filter(|i| (x & !i) == 0).collect::<Vec<_>>();

        let b = super_asc(x, n).collect::<Vec<_>>();

        assert_eq!(a, b);
    }
}
