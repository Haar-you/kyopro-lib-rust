//! ビット列の部分集合を降順に列挙する
use std::iter::successors;

/// ビット列としての`a`の部分集合を降順に列挙するイテレータを返す。
pub fn subset_desc(a: u32) -> impl Iterator<Item = u32> {
    successors(Some(a), move |&t| t.checked_sub(1).map(|x| x & a))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0b11111111)]
    #[test_case(0b00000000)]
    #[test_case(0b10101010)]
    #[test_case(0b00000001)]
    #[test_case(0b10000000)]
    #[test_case(0b10000001)]
    #[test_case(0b11011011)]
    fn check(x: u32) {
        let a = (0..=x).rev().filter(|i| (!x & i) == 0).collect::<Vec<_>>();

        let b = subset_desc(x).collect::<Vec<_>>();

        assert_eq!(a, b);
    }
}
