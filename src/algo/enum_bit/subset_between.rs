//! $a \subseteq x \subseteq b$を満たす`x`を列挙する
use std::iter::successors;

/// $a \subseteq x \subseteq b$を満たす`x`を列挙するイテレータを返す。
pub fn subset_between(a: u32, b: u32) -> impl Iterator<Item = u32> {
    let x = b ^ (a & b);

    successors((a & !b == 0).then_some(0), move |&t: &u32| {
        t.checked_sub(1).map(|a| a % x)
    })
    .map(move |t| t | a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0b11111111, 0b11111111)]
    #[test_case(0b00000000, 0b11111111)]
    #[test_case(0b10101010, 0b11111111)]
    #[test_case(0b00000001, 0b01010101)]
    #[test_case(0b00000001, 0b00000010)]
    fn check(x: u32, y: u32) {
        let a = (0..=x)
            .filter(|i| (x & !i) == 0 && (!y & i) == 0)
            .collect::<Vec<_>>();

        let b = subset_between(x, y).collect::<Vec<_>>();

        assert_eq!(a, b);
    }
}
